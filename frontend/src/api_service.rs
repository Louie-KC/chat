use common::{
    AccountPasswordChange, AccountRequest, ChatMessage, ChatRoom, ChatRoomManageUser, ChatRoomName, LoginResponse, LoginTokenInfo, UserInfo
};

use gloo::console::log;

use reqwest::{self, StatusCode};
use uuid::Uuid;

const BASE_URI: &str = "http://127.0.0.1:8000";

#[derive(Debug)]
pub enum ApiError {
    Timeout,
    Unauthorized,
    BadRequest,
    ResponseParseFailure,
    Other{ _desc: String }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        let mut result = None;

        if value.is_timeout() {
            result = Some(ApiError::Timeout)
        }
        if value.is_request() {
            result = Some(ApiError::BadRequest)
        }
        if value.is_decode() {
            result = Some(ApiError::ResponseParseFailure)
        }
        if result.is_none() {
            result = Some(ApiError::Other{
                _desc: format!("Non-covered reqwest error: {:?}", value.to_string())
            })
        }
        let result = result.unwrap();
        log!(format!("ApiService error: {:?}", result));
        result
    }
}

impl From<reqwest::Response> for ApiError {
    fn from(value: reqwest::Response) -> Self {
        let result = match value.status() {
            StatusCode::GATEWAY_TIMEOUT | StatusCode::REQUEST_TIMEOUT => ApiError::Timeout,
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => ApiError::Unauthorized,
            StatusCode::BAD_REQUEST => ApiError::BadRequest,
            _ => ApiError::Other{ _desc: value.status().canonical_reason().unwrap().into() }
        };
        
        log!(format!("ApiService error: {:?}", result));
        result
    }
}

type ApiResult<T> = Result<T, ApiError>;

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
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into())
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
        Ok(res) => return Err(res.into()),
        Err(err)   => return Err(err.into())
    };

    match response.json::<LoginResponse>().await {
        Ok(res) => Ok(res),
        Err(err) => Err(err.into())
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
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into())
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
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into())
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
        Ok(res) => return Err(res.into()),
        Err(err) => return Err(err.into())
    };

    match response.json::<Vec<LoginTokenInfo>>().await {
        Ok(info) => Ok(info),
        Err(err) => Err(err.into())
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
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into())
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
        Ok(res) => return Err(res.into()),
        Err(err) => return Err(err.into()),
    };

    match response.json::<Vec<ChatRoom>>().await {
        Ok(rooms) => Ok(rooms),
        Err(err) => Err(err.into())
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
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into())
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
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into())
    }
}

pub async fn chat_get_members(token: &Uuid, room_id: u64) -> ApiResult<Vec<UserInfo>> {
    let endpoint = format!("{}/chat/{}/members", BASE_URI, room_id);

    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token.to_string())
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status() == StatusCode::OK => res,
        Ok(res) => return Err(res.into()),
        Err(err) => return Err(err.into())
    };

    match response.json::<Vec<UserInfo>>().await {
        Ok(members) => Ok(members),
        Err(err) => Err(err.into())
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
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into())
    }
}

// Chat interaction

pub async fn chat_get_messages(token: &Uuid, room_id: u64, offset: u64, limit: u64) -> ApiResult<Vec<ChatMessage>> {
    let endpoint = format!("{}/chat/{}/{}/{}", BASE_URI, room_id, offset, limit);

    let response = reqwest::Client::new()
        .get(endpoint)
        .bearer_auth(token.to_string())
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status() == StatusCode::OK => res,
        Ok(res) => return Err(res.into()),
        Err(err) => return Err(err.into())
    };

    match response.json::<Vec<ChatMessage>>().await {
        Ok(message_window) => Ok(message_window),
        Err(err) => Err(err.into())
        
    }
}

pub async fn chat_send_message(token: &Uuid, message: ChatMessage) -> ApiResult<()> {
    let endpoint = format!("{}/chat", BASE_URI);

    let response = reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(token)
        .json(&message)
        .send()
        .await;

    match response {
        Ok(res) if res.status() == StatusCode::OK => Ok(()),
        Ok(res) => Err(res.into()),
        Err(err) => Err(err.into()),
    }
}
