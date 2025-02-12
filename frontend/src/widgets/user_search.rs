use std::ops::Deref;

use common::UserInfo;
use yew::prelude::*;
use yewdux::use_store;

use crate::{
    api_service,
    components::{
        button::Button,
        input_field::InputField,
        user::UserDetailComponent
    },
    store::Store,
    widgets::list_view::ListView,
};

const MIN_SEARCH_LEN: usize = 2;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub buttons: Vec<(String, Callback<u64>)>
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
            updated_state.search_result.clear();
            state_handle.set(updated_state);
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

    let list_html: Vec<Html> = component_state.search_result.iter()
        .map(|user| {
            let user_id = user.id;
            html! {
                <div class={classes!("user_button_row")}>
                    <UserDetailComponent data={user.clone()} />
                    {
                        for props.buttons.iter()
                            .map(|data| data.clone())
                            .map(|(label, callback)| {
                                let on_click = Callback::from(move |_: MouseEvent| {
                                    callback.emit(user_id)
                                });
                                html! {
                                    <Button label={label.clone()} on_click={Some(on_click)} />
                                }
                            })
                    }
                </div>
            }
        })
        .collect();
    
    html! {
        <>
            <InputField name={"username"} on_change={search_text_changed} />
            <ListView children={list_html} />
        </>
    }
}