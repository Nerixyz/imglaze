use super::overlay::messages::OverlayCommand;
use crate::{
    actors::{
        db::DbActor,
        irc::messages::{JoinAllMessage, JoinMessage, PartMessage},
        overlay::messages::OverlayCommandData,
    },
    constants::{TWITCH_CLIENT_ID, TWITCH_CLIENT_SECRET, TWITCH_CLIENT_USER_LOGIN},
    log_err,
    services::{
        chat::opt_next_space,
        images::{check_image_url, wrap_proxy},
    },
};
use actix::{
    Actor, Addr, AsyncContext, Context, ContextFutureSpawner, Handler, Recipient, StreamHandler,
    WrapFuture,
};
use futures::StreamExt;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use twitch_irc::{
    login::RefreshingLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

pub mod messages;

mod token_storage;

type IrcCredentials = RefreshingLoginCredentials<token_storage::PgTokenStorage>;
type IrcClient = TwitchIRCClient<SecureTCPTransport, IrcCredentials>;

pub struct IrcActor {
    client: IrcClient,
    overlay: Recipient<OverlayCommand>,

    cooldowns: HashMap<String, Instant>,
}

impl IrcActor {
    pub fn run(overlay: Recipient<OverlayCommand>, db: Addr<DbActor>) -> Addr<Self> {
        let config = ClientConfig {
            metrics_identifier: Some("imglaze".into()),
            ..ClientConfig::new_simple(IrcCredentials::new(
                TWITCH_CLIENT_USER_LOGIN.to_string(),
                TWITCH_CLIENT_ID.to_string(),
                TWITCH_CLIENT_SECRET.to_string(),
                token_storage::PgTokenStorage(db),
            ))
        };
        let (incoming, client) = IrcClient::new(config);

        Self::create(|ctx| {
            let stream = UnboundedReceiverStream::new(incoming).filter_map(|s| async move {
                match s {
                    ServerMessage::Privmsg(pmsg) => Some(pmsg),
                    _ => None,
                }
            });
            ctx.add_stream(stream);

            Self {
                client,
                overlay,
                cooldowns: HashMap::new(),
            }
        })
    }

    /// Returns true if there's _no_ cooldown for the channel.
    fn check_update_cooldown(&mut self, login: &str) -> bool {
        let now = Instant::now();
        if let Some(v) = self.cooldowns.get_mut(login) {
            if now.duration_since(*v) < Duration::from_secs(4) {
                false
            } else {
                *v = now;
                true
            }
        } else {
            self.cooldowns.insert(login.to_string(), now);
            true
        }
    }
}

impl Actor for IrcActor {
    type Context = Context<Self>;
}

impl StreamHandler<PrivmsgMessage> for IrcActor {
    fn handle(&mut self, msg: PrivmsgMessage, ctx: &mut Self::Context) {
        if !msg.message_text.starts_with("::")
            || msg.message_text.len() < 4
            || !msg
                .badges
                .iter()
                .any(|b| b.name == "broadcaster" || b.name == "moderator")
            || !self.check_update_cooldown(&msg.channel_login)
        {
            return;
        }

        let (command, args) = opt_next_space(&msg.message_text[2..]);
        match (command, args) {
            ("img", Some(args)) => {
                let (image, _) = opt_next_space(args);
                let client = self.client.clone();
                let overlay = self.overlay.clone();
                let image = image.to_string();
                async move {
                    if let Err(err) = check_image_url(&image) {
                        log_err!(
                            client
                                .say(msg.channel_login, format!("Bad image: {}", err))
                                .await,
                            "Failed to say"
                        );
                        return;
                    }

                    let image = wrap_proxy(&image);
                    if image.len() >= 255 {
                        log_err!(
                            client
                                .say(msg.channel_login, "Link is too long".to_string())
                                .await,
                            "Failed to say"
                        );
                        return;
                    }

                    metrics::increment_counter!("imglaze_images_changed");
                    let message = match overlay
                        .send(OverlayCommand {
                            channel_login: msg.channel_login.clone(),
                            data: OverlayCommandData::Image(image),
                        })
                        .await
                    {
                        Ok(_) => "Ok",
                        Err(_) => "Too much traffic",
                    }
                    .to_string();
                    log_err!(
                        client.say(msg.channel_login, message).await,
                        "Failed to say"
                    );
                }
                .into_actor(self)
                .spawn(ctx)
            }
            _ => (),
        };
    }
}

impl Handler<JoinMessage> for IrcActor {
    type Result = ();

    fn handle(&mut self, msg: JoinMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.client.join(msg.0);
    }
}

impl Handler<JoinAllMessage> for IrcActor {
    type Result = ();

    fn handle(&mut self, msg: JoinAllMessage, _ctx: &mut Self::Context) -> Self::Result {
        for channel in msg.0 {
            self.client.join(channel);
        }
    }
}

impl Handler<PartMessage> for IrcActor {
    type Result = ();

    fn handle(&mut self, msg: PartMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.cooldowns.remove(&msg.0);
        self.client.part(msg.0);
    }
}
