use common::{
    AccountPasswordChange,
    AccountRequest,
    ChatRoom,
    ChatRoomManageUser,
    ChatRoomName,
    LoginResponse, UserList
};

use reqwest::{self, StatusCode};

const BASE_URI: &str = "http://127.0.0.1:8000";

// Account management
pub async fn account_register(details: AccountRequest) -> Result<(), ()> {
    let endpoint = format!("{}/account/register", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .json(&details)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        _ => Err(())
    }
}

pub async fn account_login(details: &AccountRequest) -> Result<LoginResponse, ()> {
    let endpoint = format!("{}/account/login", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .json(&details)
        .send()
        .await;

    let response = match response {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    match response.json::<LoginResponse>().await {
        Ok(res) => Ok(res),
        Err(_) => Err(()),
    }
}

pub async fn account_change_password(token: &str, details: AccountPasswordChange) -> Result<(), ()> {
    let endpoint = format!("{}/account/change-password", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token)
        .json(&details)
        .send()
        .await;
    
    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        _ => Err(())
    }
}

pub async fn account_logout(token: &str) -> Result<(), ()> {
    let endpoint = format!("{}/account/logout", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        _ => Err(())
    }
}

pub async fn account_clear_tokens(token: &str) -> Result<(), ()> {
    let endpoint= format!("{}/account/clear-tokens", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        _ => Err(())
    }
}

// Room management

pub async fn chat_get_rooms(token: &str) -> Result<Vec<ChatRoom>, ()> {
    let endpoint= format!("{}/chat/rooms", BASE_URI);

    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token)
        .send()
        .await;

    let response = match response {
        Ok(res) => res,
        Err(_) => return Err(()),
    };

    match response.json::<Vec<ChatRoom>>().await {
        Ok(rooms) => Ok(rooms),
        Err(_) => Err(()),
    }
}

pub async fn chat_create_room(token: &str, room_name: &str) -> Result<(), ()> {
    let endpoint= format!("{}/chat/create-room", BASE_URI);

    let body = ChatRoomName { room_name: room_name.to_string() };

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        _ => Err(())
    }
}

pub async fn chat_change_name(token: &str, room_id: u64, new_name: &str) -> Result<(), ()> {
    let endpoint = format!("{}/chat/{}/change-name", BASE_URI, room_id);

    let body = ChatRoomName { room_name: new_name.to_string() };

    let response = reqwest::Client::new()
        .put(endpoint)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await;
    
    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        _ => Err(())
    }
}

pub async fn chat_get_members(token: &str, room_id: u64) -> Result<Vec<UserList>, ()> {
    let endpoint = format!("{}/chat/{}/members", BASE_URI, room_id);

    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token)
        .send()
        .await;

    let response = match response {
        Ok(res) => res,
        Err(_) => return Err(())
    };

    match response.json::<Vec<UserList>>().await {
        Ok(members) => Ok(members),
        Err(_) => Err(()),
    }
}

pub async fn chat_manage_user(token: &str, room_id: u64, action: ChatRoomManageUser) -> Result<(), ()> {
    let endpoint = format!("{}/chat/{}/manage-user", BASE_URI, room_id);

    let response = reqwest::Client::new()
        .put(endpoint)
        .bearer_auth(token)
        .json(&action)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        _ => Err(())
    }
}
