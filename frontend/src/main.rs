mod app;
mod api_service;
mod components;
mod widgets;
mod pages;
mod router;
mod store;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
