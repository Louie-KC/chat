use common::AccountRequest;
use yew::prelude::*;

use crate::components::molecules::login_form::LoginForm;
use gloo::console::log;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let on_submit = Callback::from(move |user: AccountRequest| {
        log!(format!("username: {}, password: {}", user.username, user.password));
    });

    html! {
        <>
            <h1>{ "login page" }</h1>
            <LoginForm on_submit={on_submit}/>
        </>
    }
}