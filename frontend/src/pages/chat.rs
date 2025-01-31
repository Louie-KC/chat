use common::ChatRoom;
use yew::prelude::*;
use yew_router::prelude::Redirect;
use yewdux::use_store;

use crate::{api_service, components::{atoms::{button::Button, chat_message::ChatMessage, chat_room_preview::ChatRoomPreview}, molecules::list_view::ListView}, router::Route, store::Store};

const MESSAGE_WINDOW_SIZE: u64 = 50;

#[function_component(ChatPage)]
pub fn chat_page() -> Html {
    // Global state
    let (store, _) = use_store::<Store>();

    // Redirect to Home is not logged in
    if store.user.is_none() {
        return html! {
            <Redirect<Route> to={Route::Home} />
        }
    }

    let token = store.user.clone().unwrap().token.clone();

    // Component state
    let chat_room_list = use_state_eq(|| Vec::<ChatRoom>::new());
    let selected_room_id = use_state_eq(|| Option::<u64>::None);
    let selected_room_pos = use_state_eq(|| MESSAGE_WINDOW_SIZE);
    let selected_room_messages = use_state_eq(|| Vec::<common::ChatMessage>::new());
    let selected_room_exhausted = use_state_eq(|| false);

    // Retrieve chat room state
    let chat_room_list_handle = chat_room_list.clone();
    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(rooms) = api_service::chat_get_rooms(&token).await {
            chat_room_list_handle.set(rooms);
        }
    });

    let selected_room_messages_handle = selected_room_messages.clone();
    let on_chat_room_select = {
        let selected_room_id_handle = selected_room_id.clone();
        let selected_room_pos_handle = selected_room_pos.clone();
        let selected_room_exhausted_handle = selected_room_exhausted.clone();
        Callback::from(move |chat_id: u64| {
            selected_room_id_handle.set(Some(chat_id));
            selected_room_pos_handle.set(0);
            selected_room_exhausted_handle.set(false);
            
            let selected_room_messages_handle = selected_room_messages_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api_service::chat_get_messages(&token, chat_id, 0, MESSAGE_WINDOW_SIZE).await {
                    Ok(messages) => selected_room_messages_handle.set(messages),
                    _ => {}
                }
            });
        })
    };

    let selected_room_messages_handle = selected_room_messages.clone();
    let selected_room_id_handle = selected_room_id.clone();
    let selected_room_exhausted_handle = selected_room_exhausted.clone();
    let on_load_more_messages = {
        let selected_room_id_handle = selected_room_id_handle.clone();
        Callback::from(move |_: MouseEvent| {
            let selected_room_id_handle = selected_room_id_handle.clone();
            let selected_room_pos_handle = selected_room_pos.clone();
            let selected_room_messages_handle = selected_room_messages_handle.clone();
            let selected_room_exhausted_handle = selected_room_exhausted_handle.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let room_id = selected_room_id_handle.unwrap();
                let offset = *selected_room_pos_handle;
                match api_service::chat_get_messages(&token, room_id, offset, MESSAGE_WINDOW_SIZE).await {
                    Ok(next_messages) if next_messages.len() > 0 => {
                        // Join existing messages to newly fetched messages
                        let chained_iter = selected_room_messages_handle.iter()
                            .map(|old_msg| old_msg.clone())
                            .chain(next_messages.iter().map(|new_msg| new_msg.clone()));

                        let new_message_list = Vec::from_iter(chained_iter);
                        selected_room_messages_handle.set(new_message_list);

                        // Adjust position of the window for next message fetch/load more messages
                        selected_room_pos_handle.set(*selected_room_pos_handle + MESSAGE_WINDOW_SIZE);
                    },
                    Ok(_) => {
                        selected_room_exhausted_handle.set(true);
                    }
                    Err(_) => {},
                }
            });
        })
    };

    let chat_room_preview_html: Vec<Html> = chat_room_list.iter()
        .map(|room: &ChatRoom| html! {
            <ChatRoomPreview chat={room.clone()} on_select={on_chat_room_select.clone()} />
        })
        .collect();

    let chat_room_mesages_html: Vec<Html> = selected_room_messages.iter()
        .map(|message: &common::ChatMessage| html! {
            <ChatMessage message={message.clone()} />
        })
        .collect();

    html! {
        <>
            <h>{ "Chat rooms" }</h>
            <div>
                <ListView children={chat_room_preview_html} />
            </div>
            <div>
                if selected_room_id.is_some() {
                    if *selected_room_exhausted {
                        <p>{ "No more messages" }</p>
                    } else {
                        <Button label={ "Load more" } on_click={on_load_more_messages} />
                    }
                    <ListView children={chat_room_mesages_html} />
                } else {
                    <p>{ "No chat selected" }</p>
                }
            </div>
        </>
    }
}