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

#[derive(Debug, serde::Deserialize)]
pub struct UserSearchParam {
    pub username: String
}