use yewdux::prelude::*;

#[derive(Clone, Default, PartialEq, Store)]
pub struct Store {
    pub username: Option<String>,
    pub user_id: Option<u64>,
    pub token: Option<String>
}