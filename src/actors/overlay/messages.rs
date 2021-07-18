use crate::actors::ws::WsSessionActor;
use actix::{Addr, Message};
use anyhow::Result as AnyResult;
use serde::Serialize;

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Connect {
    pub addr: Addr<WsSessionActor>,
    pub overlay_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect(pub usize);

#[derive(Message)]
#[rtype(result = "()")]
pub struct DeleteOverlay(pub i32);

pub struct OverlayCommand {
    pub channel_login: String,
    pub data: OverlayCommandData,
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum OverlayCommandData {
    Image(String),
}

impl Message for OverlayCommand {
    type Result = ();
}

pub struct Outgoing(pub OverlayCommandData);

impl Message for Outgoing {
    type Result = AnyResult<()>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseSock;
