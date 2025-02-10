use std::ops::Deref;

use yew::prelude::*;
use yewdux::use_store;

use crate::{api_service, components::{button::Button, input_field::InputField}, store::Store};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub notify: Callback<bool>
}

#[derive(PartialEq, Clone, Default)]
struct FormState {
    pub name: String
}

#[function_component(NewRoomForm)]
pub fn new_room_form(props: &Props) -> Html {
    // Global state
    let (store, _) = use_store::<Store>();
    let token = store.user.clone().unwrap().token.clone();

    // Component state
    let form_state = use_state_eq(|| FormState::default());

    let on_name_change = {
        let state_handle = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = state_handle.deref().clone();
            updated_state.name = text;
            state_handle.set(updated_state);
        })
    };

    let on_create = {
        let props_callback = props.notify.clone();
        let state_handle = form_state.clone();
        Callback::from(move |_: MouseEvent| {
            let props_callback = props_callback.clone();
            let state_handle = state_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let success = api_service::chat_create_room(&token, &state_handle.name).await.is_ok();
                props_callback.emit(success)
            });
        })
    };

    let on_cancel = {
        let props_callback = props.notify.clone();
        Callback::from(move |_: MouseEvent| {
            props_callback.emit(false)
        })
    };

    html! {
        <>
            <InputField name={"Room name"} on_change={on_name_change} />
            <Button label={"Create"} on_click={Some(on_create)} />
            <Button label={"Cancel"} on_click={Some(on_cancel)} />
        </>
    }
}