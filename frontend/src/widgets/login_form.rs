use std::ops::Deref;

use common::AccountRequest;
use yew::prelude::*;

use crate::components::{
    button::Button,
    input_field::InputField
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<AccountRequest>
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let form_state = use_state(|| AccountRequest {
        username: String::new(),
        password: String::new()
    });

    let username_changed = {
        let username_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = username_state.deref().clone();
            updated_state.username = text;
            username_state.set(updated_state);
        })
    };

    let password_changed = {
        let password_state = form_state.clone();
        Callback::from(move |text: String| {
            let mut updated_state = password_state.deref().clone();
            updated_state.password = text;
            password_state.set(updated_state);
        })
    };

    let on_submit = {
        let props_on_submit = props.on_submit.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let username = form_state.username.clone();
            let password = form_state.password.clone();
            props_on_submit.emit(AccountRequest { username, password })
        })
    };

    html! {
        <form onsubmit={on_submit} class={classes!("account_form")}>
            <InputField name="username" on_change={username_changed} />
            <br />
            <InputField name="password" password=true on_change={password_changed} />
            <br />
            <Button label="Login" />
        </form>
    }

}