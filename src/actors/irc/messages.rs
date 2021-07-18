use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinAllMessage(pub Vec<String>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct PartMessage(pub String);
