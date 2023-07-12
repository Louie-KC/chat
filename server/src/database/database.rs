use std::{sync::{Arc, Mutex}, collections::HashMap};

use chrono::Utc;

use crate::models::message::Message;

/// Mock database

type Token = String;

pub struct Database {
    user_list: Arc<Mutex<HashMap<(String, String), Token>>>,
    msg_db: Arc<Mutex<Vec<Message>>>,
    // live_tokens: Arc<Mutex<HashSet<String>>>
    token_map: Arc<Mutex<HashMap<Token, String>>>
}

impl Database {
    pub fn new() -> Database {
        let mut users: HashMap<(String, String), String> = HashMap::new();
        users.insert(("test".into(), "password".into()), "12345".into());

        let msg: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(vec![]));
        let u_list: Arc<Mutex<HashMap<(String, String), String>>> = Arc::new(Mutex::new(users));
        let tokens = Arc::new(Mutex::new(HashMap::new()));
        
        Database { user_list: u_list, msg_db: msg, token_map: tokens }
    }

    pub fn login(&self, uname: String, pword: String) -> Result<String, ()> {
        let users = self.user_list.lock().unwrap();
        let token = match users.get(&(uname.clone(), pword)) {
            Some(t) => t,
            None => return Err(())
        };

        let mut token_map = self.token_map.lock().unwrap();
        match token_map.contains_key(token) {
            true => return Err(()),
            false => { token_map.insert(token.clone(), uname); }
        }
        Ok(token.to_string())
    }

    pub fn valid_token(&self, token: &Token, user_id: &String) -> bool {
        let live_users = self.token_map.lock().unwrap();
        if let Some(uid) = live_users.get(token) {
            return user_id.eq(uid)
        }
        false
    }

    pub fn add_message(&self, to: String, from: String, content: String) -> Result<(), ()> {
        let mut msgs = self.msg_db.lock().unwrap();
        msgs.push(Message { to: to, from: from, content: content, time: Some(Utc::now()) });
        Ok(())
    }

    /// 
    pub fn get_messages_brief(&self, requester_id: &String) -> Vec<Message> {
        // let live = self.live_tokens.lock().unwrap();
        // if !live.contains(&requester_id) {
        //     return Err(());
        // }
        
        let msgs = self.msg_db.lock().unwrap();
        let result: Vec<Message> = msgs
            .iter()
            .filter(|msg| msg.to.eq(requester_id) || msg.from.eq(requester_id))
            .cloned()
            .collect();
        
        result
    }

    pub fn get_conversation_messages(&self, requester_id: &String, other_id: &String) -> Vec<Message> {
        // let live = self.live_tokens.lock().unwrap();
        // if !live.contains(&requester_id) {
        //     return Err(());
        // }
        let msgs = self.msg_db.lock().unwrap();
        let result: Vec<Message> = msgs
            .iter()
            .filter(|msg| {

                (msg.to.eq(requester_id) && msg.from.eq(other_id))
                || (msg.to.eq(other_id) && msg.from.eq(requester_id))
                // match (msg.to.as_str(), msg.from.as_str()) {
                //     (requester_id, other_id)
                //     | (other_id, requester_id) => true,
                //     _ => false
                // }
            })
            .cloned()
            .collect();
        
        result
    }

}