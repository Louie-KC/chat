use std::ops::Deref;

use common::AccountRequest;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::{
    button::Button,
    input_field::InputField
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Result<AccountRequest, ()>>,
}

#[derive(Default, Clone)]
pub struct Form {
    pub username: String,
    pub password: String,
    pub password_confirm: String
}

#[function_component(RegistrationForm)]
pub fn registration_form(props: &Props) -> Html {
    let form_state = use_state(|| Form::default());

    let passwords_match = form_state.password.eq(&form_state.password_confirm);

    let username_changed = {
        let username_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = username_state.deref().clone();
            updated_state.username = text;
            username_state.set(updated_state);
        })
    };

    let password_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_form = form_state.deref().clone();
            updated_form.password = text;
            form_state.set(updated_form);
        })
    };

    let password_confirm_changed = {
        let form_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_form = form_state.deref().clone();
            updated_form.password_confirm = text;
            form_state.set(updated_form);
        })
    };

    let on_submit = {
        let props_on_submit = props.on_submit.clone();
        let form_state = form_state.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            match !form_state.password.is_empty() && passwords_match {
                true  => {
                    let username = form_state.username.clone();
                    let password = form_state.password.clone();
                    props_on_submit.emit(Ok(AccountRequest { username, password }))
                },
                false => {
                    log!("passwords are empty or are different");
                    props_on_submit.emit(Err(()))
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
            <br />
            <Button label="Register" />
            <Link<Route> to={Route::AccountLogin}> {"Already have an account?"} </Link<Route>>
        </form>
    }
}