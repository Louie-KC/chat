use common::LoginTokenInfo;
use gloo::console::log;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, prelude::Redirect};
use yewdux::use_store;

use crate::{
    api_service,
    components::{
        atoms::{
            button::Button,
            token_info::TokenInfo
        },
        molecules::list_view::ListView
    },
    router::Route,
    store::{
        Store,
        StoreDispatchExt
    }
};

#[function_component(AccountManagementPage)]
pub fn account_management_page() -> Html {
    // Global state
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<Store>();

    // Redirect to Home if not logged in
    if store.user.is_none() {
        return html! {
            <Redirect<Route> to={Route::Home}/>
        }
    }

    // Component state
    let token_info = use_state_eq(|| Vec::<LoginTokenInfo>::new());

    // Update token_info state if needed
    if let Some(user_data) = store.user.clone() {
        let token_info = token_info.clone();
        let user_token = user_data.token;
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(info) = api_service::account_get_active_token_info(&user_token).await {
                token_info.set(info);
            }
        })
    }

    // Convert token info to renderable items
    let token_children_info: Vec<Html> = token_info.iter()
        .map(|info| html! { <TokenInfo info={info.clone()}/>})
        .collect();

    let on_refresh_tokens = {
        let token_info = token_info.clone();
        let store = store.clone();
        Callback::from(move |_: MouseEvent| {
            let token_info = token_info.clone();
            let store = store.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let store = store.clone();
                let user_token = match &store.user {
                    Some(user) => user.token,
                    None => return
                };
                if let Ok(info) = api_service::account_get_active_token_info(&user_token).await {
                    token_info.set(info);
                }
            })
        })
    };

    let on_clear_tokens = {
        let navigator = navigator.clone();
        let store = store.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let navigator = navigator.clone();
            let store = store.clone();
            let dispatch = dispatch.clone();
            if let Some(user_data) = store.user.clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(_) = api_service::account_clear_tokens(&user_data.token).await {
                        dispatch.logout_reduce();
                        navigator.push(&Route::Home);
                    } else {
                        log!("Clear request failed");
                    }
                });
            } else {
                log!("Clear request failed - Missing auth token");
            }
        })
    };

    let on_logout = {
        let navigator = navigator.clone();
        let store = store.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let navigator = navigator.clone();
            let store = store.clone();
            let dispatch = dispatch.clone();
            if let Some(user_data) = store.user.clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(_) = api_service::account_logout(&user_data.token).await {
                        dispatch.logout_reduce();
                        navigator.push(&Route::Home);
                    } else {
                        log!("Logout request failed");
                    }
                });
            } else {
                log!("Logout request failed - Missing auth token");
            }
        })
    };

    let on_change_password = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            navigator.push(&Route::AccountChangePassword);
        })
    };

    html! {
        <>
            <h>{"Currently logged in devices/tokens"}</h>
            <Button label={"Refresh list"} on_click={Some(on_refresh_tokens)} />
            <ListView children={token_children_info} />
            <Button label={"Change password"} on_click={Some(on_change_password)} />
            <br />
            <Button label={"Log out"} on_click={Some(on_logout)} />
            <br />
            <Button label={"Log out of all devices"} on_click={Some(on_clear_tokens)} />
        </>
    }
}