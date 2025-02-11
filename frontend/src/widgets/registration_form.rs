use std::ops::Deref;

use common::AccountRequest;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    components::{
        button::Button,
        input_field::InputField
    },
    router::Route,
    widgets::{
        password_offline_check,
        username_offline_check
    }
};

use super::AccountErrorReason;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Result<AccountRequest, ()>>,
}

#[derive(Default, Clone)]
pub struct FormState {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
    error: Option<AccountErrorReason>
}

#[function_component(RegistrationForm)]
pub fn registration_form(props: &Props) -> Html {
    let form_state = use_state(|| FormState::default());

    let username_changed = {
        let username_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = username_state.deref().clone();
            updated_state.username = text;
            updated_state.error = match username_offline_check(&updated_state.username) {
                Ok(()) => None,
                Err(e) => Some(e)
            };
            username_state.set(updated_state);
        })
    };

    let password_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_form = form_state.deref().clone();
            updated_form.password = text;
            if updated_form.error.is_none() {
                let password = &updated_form.password;
                let password_confirm = &updated_form.password_confirm;
                updated_form.error = match password_offline_check(password, password_confirm) {
                    Ok(()) => None,
                    Err(e) => Some(e)
                };
            }
            form_state.set(updated_form);
        })
    };

    let password_confirm_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_form = form_state.deref().clone();
            updated_form.password_confirm = text;
            if updated_form.error.is_none() {
                let password = &updated_form.password;
                let password_confirm = &updated_form.password_confirm;
                updated_form.error = match password_offline_check(password, password_confirm) {
                    Ok(()) => None,
                    Err(e) => Some(e)
                };
            }
            form_state.set(updated_form);
        })
    };

    let on_submit = {
        let props_on_submit = props.on_submit.clone();
        let form_state = form_state.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let mut mutable_form = form_state.deref().clone();
            let username = &form_state.username;
            let password = &form_state.password;
            let password_confirm = &form_state.password_confirm;

            match (username_offline_check(username), password_offline_check(&password, &password_confirm)) {
                (Ok(_), Ok(_)) => {
                    let username = form_state.username.clone();
                    let password = form_state.password.clone();
                    props_on_submit.emit(Ok(AccountRequest { username, password }))
                },
                (Err(r), Err(_)) => {
                    mutable_form.error = Some(r);
                    form_state.set(mutable_form);
                },
                (Ok(_), Err(r)) => {
                    mutable_form.error = Some(r);
                    form_state.set(mutable_form);
                },
                (Err(r), Ok(_)) => {
                    mutable_form.error = Some(r);
                    form_state.set(mutable_form);
                },
            }
        })
    };

    html! {
        <form onsubmit={on_submit} class={classes!("account_form")}>
            <h1>{ "Create an account" }</h1>
            <InputField name="username" on_change={username_changed} />
            <br />
            <InputField name="password" password=true on_change={password_changed} />
            <br />
            <InputField name="confirm password" password=true on_change={password_confirm_changed} />
            if let Some(error) = &form_state.error {
                <p>{ error.to_string() }</p>
            }
            <br />
            <Button label="Register" />
            <Link<Route> to={Route::AccountLogin}> {"Already have an account?"} </Link<Route>>
        </form>
    }
}