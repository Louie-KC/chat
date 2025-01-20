use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav>
            <Link<Route> to={Route::Home}> {"Home"} </Link<Route>>
            <Link<Route> to={Route::AccountRegister}> {"Register"} </Link<Route>>
            <Link<Route> to={Route::AccountLogin}> {"Login"} </Link<Route>>
        </nav>
    }
}