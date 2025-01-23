use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{router::Route, store::Store};

#[function_component]
pub fn NavBar() -> Html {
    let (store, _) = use_store::<Store>();

    html! {
        <nav>
            <Link<Route> to={Route::Home}> {"Home"} </Link<Route>>
            if store.token.is_none() {
                <Link<Route> to={Route::AccountRegister}> {"Register"} </Link<Route>>
                <Link<Route> to={Route::AccountLogin}> {"Login"} </Link<Route>>
            } else {
                <Link<Route> to={Route::AccountManage}> {"Manage Account"} </Link<Route>>
            }
        </nav>
    }
}