use std::str::FromStr;

use gloo_storage::Storage;
use uuid::Uuid;
use gloo::{console::log, storage::LocalStorage};
use yewdux::prelude::*;

pub trait StoreDispatchExt {
    fn login_reduce(&self, username: String, user_id: u64, token: Uuid) -> ();
    fn logout_reduce(&self) -> ();
}

#[derive(Clone, PartialEq, Store)]
pub struct Store {
    pub user: Option<User>,
}

impl Default for Store {
    fn default() -> Self {
        let local_username = LocalStorage::get::<String>("user.username");
        let local_user_id = LocalStorage::get::<u64>("user.user_id");
        let local_token = LocalStorage::get::<String>("user.token");

        let local_parsed_token = match local_token {
            Ok(token_str) => Uuid::from_str(&token_str).ok(),
            Err(_) => None,
        };

        let user = match (local_username, local_user_id, local_parsed_token) {
            (Ok(username), Ok(id), Some(token)) => Some(User::from_storage(username, id, token)),
            _ => None
        };
        
        Self { user }
    }
}

#[derive(Clone, Default, PartialEq, Store)]
pub struct User {
    pub username: String,
    pub user_id: u64,
    pub token: Uuid,
}

impl User {
    fn from_storage(username: String, user_id: u64, token: Uuid) -> Self {
        Self { username, user_id, token }
    }
}

impl StoreDispatchExt for Dispatch<Store> {
    fn login_reduce(&self, username: String, user_id: u64, token: Uuid) -> () {
        let data = User { username, user_id, token };
        let mut local_storage_failed = false;
        local_storage_failed |= LocalStorage::set("user.username", &data.username).is_err();
        local_storage_failed |= LocalStorage::set("user.user_id", &data.user_id).is_err();
        local_storage_failed |= LocalStorage::set("user.token", data.token.to_string()).is_err();
        self.reduce_mut(move |store| {
            store.user = Some(data);
        });
        if local_storage_failed {
            log!("local storage write failed on login");
        }
    }

    fn logout_reduce(&self) -> () {
        LocalStorage::delete("user.username");
        LocalStorage::delete("user.user_id");
        LocalStorage::delete("user.token");

        self.reduce_mut(move |store| {
            store.user = None;
        })
    }
}