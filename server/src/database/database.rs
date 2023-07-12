use std::{sync::{Arc, Mutex}, collections::{HashMap, HashSet}};

use chrono::Utc;

use crate::models::message::Message;

/// Mock database

type Token = String;

pub struct Database {
    user_list: Arc<Mutex<HashMap<(String, String), Token>>>,
    msg_db: Arc<Mutex<Vec<Message>>>,
    live_tokens: Arc<Mutex<HashSet<Token>>>
}

impl Database {
    pub fn new() -> Database {
        let mut users: HashMap<(String, String), String> = HashMap::new();
        users.insert(("test".into(), "password".into()), "12345".into());

        let msg: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(vec![]));
        let u_list: Arc<Mutex<HashMap<(String, String), String>>> = Arc::new(Mutex::new(users));
        let tokens = Arc::new(Mutex::new(HashSet::new()));
        
        Database { user_list: u_list, msg_db: msg, live_tokens: tokens }
    }

    pub fn login(&self, uname: String, pword: String) -> Result<String, ()> {
        let users = self.user_list.lock().unwrap();
        let token = match users.get(&(uname, pword)) {
            Some(t) => t,
            None => return Err(())
        };

        let mut live_tokens = self.live_tokens.lock().unwrap();
        match live_tokens.contains(token) {
            true => return Err(()),
            false => { live_tokens.insert(token.clone()); }
        }
        Ok(token.to_string())
    }

    pub fn add_message(&self, to: String, from: String, content: String) -> Result<(), ()> {
        let mut msgs = self.msg_db.lock().unwrap();
        msgs.push(Message { to: to, from: from, content: content, time: Some(Utc::now()) });
        Ok(())
    }

    pub fn get_messages(&self, user_token: Token) -> Result<Vec<Message>, ()> {
        let live = self.live_tokens.lock().unwrap();
        if !live.contains(&user_token) {
            return Err(());
        }
        
        let msgs = self.msg_db.lock().unwrap();
        let result: Vec<Message> = msgs
            .iter()
            .filter(|msg| msg.to.eq(&user_token) || msg.from.eq(&user_token))
            .cloned()
            .collect();
        
        Ok(result)
    }

}