use yew::prelude::*;
use yewdux::use_store;

use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: common::ChatMessage,
}

#[function_component(ChatMessage)]
pub fn chat_message(props: &Props) -> Html {
    let (store, _) = use_store::<Store>();

    let sender_name = match props.message.sender_id {
        Some(id) => store.cache.get_username_from_id(id),
        None => "Unknown user".to_string()
    };

    let time_sent = match props.message.time_sent {
        Some(time) => time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        None => "Unknown".to_string(),
    };

    html! {
        <div class={classes!("message_container")}>
            <p>{"sender: "}{ sender_name }</p>
            <p>{"body: "}{ props.message.body.clone() }</p>
            <p>{"time sent: "}{ time_sent }</p>
        </div>
    }
}