use std::ops::Deref;

use common::AccountRequest;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{api_service, components::molecules::registration_form::RegistrationForm, router::Route, store::Store};

#[derive(PartialEq)]
enum RegisterStatus {
    NotAttempted,
    Success,
    Failed
}

#[function_component(RegistrationPage)]
pub fn registration_page() -> Html {
    // Global state
    let (store, _) = use_store::<Store>();

    // Redirect to Home if already logged in
    if store.token.is_some() {
        return html! {
            <Redirect<Route> to={Route::Home}/>
        }
    }

    // Component state
    let status = use_state(|| RegisterStatus::NotAttempted);
    let render_status = status.clone();

    let on_submit = {
        Callback::from(move |valid_form: Result<AccountRequest, ()>| {
            let status = status.clone();
            match valid_form {
                Ok(acc_req) => wasm_bindgen_futures::spawn_local(async move {
                    match api_service::account_register(acc_req).await {
                        Ok(()) => status.set(RegisterStatus::Success),
                        Err(e) => {
                            log!(format!("Registration failed: {}", e));
                            status.set(RegisterStatus::Failed)
                        }
                    }
                }),
                Err(_) => {}  // do nothing
            }
        })
    };

    let status_msg = match render_status.deref() {
        RegisterStatus::NotAttempted => None,
        RegisterStatus::Success      => Some("Success"),
        RegisterStatus::Failed       => Some("Failed"),
    };

    html! {
        <div>
            <h1>{ "registration page" }</h1>
            <RegistrationForm on_submit={on_submit} />
            if let Some(msg) = status_msg {
                <p>{msg}</p>
            }
        </div>
    }
}