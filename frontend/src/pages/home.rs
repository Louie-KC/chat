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
                <HomePageLoggedIn />
            } else {
                <HomePageLoggedOut />
            }
        </div>
    }
}

#[function_component(HomePageLoggedIn)]
fn home_page_logged_in() -> Html {
    let (store, _) = use_store::<Store>();
    html! {
        <p>{ "Logged in as user: "}{ store.user.clone().unwrap().username.clone() }</p>
    }
    
}
    
#[function_component(HomePageLoggedOut)]
fn home_page_logged_out() -> Html {
        
    html! {
        <p>{ "Not logged in" }</p>
    }
}