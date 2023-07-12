use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub creation_date: Option<DateTime<Utc>>
}