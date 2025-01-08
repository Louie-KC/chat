use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRequest {
    pub username: String,
    pub password: String,
}
