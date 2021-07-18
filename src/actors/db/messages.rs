use crate::errors::sql::SqlResult;
use actix::Message;
use twitch_irc::login::UserAccessToken;

pub struct SaveToken(pub UserAccessToken);

impl Message for SaveToken {
    type Result = SqlResult<()>;
}

pub struct GetToken;

impl Message for GetToken {
    type Result = SqlResult<UserAccessToken>;
}
