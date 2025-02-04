use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let on_click_callback = {
        let props_on_click = props.on_click.clone();
        Callback::from(move |event: MouseEvent| {
            if let Some(callback) = props_on_click.clone() {
                callback.emit(event);
            }
        })
    };

    html! {
        // do not specify button type. Blocks onsubmit if part of a form.
        <button onclick={on_click_callback} class={classes!("button")}> {props.label.clone()} </button>
    }
}