use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountPasswordChange {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub user_id: u64,
    pub token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoom {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoomList {
    pub rooms: Vec<ChatRoom>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoomName {
    pub room_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserList {
    pub members: Vec<UserInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatRoomManageUserAction {
    AddUser,
    RemoveUser
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoomManageUser {
    pub username: String,
    pub action: ChatRoomManageUserAction
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub id: Option<u64>,  // is 2^64 enough? also in schema
    pub room_id: u64,
    pub sender_id: Option<u64>,
    pub body: String,
    pub time_sent: Option<DateTime<Utc>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessageList {
    pub messages: Vec<ChatMessage>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: u64,
    pub username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserAssociationType {
    Friend,
    Block,
    Remove
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAssociationUpdate {
    pub other_user_id: u64,
    pub association_type: UserAssociationType
}