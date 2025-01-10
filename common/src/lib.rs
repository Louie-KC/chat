use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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
pub struct ChatRoom {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRoomName {
    pub room_name: String,
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