use std::{sync::{Arc, Mutex}, collections::HashMap};

use chrono::Utc;
use uuid::Uuid;
use crate::models::message::Message;

/// Mock database

type Token = String;
type UserID = String;

pub struct Database {
    user_list: Arc<Mutex<HashMap<UserID, String>>>,
    msg_db: Arc<Mutex<Vec<Message>>>,
    token_map: Arc<Mutex<Vec<(UserID, Token)>>>
}

impl Database {
    pub fn new() -> Database {
        let users: HashMap<UserID, String> = HashMap::new();

        let msg: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(vec![]));
        let u_list: Arc<Mutex<HashMap<UserID, String>>> = Arc::new(Mutex::new(users));
        let map = Arc::new(Mutex::new(vec![]));
        
        Database { user_list: u_list, msg_db: msg, token_map: map }
    }

    pub fn create_account(&self, uname: &str, pword: &str) -> Result<(), ()> {
        let mut users = self.user_list.lock().unwrap();
        if users.contains_key(uname) {
            return Err(())
        }

        users.insert(uname.to_string(), pword.to_string());
        Ok(())
    }

    pub fn login(&self, uname: &str, pword: &str) -> Result<String, ()> {
        let users = self.user_list.lock().unwrap();
        let valid_credentials = match users.get(uname) {
            Some(p) => p.eq(&pword),
            None    => false
        };
        if !valid_credentials {
            return Err(())
        }
        let token = Uuid::new_v4().to_string();
        let mut token_map = self.token_map.lock().unwrap();
        for (uid, _) in token_map.iter() {
            if uid.eq(&uname) {
                return Err(())
            }
        }
        token_map.push((uname.to_string(), token.clone()));
        Ok(token)

    }

    pub fn valid_token(&self, token: &str, user_id: &str) -> bool {
        let live_users = self.token_map.lock().unwrap();
        for (uid, tok) in live_users.iter() {
            if uid.eq(user_id) && tok.eq(token) {
                return true
            }
        }
        false
    }

    pub fn add_message(&self, to: &str, from: &str, content: &str) -> Result<(), ()> {
        let mut msgs = self.msg_db.lock().unwrap();
        let msg = Message {
            message_id: Some(Uuid::new_v4().to_string()),
            to: to.to_string(),
            from: from.to_string(),
            content: content.to_string(),
            time: Some(Utc::now()),
        };
        msgs.push(msg);
        Ok(())
    }

    pub fn get_messages_brief(&self, requester_id: &str) -> Vec<Message> {   
        let msgs = self.msg_db.lock().unwrap();
        let result: Vec<Message> = msgs
            .iter()
            .filter(|msg| msg.to.eq(requester_id) || msg.from.eq(requester_id))
            .cloned()
            .collect();
        
        result
    }

    pub fn get_conversation_messages(&self, requester_id: &str, other_id: &str) -> Vec<Message> {
        let msgs = self.msg_db.lock().unwrap();
        let result: Vec<Message> = msgs
            .iter()
            .filter(|msg| {
                (msg.to.eq(requester_id) && msg.from.eq(other_id))
                || (msg.to.eq(other_id) && msg.from.eq(requester_id))
            })
            .cloned()
            .collect();
        
        result
    }

}