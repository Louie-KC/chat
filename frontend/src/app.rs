use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::BrowserRouter;

use crate::components::nav_bar::NavBar;
use crate::router::{self, Route};

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <div>
                <NavBar />
            </div>
            <div>
                <Switch<Route> render={router::switch}/>
            </div>
        </BrowserRouter>
    }
}
