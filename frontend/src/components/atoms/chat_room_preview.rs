use common::ChatRoom;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub chat: ChatRoom,
    #[prop_or("...".to_string())]
    pub last_msg_preview: String,
    pub on_select: Callback<u64>
}

#[function_component(ChatRoomPreview)]
pub fn chat_room_preview(props: &Props) -> Html {
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
            <p>{ props.last_msg_preview.clone() }</p>
        </div>
    }
}