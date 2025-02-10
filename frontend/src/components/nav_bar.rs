use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{router::Route, store::Store};

#[function_component]
pub fn NavBar() -> Html {
    let (store, _) = use_store::<Store>();

    html! {
        <nav class={classes!("topnav")}>
            <Link<Route> to={Route::Home}> {"Home"} </Link<Route>>
            if store.user.is_none() {
                <Link<Route> to={Route::AccountRegister}> {"Register"} </Link<Route>>
                <Link<Route> to={Route::AccountLogin}> {"Login"} </Link<Route>>
            } else {
                <Link<Route> to={Route::Chats}> {"Chat"} </Link<Route>>
                <Link<Route> to={Route::Associations}> {"Associations"} </Link<Route>>
                <Link<Route> to={Route::AccountManage}> {"Manage Account"} </Link<Route>>
            }
        </nav>
    }
}