use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::account::AccountManagementPage;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::registration::RegistrationPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/account/register")]
    AccountRegister,
    #[at("/account/login")]
    AccountLogin,
    #[at("/account/manage")]
    AccountManage,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::AccountRegister => html! { <RegistrationPage /> },
        Route::AccountLogin => html! { <LoginPage /> },
        Route::AccountManage => html! { <AccountManagementPage /> },
        Route::NotFound => html! { <p1>{ "404 - Not Found" }</p1> }
    }
}