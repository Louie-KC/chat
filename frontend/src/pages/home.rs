use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let (store, _) = use_store::<Store>();

    html! {
        <div>
            <h1>{ "home page" }</h1>
            <h2>{ "placeholder text" }</h2>
            if store.user.is_some() {
                <p>{ "Logged in as user: "}{ store.user.clone().unwrap().username.clone() }</p>
            } else {
                <p>{ "Not logged in" }</p>
            }
        </div>
    }
}