pub mod messages;

use super::ws::WsSessionActor;
use crate::{
    actors::overlay::messages::{CloseSock, DeleteOverlay, Disconnect, Outgoing, OverlayCommand},
    models::overlay,
};
use actix::{Actor, ActorFutureExt, Addr, Context, ContextFutureSpawner, Handler, WrapFuture};
use messages::Connect;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};

type Session = Addr<WsSessionActor>;
type Sessions = HashMap<usize, Session>;
type Overlays = HashMap<i32, HashSet<usize>>;

pub struct OverlayActor {
    sessions: Sessions,
    overlays: Overlays,
    next_session_id: usize,

    pool: PgPool,
}

impl OverlayActor {
    pub fn new(pool: PgPool) -> Self {
        Self {
            sessions: Default::default(),
            overlays: Default::default(),
            next_session_id: 0,

            pool,
        }
    }
}

impl Actor for OverlayActor {
    type Context = Context<Self>;
}

impl Handler<Connect> for OverlayActor {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let session_id = self.next_session_id;
        self.next_session_id += 1;

        self.overlays
            .entry(msg.overlay_id)
            .or_insert_with(HashSet::new)
            .insert(session_id);
        self.sessions.insert(session_id, msg.addr);

        session_id
    }
}

impl Handler<Disconnect> for OverlayActor {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        for (_, overlay) in &mut self.overlays {
            overlay.remove(&msg.0);
        }
        self.sessions.remove(&msg.0);
    }
}

impl Handler<DeleteOverlay> for OverlayActor {
    type Result = ();

    fn handle(&mut self, msg: DeleteOverlay, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(sessions) = self.overlays.remove(&msg.0) {
            for session_id in sessions {
                if let Some(session) = self.sessions.remove(&session_id) {
                    session.do_send(CloseSock);
                }
            }
        }
    }
}

impl Handler<OverlayCommand> for OverlayActor {
    type Result = ();

    fn handle(&mut self, msg: OverlayCommand, ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        let (channel_login, data) = (msg.channel_login, msg.data);
        async move { overlay::by_login(&channel_login, &pool).await }
            .into_actor(self)
            .then(move |res, this, ctx| {
                if let Ok(overlay) = res {
                    if let Some(overlay) = this.overlays.get(&overlay.id) {
                        for session in overlay {
                            if let Some(session) = this.sessions.get(session) {
                                session
                                    .send(Outgoing(data.clone()))
                                    .into_actor(this)
                                    .then(|_, _, _| actix::fut::ready(()))
                                    .spawn(ctx);
                            }
                        }
                    }
                }
                actix::fut::ready(())
            })
            .spawn(ctx);
    }
}
