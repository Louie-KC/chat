use std::str::FromStr;

use common::AccountRequest;
use gloo::console::log;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::api_service;
use crate::components::molecules::login_form::LoginForm;
use crate::router::Route;
use crate::store::{Store, StoreDispatchExt};

#[derive(PartialEq, Clone)]
enum LoginStatus {
    NotAttempted,
    Failed,
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    // Global controls & state
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<Store>();

    // Redirect to Home if already logged in
    if store.user.is_some() {
        return html! {
            <Redirect<Route> to={Route::Home}/>
        }
    }

    // Component state
    let status = use_state(|| LoginStatus::NotAttempted);
    let render_status = status.clone();

    let on_submit = {
        Callback::from(move |user: AccountRequest| {
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api_service::account_login(&user).await {
                    Ok(response) => {
                        match Uuid::from_str(&response.token) {
                            Ok(token) => {
                                dispatch.login_reduce(user.username, response.user_id, token);
                                navigator.push(&Route::Home);
                            },
                            Err(_) => {
                                log!("The stored token is in an invalid format");
                                status.set(LoginStatus::Failed);
                            },
                        }
                    },
                    Err(_) => {
                        status.set(LoginStatus::Failed);
                    }
                }

            });
        })
    };

    html! {
        <>
            <h1>{ "login page" }</h1>
            <LoginForm on_submit={on_submit}/>
            if (&*render_status).eq(&LoginStatus::Failed) {
                <p>{ "Incorrect details" }</p>
            }
        </>
    }
}