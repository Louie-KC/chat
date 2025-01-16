use awc::http::StatusCode;
use common::{
    AccountPasswordChange,
    AccountRequest,
    ChatRoom,
    ChatRoomManageUser,
    ChatRoomName,
    LoginResponse, UserList
};

struct ApiService {
    base_uri: String,
    client: awc::Client
}

impl ApiService {

    pub fn new(base_uri: String) -> Self {
        ApiService { base_uri: base_uri, client: awc::Client::default() }
    }

    // Account management    
    pub async fn account_register(self, details: AccountRequest) -> Result<(), ()> {
        let endpoint = format!("{}/account/register", self.base_uri);

        let response = self.client
            .post(endpoint)
            .send_json(&details)
            .await;

        match response {
            Ok(res) if res.status() == StatusCode::OK => Ok(()),
            _ => Err(())
        }
    }
    
    pub async fn account_login(self, details: AccountRequest) -> Result<LoginResponse, ()> {
        let endpoint = format!("{}/account/login", self.base_uri);
    
        let response = self.client
            .post(endpoint)
            .send_json(&details)
            .await;

        let mut response = match response {
            Ok(res) => res,
            Err(_) => return Err(()),
        };
    
        match response.json::<LoginResponse>().await {
            Ok(res) => Ok(res),
            Err(_) => Err(()),
        }
    }
    
    pub async fn account_change_password(self, token: &str, details: AccountPasswordChange) -> Result<(), ()> {
        let endpoint = format!("{}/account/change-password", self.base_uri);
    
        let response = self.client
            .post(endpoint)
            .bearer_auth(token)
            .send_json(&details)
            .await;
        
        match response {
            Ok(res) if res.status() == StatusCode::OK => Ok(()),
            _ => Err(())
        }
    }
    
    pub async fn account_logout(self, token: &str) -> Result<(), ()> {
        let endpoint = format!("{}/account/logout", self.base_uri);
    
        let response = self.client
            .post(endpoint)
            .bearer_auth(token)
            .send()
            .await;
    
        match response {
            Ok(res) if res.status() == StatusCode::OK => Ok(()),
            _ => Err(())
        }
    }
    
    pub async fn account_clear_tokens(self, token: &str) -> Result<(), ()> {
        let endpoint= format!("{}/account/clear-tokens", self.base_uri);
    
        let response = self.client
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
    
    pub async fn chat_get_rooms(self, token: &str) -> Result<Vec<ChatRoom>, ()> {
        let endpoint= format!("{}/chat/rooms", self.base_uri);
    
        let response = self.client
            .get(endpoint)
            .bearer_auth(token)
            .send()
            .await;
    
        let mut response = match response {
            Ok(res) => res,
            Err(_) => return Err(()),
        };
    
        match response.json::<Vec<ChatRoom>>().await {
            Ok(rooms) => Ok(rooms),
            Err(_) => Err(()),
        }
    }
    
    pub async fn chat_create_room(self, token: &str, room_name: &str) -> Result<(), ()> {
        let endpoint= format!("{}/chat/create-room", self.base_uri);
    
        let body = ChatRoomName { room_name: room_name.to_string() };
    
        let response = self.client
            .post(endpoint)
            .bearer_auth(token)
            .send_json(&body)
            .await;
    
        match response {
            Ok(res) if res.status() == StatusCode::OK => Ok(()),
            _ => Err(())
        }
    }
    
    pub async fn chat_change_name(self, token: &str, room_id: u64, new_name: &str) -> Result<(), ()> {
        let endpoint = format!("{}/chat/{}/change-name", self.base_uri, room_id);

        let body = ChatRoomName { room_name: new_name.to_string() };
    
        let response = self.client
            .put(endpoint)
            .bearer_auth(token)
            .send_json(&body)
            .await;
        
        match response {
            Ok(res) if res.status() == StatusCode::OK => Ok(()),
            _ => Err(())
        }
    }
    
    pub async fn chat_get_members(self, token: &str, room_id: u64) -> Result<Vec<UserList>, ()> {
        let endpoint = format!("{}/chat/{}/members", self.base_uri, room_id);
    
        let response = self.client
            .get(endpoint)
            .bearer_auth(token)
            .send()
            .await;
    
        let mut response = match response {
            Ok(res) => res,
            Err(_) => return Err(())
        };
    
        match response.json::<Vec<UserList>>().await {
            Ok(members) => Ok(members),
            Err(_) => Err(()),
        }
    }
    
    pub async fn chat_manage_user(self, token: &str, room_id: u64, action: ChatRoomManageUser) -> Result<(), ()> {
        let endpoint = format!("{}/chat/{}/manage-user", self.base_uri, room_id);
    
        let response = self.client
            .put(endpoint)
            .bearer_auth(token)
            .send_json(&action)
            .await;
    
        match response {
            Ok(res) if res.status() == StatusCode::OK => Ok(()),
            _ => Err(())
        }
    }
}

