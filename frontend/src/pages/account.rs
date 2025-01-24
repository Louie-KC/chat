use common::LoginTokenInfo;
use gloo::console::log;
use yew::prelude::*;
use yew_router::{hooks::use_navigator, prelude::Redirect};
use yewdux::use_store;

use crate::{api_service, components::{atoms::{button::Button, token_info::TokenInfo}, molecules::list_view::ListView}, router::Route, store::Store};

#[function_component(AccountManagementPage)]
pub fn account_management_page() -> Html {
    // Global state
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<Store>();

    // Redirect to Home if not logged in
    if store.token.is_none() {
        return html! {
            <Redirect<Route> to={Route::Home}/>
        }
    }

    // Component state
    let token_info = use_state(|| Vec::<LoginTokenInfo>::new());

    // Update token_info state if needed
    match store.clone().token {
        Some(token) => {
            let token_info = token_info.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(info) = api_service::account_get_active_token_info(&token).await {
                    token_info.set(info);
                }
            })
        },
        None => {}
    };

    let on_clear_tokens = {
        let navigator = navigator.clone();
        let store = store.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let navigator = navigator.clone();
            let store = store.clone();
            let dispatch = dispatch.clone();
            if let Some(token) = store.token {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(_) = api_service::account_clear_tokens(&token).await {
                        dispatch.reduce_mut(move |store| {
                            store.username = None;
                            store.user_id = None;
                            store.token = None
                        });
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
            if let Some(token) = store.token {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(_) = api_service::account_logout(&token).await {
                        dispatch.reduce_mut(move |store| {
                            store.username = None;
                            store.user_id = None;
                            store.token = None;
                        });
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

    let token_children_info: Vec<Html> = token_info.iter()
        .map(|info| html! { <TokenInfo info={info.clone()}/>})
        .collect();

    html! {
        <>
            <h>{"Currently logged in devices/tokens"}</h>
            <ListView children={token_children_info} />
            <Button label={"Log out"} on_click={Some(on_logout)} />
            <br />
            <Button label={"Log out of all devices"} on_click={Some(on_clear_tokens)} />
        </>
    }
}