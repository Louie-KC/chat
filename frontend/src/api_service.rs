use common::{
    AccountPasswordChange, AccountRequest, ChatRoom, ChatRoomManageUser, ChatRoomName, LoginResponse, LoginTokenInfo, UserList
};

use gloo::console::log;

use reqwest::{self, Response, StatusCode};
use uuid::Uuid;

const BASE_URI: &str = "http://127.0.0.1:8000";

type ApiResult<T> = Result<T, String>;

// Account management
pub async fn account_register(details: AccountRequest) -> ApiResult<()> {
    let endpoint = format!("{}/account/register", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .json(&details)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(err_status_code_to_msg(&res).await),
        _ => Err("Undefined error".into())
    }
}

pub async fn account_login(details: &AccountRequest) -> ApiResult<LoginResponse> {
    let endpoint = format!("{}/account/login", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .json(&details)
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status() == StatusCode::OK => res,
        Ok(res) => return Err(err_status_code_to_msg(&res).await),
        Err(_) => return Err("Undefined error".into())
    };

    match response.json::<LoginResponse>().await {
        Ok(res) => Ok(res),
        Err(_) => Err("Undefined error".into())
    }
}

pub async fn account_change_password(token: &Uuid, details: AccountPasswordChange) -> ApiResult<()> {
    let endpoint = format!("{}/account/change-password", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token.to_string())
        .json(&details)
        .send()
        .await;
    
    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(err_status_code_to_msg(&res).await),
        _ => Err("Undefined error".into())
    }
}

pub async fn account_logout(token: &Uuid) -> ApiResult<()> {
    let endpoint = format!("{}/account/logout", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token.to_string())
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(err_status_code_to_msg(&res).await),
        _ => Err("Undefined error".into())
    }
}

pub async fn account_get_active_token_info(token: &Uuid) -> ApiResult<Vec<LoginTokenInfo>> {
    let endpoint = format!("{}/account/tokens", BASE_URI);

    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token.to_string())
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status() == StatusCode::OK => res,
        Ok(res) => return Err(err_status_code_to_msg(&res).await),
        Err(_) => return Err("Undefined error".into())
    };

    match response.json::<Vec<LoginTokenInfo>>().await {
        Ok(info) => Ok(info),
        Err(_) => Err("Undefined error".into())
    }
}

pub async fn account_clear_tokens(token: &Uuid) -> ApiResult<()> {
    let endpoint= format!("{}/account/clear-tokens", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token.to_string())
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(err_status_code_to_msg(&res).await),
        _ => Err("Undefined error".into())
    }
}

// Room management

pub async fn chat_get_rooms(token: &Uuid) -> ApiResult<Vec<ChatRoom>> {
    let endpoint= format!("{}/chat/rooms", BASE_URI);

    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token.to_string())
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status() == StatusCode::OK => res,
        Ok(res) => return Err(err_status_code_to_msg(&res).await),
        Err(_) => return Err("Undefined error".into()),
    };

    match response.json::<Vec<ChatRoom>>().await {
        Ok(rooms) => Ok(rooms),
        Err(_) => Err("Undefined error".into())
    }
}

pub async fn chat_create_room(token: &Uuid, room_name: &str) -> ApiResult<()> {
    let endpoint= format!("{}/chat/create-room", BASE_URI);

    let body = ChatRoomName { room_name: room_name.to_string() };

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token.to_string())
        .json(&body)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(err_status_code_to_msg(&res).await),
        _ => Err("Undefined error".into())
    }
}

pub async fn chat_change_name(token: &Uuid, room_id: u64, new_name: &str) -> ApiResult<()> {
    let endpoint = format!("{}/chat/{}/change-name", BASE_URI, room_id);

    let body = ChatRoomName { room_name: new_name.to_string() };

    let response = reqwest::Client::new()
        .put(endpoint)
        .bearer_auth(token.to_string())
        .json(&body)
        .send()
        .await;
    
    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(err_status_code_to_msg(&res).await),
        _ => Err("Undefined error".into())
    }
}

pub async fn chat_get_members(token: &Uuid, room_id: u64) -> ApiResult<Vec<UserList>> {
    let endpoint = format!("{}/chat/{}/members", BASE_URI, room_id);

    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token.to_string())
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status() == StatusCode::OK => res,
        Ok(res) => return Err(err_status_code_to_msg(&res).await),
        Err(_) => return Err("Undefined error".into())
    };

    match response.json::<Vec<UserList>>().await {
        Ok(members) => Ok(members),
        Err(_) => Err("Undefined error".into())
    }
}

pub async fn chat_manage_user(token: &Uuid, room_id: u64, action: ChatRoomManageUser) -> ApiResult<()> {
    let endpoint = format!("{}/chat/{}/manage-user", BASE_URI, room_id);

    let response = reqwest::Client::new()
        .put(endpoint)
        .bearer_auth(token.to_string())
        .json(&action)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(err_status_code_to_msg(&res).await),
        _ => Err("Undefined error".into())
    }
}

// Util

async fn err_status_code_to_msg(response: &Response) -> String {
    match response.status() {
        StatusCode::BAD_REQUEST  => "Bad Request".into(),
        StatusCode::UNAUTHORIZED => "Unauthorized".into(),
        StatusCode::INTERNAL_SERVER_ERROR => "Internal Server Error".into(),
        _ => {
            log!(format!("api_service err_status_code_to_msg uncaught status {}", response.status().as_u16()));
            "".into()
        }
    }
}
