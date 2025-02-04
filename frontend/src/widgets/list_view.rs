use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children
}

#[function_component(ListView)]
pub fn list_view(props: &Props) -> Html {
    html! {
        <ul>
            { for props.children.clone() }
        </ul>
    }
}