use std::ops::Deref;

use gloo::console::log;
use yew::prelude::*;

use common::AccountPasswordChange;
use yew_router::hooks::use_navigator;

use crate::components::{
    button::Button,
    input_field::InputField
};

use super::{
    password_offline_check,
    AccountErrorReason
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Result<AccountPasswordChange, ()>>
}

#[derive(Default, Clone)]
struct Form {
    old_password: String,
    new_password: String,
    new_password_confirm: String,
    error: Option<AccountErrorReason>
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
            let confirm_ref = &updated_state.new_password_confirm;
            updated_state.error = match password_offline_check(&text, confirm_ref) {
                Ok(()) => None,
                Err(e) => Some(e)
            };
            updated_state.new_password = text;
            form_state.set(updated_state);
        })
    };

    let new_password_confirm_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = form_state.deref().clone();
            let new_ref = &updated_state.new_password;
            updated_state.error = match password_offline_check(&text, new_ref) {
                Ok(()) => None,
                Err(e) => Some(e)
            };
            updated_state.new_password_confirm = text;
            form_state.set(updated_state);
        })
    };

    let on_submit = {
        let form_state = form_state.clone();
        let props_on_submit = props.on_submit.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            
            let old_password = &form_state.old_password;
            let new_password = &form_state.new_password;
            let new_confirm = &form_state.new_password_confirm;

            let old_check = password_offline_check(old_password, old_password);
            let new_check = password_offline_check(new_password, new_confirm);

            match (old_check, new_check) {
                (Ok(_), Ok(_)) => props_on_submit.emit(Ok(
                    AccountPasswordChange {
                        old_password: old_password.clone(),
                        new_password: new_password.clone()
                    }
                )),
                (Ok(_), Err(e)) => {
                    let mut updated_state = form_state.deref().clone();
                    updated_state.error = Some(e);
                    form_state.set(updated_state);
                    props_on_submit.emit(Err(()))
                },
                (Err(e), Ok(_)) => {
                    let mut updated_state = form_state.deref().clone();
                    updated_state.error = Some(e);
                    form_state.set(updated_state);
                    props_on_submit.emit(Err(()))
                },
                (Err(e), Err(_)) => {
                    let mut updated_state = form_state.deref().clone();
                    updated_state.error = Some(e);
                    form_state.set(updated_state);
                    props_on_submit.emit(Err(()))
                },
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
            <h1>{ "Change password" }</h1>
            <InputField name="old password" password=true on_change={old_password_changed} />
            <br />
            <InputField name="new password" password=true on_change={new_password_changed} />
            <br />
            <InputField name="new password confirm" password=true on_change={new_password_confirm_changed} />
            if let Some(error) = &form_state.error {
                <p>{ "(new) " }{ error.to_string() }</p>
            }
            <br />
            <Button label="Change password" />
            <Button label="Cancel" on_click={Some(on_cancel)} />
        </form>
    }
}