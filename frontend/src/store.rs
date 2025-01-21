use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Store)]
pub struct Store {
    pub user_id: u64,
    pub username: String,
    pub token: String
}