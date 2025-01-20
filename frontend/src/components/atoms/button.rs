use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        // do not specify button type. Blocks onsubmit if part of a form.
        <button> {props.label.clone()} </button>
    }
}