use std::ops::Deref;

use common::UserInfo;
use yew::prelude::*;
use yewdux::use_store;

use crate::{
    api_service,
    components::{
        input_field::InputField,
        user::UserDetailComponent
    },
    store::Store
};

const MIN_SEARCH_LEN: usize = 2;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_user_click: Callback<u64>
}

#[derive(PartialEq, Clone, Default)]
struct State {
    search_text: String,
    search_result: Vec<UserInfo>
}

#[function_component(UserSearch)]
pub fn user_search(props: &Props) -> Html {
    let (store, _) = use_store::<Store>();
    let token = store.user.clone().unwrap().token;

    let component_state = use_state_eq(|| State::default());
    
    let state_handle = component_state.clone();
    let search_text_changed = Callback::from(move |text: String| {
        let state_handle = state_handle.clone();
        let mut updated_state = state_handle.deref().clone();
        if text.len() < MIN_SEARCH_LEN {
            return
        }
        wasm_bindgen_futures::spawn_local(async move {
            match api_service::user_search(&token, &text.clone()).await {
                Ok(search_result) => {
                    updated_state.search_result = search_result;
                },
                Err(_) => {},
            }
            state_handle.set(updated_state);
        });
    });

    let props_handle = props.clone();
    let on_user_clicked = {
        Callback::from(move |selected_user_id: u64| {
            props_handle.on_user_click.emit(selected_user_id)
        })
    };
    
    html! {
        <>
        <InputField name={"username"} on_change={search_text_changed} />
        <ul>
            {
            component_state.search_result.iter()
                .map(|user| {
                    html! {
                        <UserDetailComponent data={user.clone()} on_select={on_user_clicked.clone()} />
                    }
                })
                .collect::<Vec<Html>>()
            }
            </ul>
        </>
    }
}