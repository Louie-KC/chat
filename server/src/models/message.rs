use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct NewMessage {
    pub id: Option<String>,
    pub content: String,
    pub time: Option<DateTime<Utc>>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MessageRequest {
    pub from_time: DateTime<Utc>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub sender_id: String,
    pub chat_id: String,
    pub content: String,
    pub time_sent: NaiveDateTime
}