use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    #[prop_or(false)]
    pub autofocus: bool,
    #[prop_or(false)]
    pub password: bool,
    pub on_change: Callback<String>
}

#[function_component(InputField)]
pub fn input_field(props: &Props) -> Html {
    let input_type = match props.password {
        true  => "password",
        false => "text"
    };

    let on_change = props.on_change.clone();
    let internal_on_change = Callback::from(move |event: Event| {
        let value = match event.target() {
            Some(target) => target.unchecked_into::<HtmlInputElement>().value(),
            None => "".to_string()
        };
        on_change.emit(value)
    });

    html! {
        <input type={input_type} autofocus={props.autofocus}
            name={props.name.clone()} placeholder={props.name.clone()}
            onchange={internal_on_change} />
    }
}
