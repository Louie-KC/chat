use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: common::ChatMessage
}

#[function_component(ChatMessage)]
pub fn chat_message(props: &Props) -> Html {
    // placeholder widget/component

    let sender_id = match props.message.sender_id {
        Some(id) => format!("{id}"),
        None => "Unknown".to_string(),
    };

    let time_sent = match props.message.time_sent {
        Some(time) => time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        None => "Unkown".to_string(),
    };

    html! {
        <div>
            <p>{"sender id: "}{ sender_id }</p>
            <p>{"body: "}{ props.message.body.clone() }</p>
            <p>{"time sent: "}{ time_sent }</p>
        </div>
    }
}