use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct LoginToken {
    pub token: String
}