use chrono::{DateTime, Utc};
use common::LoginTokenInfo;
use serde;

#[derive(Debug, serde::Deserialize)]
pub struct DBUser {
    pub(crate) id: u64,
    pub(crate) username: String,
    pub(crate) password_hash: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DBRoomMember {
    pub(crate) user_id: u64,
    pub(crate) username: String
}

#[derive(sqlx::Type, serde::Deserialize)]
#[sqlx(transparent)]
pub struct MySqlBool (pub bool);

pub struct DBAuthInfo {
    pub user_agent: String,
    pub time_set: DateTime<Utc>,
    pub is_requester: MySqlBool
}

impl Into<LoginTokenInfo> for &DBAuthInfo {
    fn into(self) -> LoginTokenInfo {
        LoginTokenInfo {
            user_agent: self.user_agent.clone(),
            time_set: self.time_set,
            is_requester: self.is_requester.0
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct UserSearchParam {
    pub username: String
}