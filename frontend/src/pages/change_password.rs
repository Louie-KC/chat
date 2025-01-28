use common::AccountPasswordChange;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{api_service, components::molecules::password_change_form::PasswordChangeForm, router::Route, store::Store};

#[function_component(ChangePasswordPage)]
pub fn change_password_page() -> Html {
    let navigator = use_navigator().unwrap();
    let (store, _) = use_store::<Store>();

    if store.token.is_none() {
        log!("ChangePasswordPage redirect");
        return html! {
            <Redirect<Route> to={Route::AccountLogin} />
        }
    }

    // Component state
    let failed = use_state(|| false);
    let render_failed = failed.clone();
    let token = Box::new(store.token.unwrap());

    let on_submit = {
        Callback::from(move |change_form: Result<AccountPasswordChange, ()>| {
            let token = token.clone();
            let navigator = navigator.clone();
            let failed = failed.clone();
            match change_form {
                Ok(change_req) => wasm_bindgen_futures::spawn_local(async move {
                    match api_service::account_change_password(&token, change_req).await {
                        Ok(()) => {
                            navigator.push(&Route::AccountManage);
                        },
                        Err(_) => failed.set(true),
                    }
                }),
                Err(_) => failed.set(true)
            }
        })
    };

    html! {
        <>
            <h1>{ "Change password" }</h1>
            <PasswordChangeForm on_submit={on_submit} />
            if *render_failed {
                <p>{ "Request failed" }</p>
            }
        </>
    }
}