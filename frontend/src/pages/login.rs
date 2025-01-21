use common::AccountRequest;
use yew::prelude::*;
use yewdux::use_store;

use crate::api_service;
use crate::components::molecules::login_form::LoginForm;
use crate::store::Store;

#[derive(PartialEq, Clone)]
enum LoginStatus {
    NotAttempted,
    Failed,
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {

    let (_, dispatch) = use_store::<Store>();
    let status = use_state(|| LoginStatus::NotAttempted);
    let render_status = status.clone();
    let on_submit = {
        Callback::from(move |user: AccountRequest| {
            let dispatch = dispatch.clone();
            let status = status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api_service::account_login(&user).await {
                    Ok(response) => {
                        dispatch.reduce_mut(move |store| {
                            store.username = user.username;
                            store.user_id = response.user_id;
                            store.token = response.token;
                        });
                        todo!("Navigate away on successful login")
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