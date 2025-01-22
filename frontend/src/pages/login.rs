use common::AccountRequest;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::api_service;
use crate::components::molecules::login_form::LoginForm;
use crate::router::Route;
use crate::store::Store;

#[derive(PartialEq, Clone)]
enum LoginStatus {
    NotAttempted,
    Failed,
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    // Global controls & state
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<Store>();

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
                        dispatch.reduce_mut(move |store| {
                            store.username = Some(user.username);
                            store.user_id = Some(response.user_id);
                            store.token = Some(response.token);
                        });
                        navigator.push(&Route::Home);
                    },
                    Err(reason) => {
                        log!(format!("fail reason: {}", reason));
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