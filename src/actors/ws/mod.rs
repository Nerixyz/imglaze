use crate::actors::overlay::{
    messages::{CloseSock, Connect, Disconnect, Outgoing},
    OverlayActor,
};
use actix::{
    Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::{
    ws,
    ws::{Message, ProtocolError},
};
use anyhow::Result as AnyResult;
use serde::Deserialize;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60 + 10);

pub struct WsSessionActor {
    client_id: usize,

    overlay_id: i32,
    overlay: Addr<OverlayActor>,

    last_hb: Instant,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum WsResponse {
    Pong,
    Ping,
}

impl WsSessionActor {
    pub fn new(overlay_id: i32, overlay: Addr<OverlayActor>) -> Self {
        Self {
            client_id: 0,
            overlay_id,
            last_hb: Instant::now(),
            overlay,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_hb) > CLIENT_TIMEOUT {
                act.overlay.do_send(Disconnect(act.client_id));
                ctx.stop();
                return;
            }

            ctx.text(
                serde_json::json!({
                    "type": "Ping"
                })
                .to_string(),
            );
        });
    }
}

impl Actor for WsSessionActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        metrics::increment_gauge!("imglaze_active_sockets", 1.0);
        self.hb(ctx);

        let this_addr = ctx.address();
        self.overlay
            .send(Connect {
                addr: this_addr,
                overlay_id: self.overlay_id,
            })
            .into_actor(self)
            .then(|res, this, ctx| {
                match res {
                    Ok(res) => this.client_id = res,
                    _ => ctx.stop(),
                };
                actix::fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        metrics::decrement_gauge!("imglaze_active_sockets", 1.0);
        self.overlay.do_send(Disconnect(self.client_id));
        log::info!("Stopping ws session for id {}", self.overlay_id);
        Running::Stop
    }
}

impl Handler<Outgoing> for WsSessionActor {
    type Result = AnyResult<()>;

    fn handle(&mut self, msg: Outgoing, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg.0)?);
        Ok(())
    }
}

impl Handler<CloseSock> for WsSessionActor {
    type Result = ();

    fn handle(&mut self, _: CloseSock, ctx: &mut Self::Context) -> Self::Result {
        ctx.close(None);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSessionActor {
    fn handle(&mut self, item: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        match msg {
            Message::Text(txt) => {
                if let Ok(msg) = serde_json::from_str::<WsResponse>(&txt) {
                    match msg {
                        WsResponse::Pong => self.last_hb = Instant::now(),
                        WsResponse::Ping => {
                            self.last_hb = Instant::now();
                            ctx.text(
                                serde_json::json!({
                                    "type": "Pong"
                                })
                                .to_string(),
                            );
                        }
                    }
                }
            }
            Message::Ping(msg) => {
                ctx.pong(&msg);
            }
            Message::Continuation(_) => {
                ctx.stop();
            }
            Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        };
    }
}
