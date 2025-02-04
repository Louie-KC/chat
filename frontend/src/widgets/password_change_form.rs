use std::ops::Deref;

use gloo::console::log;
use yew::prelude::*;

use common::AccountPasswordChange;
use yew_router::hooks::use_navigator;

use crate::components::{
    button::Button,
    input_field::InputField
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Result<AccountPasswordChange, ()>>
}

#[derive(Default, Clone)]
struct Form {
    old_password: String,
    new_password: String,
    new_password_confirm: String
}

#[function_component(PasswordChangeForm)]
pub fn password_change_form(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let form_state = use_state(|| Form::default());

    let old_password_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = form_state.deref().clone();
            updated_state.old_password = text;
            form_state.set(updated_state);
        })
    };
    
    let new_password_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = form_state.deref().clone();
            updated_state.new_password = text;
            form_state.set(updated_state);
        })
    };

    let new_password_confirm_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = form_state.deref().clone();
            updated_state.new_password_confirm = text;
            form_state.set(updated_state);
        })
    };

    let on_submit = {
        let form_state = form_state.clone();
        let props_on_submit = props.on_submit.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            
            let old_password = form_state.old_password.clone();
            let new_password = form_state.new_password.clone();
            let new_confirm = &form_state.new_password_confirm;

            let new_matches = new_password.eq(new_confirm);
            let new_old_different = !old_password.eq(&new_password);

            if new_matches && new_old_different && !new_password.is_empty() {
                props_on_submit.emit(Ok(AccountPasswordChange {old_password, new_password}))
            } else {
                props_on_submit.emit(Err(()))
            }
        })
    };

    let on_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            log!("on_cancel invoked");
            navigator.back();
        })
    };

    html! {
        <form onsubmit={on_submit} class={classes!("account_form")}>
            <InputField name="old password" password=true on_change={old_password_changed} />
            <br />
            <InputField name="new password" password=true on_change={new_password_changed} />
            <br />
            <InputField name="new password confirm" password=true on_change={new_password_confirm_changed} />
            <br />
            <Button label="Change password" />
            <Button label="Cancel" on_click={Some(on_cancel)} />
        </form>
    }
}