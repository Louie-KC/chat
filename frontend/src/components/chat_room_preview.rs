use common::ChatRoom;
use yew::prelude::*;
use yewdux::use_store;

use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub chat: ChatRoom,
    pub on_select: Callback<u64>
}

#[function_component(ChatRoomPreview)]
pub fn chat_room_preview(props: &Props) -> Html {
    let (store, _) = use_store::<Store>();

    let last_message = store.cache.get_room_previous_msg_from_id(props.chat.id);
    let last_message_preview = match last_message.len() {
        n if n > 10 => format!("{}...", &last_message[0..7]),
        _ => last_message
    };

    let on_click = {
        let chat_id = props.chat.id;
        let on_select = props.on_select.clone();
        Callback::from(move |_: MouseEvent| {
            on_select.emit(chat_id);
        })
    };

    html! {
        <div onclick={on_click} class={classes!("message_container")}>
            <p>{ props.chat.name.clone() }</p>
            <p>{ last_message_preview }</p>
        </div>
    }
}