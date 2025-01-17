mod api_service;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{ "Chat Front End (Yew)" }</h1>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
