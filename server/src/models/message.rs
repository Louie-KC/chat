use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Message {
    pub message_id: Option<String>,
    pub to: String,
    pub from: String,
    pub content: String,
    pub time: Option<DateTime<Utc>>
}