use std::ops::Deref;

use common::{ChatRoom, ChatRoomManageUser, ChatRoomManageUserAction, UserInfo};
use yew::prelude::*;
use yew_router::prelude::Redirect;
use yewdux::use_store;

use gloo::console::log;

use crate::{
    api_service,
    components::{
        button::Button,
        chat_message::ChatMessage,
        chat_room_preview::ChatRoomPreview,
        input_field::InputField,
        user::UserDetailComponent
    },
    router::Route,
    store::Store,
    widgets::{
        list_view::ListView, new_room_form::NewRoomForm, user_search::UserSearch
    },
};

const MSG_WINDOW_SIZE: u64 = 5;

#[derive(PartialEq, Debug, Clone)]
enum MsgSendStatus {
    Idle,
    Sending,
    Failed,
}

#[derive(PartialEq, Clone)]
enum MainPanelMode {
    Messages,
    NewRoom
}

#[derive(PartialEq, Clone)]
enum MemberPanelMode {
    ViewMembers,
    AddMembers
}

#[derive(PartialEq, Clone)]
struct State {
    chat_room_list: Vec::<ChatRoom>,
    selected_room_id: Option<u64>,
    selected_room_pos: u64,
    selected_room_name: String,
    selected_room_messages: Vec<common::ChatMessage>,
    selected_room_exhausted: bool,
    selected_room_members: Vec<UserInfo>,
    sending_status: MsgSendStatus,
    main_panel_mode: MainPanelMode,
    member_panel_mode: MemberPanelMode
}

impl Default for State {
    fn default() -> Self {
        Self {
            chat_room_list: Vec::with_capacity(0),
            selected_room_id: None,
            selected_room_pos: MSG_WINDOW_SIZE,
            selected_room_name: "".to_string(),
            selected_room_messages: Vec::with_capacity(0),
            selected_room_exhausted: false,
            selected_room_members: Vec::with_capacity(0),
            sending_status: MsgSendStatus::Idle,
            main_panel_mode: MainPanelMode::Messages,
            member_panel_mode: MemberPanelMode::ViewMembers
        }
    }
}

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

    let component_state = use_state_eq(|| State::default());

    // Retrieve chat room state
    let state_handle = component_state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(rooms) = api_service::chat_get_rooms(&token).await {
            let mut updated_state = state_handle.deref().clone();
            updated_state.chat_room_list = rooms;
            state_handle.set(updated_state);
        }
    });
    
    let state_handle = component_state.clone();
    let on_chat_room_select = {
        let state_handle = state_handle.clone();
        Callback::from(move |chat_id: u64| {
            let state_handle = state_handle.clone();
            let mut updated_state = state_handle.deref().clone();
            
            let room = state_handle.chat_room_list.iter().find(|room| room.id.eq(&chat_id)).unwrap();
            updated_state.selected_room_name = room.name.clone();
            updated_state.selected_room_id = Some(chat_id);
            updated_state.selected_room_exhausted = false;
            updated_state.main_panel_mode = MainPanelMode::Messages;
            wasm_bindgen_futures::spawn_local(async move {
                // Messages
                match api_service::chat_get_messages(&token, chat_id, 0, MSG_WINDOW_SIZE).await {
                    Ok(mut messages) => {
                        let room_starting_pos = u64::try_from(messages.len()).unwrap_or_else(|_| {
                            log!("Failed to parse retrieved message count");
                            0
                        });
                        updated_state.selected_room_pos = room_starting_pos;
                        messages.reverse();
                        updated_state.selected_room_messages = messages;
                    },
                    _ => {}
                }
                // Members
                match api_service::chat_get_members(&token, chat_id).await {
                    Ok(members) => updated_state.selected_room_members = members,
                    Err(_) => {},
                }
                state_handle.set(updated_state);
            });
        })
    };

    let state_handle = component_state.clone();
    let to_view_members = {
        Callback::from(move |_: MouseEvent| {
            let state_handle = state_handle.clone();
            let mut updated_state = state_handle.deref().clone();
            updated_state.member_panel_mode = MemberPanelMode::ViewMembers;

            wasm_bindgen_futures::spawn_local(async move {
                let chat_id = updated_state.selected_room_id.unwrap();
                match api_service::chat_get_members(&token, chat_id).await {
                    Ok(members) => updated_state.selected_room_members = members,
                    Err(_) => {},
                }
                state_handle.set(updated_state);
            });
            
        })
    };
    
    let state_handle = component_state.clone();
    let to_add_members = {
        Callback::from(move |_: MouseEvent| {
            let state_handle = state_handle.clone();
            let mut updated_state = state_handle.deref().clone();
            updated_state.member_panel_mode = MemberPanelMode::AddMembers;
            state_handle.set(updated_state);
        })
    };
    
    let state_handle = component_state.clone();
    let on_add_member = Callback::from(move |user_id_to_add: u64| {
        let room_id = state_handle.selected_room_id.unwrap();
        let manage_action = ChatRoomManageUser {
            user_id: user_id_to_add,
            action: ChatRoomManageUserAction::AddUser
        };
        
        wasm_bindgen_futures::spawn_local(async move {
            match api_service::chat_manage_user(&token, room_id, manage_action).await {
                Ok(()) => {},
                Err(e) => log!(format!("{:?}", e)),
            }
        });
        let room_id = state_handle.selected_room_id.unwrap();
        let manage_action = ChatRoomManageUser {
            user_id: user_id_to_add,
            action: ChatRoomManageUserAction::AddUser
        };
        
        wasm_bindgen_futures::spawn_local(async move {
            match api_service::chat_manage_user(&token, room_id, manage_action).await {
                Ok(()) => {},
                Err(e) => log!(format!("{:?}", e)),
            }
        });
    });
    
    let state_handle = component_state.clone();
    let on_remove_member = Callback::from(move |user_id_to_remove: u64| {
        let room_id = state_handle.selected_room_id.unwrap();
        let manage_action = ChatRoomManageUser {
            user_id: user_id_to_remove,
            action: ChatRoomManageUserAction::RemoveUser
        };
        
        let state_handle = state_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match api_service::chat_manage_user(&token, room_id, manage_action).await {
                Ok(()) => {
                    let mut updated_state = state_handle.deref().clone();
                    let index_to_remove = updated_state.selected_room_members.iter()
                        .position(|member| member.id == user_id_to_remove);
                    if let Some(idx) = index_to_remove {
                        updated_state.selected_room_members.remove(idx);
                        state_handle.set(updated_state)
                    }
                },
                Err(e) => log!(format!("{:?}", e)),
            }
        });
    });

    let state_handle = component_state.clone();
    let on_load_more_messages = {
        Callback::from(move |_: MouseEvent| {
            let state_handle = state_handle.clone();
            let mut updated_state = state_handle.deref().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let room_id = state_handle.selected_room_id.unwrap();
                let offset = state_handle.selected_room_pos;
                match api_service::chat_get_messages(&token, room_id, offset, MSG_WINDOW_SIZE).await {
                    Ok(next_messages) if next_messages.len() > 0 => {
                        // Join existing messages to newly fetched messages
                        let chained_iter = next_messages.iter()
                            .rev()
                            .map(|msg| msg.clone())
                            .chain(state_handle.selected_room_messages.iter().map(|msg| msg.clone()));

                        let new_message_list = Vec::from_iter(chained_iter);
                        updated_state.selected_room_messages = new_message_list;

                        // Adjust position of the window for next message fetch/load more messages
                        updated_state.selected_room_pos = updated_state.selected_room_pos + MSG_WINDOW_SIZE;
                    },
                    Ok(_) => {
                        updated_state.selected_room_exhausted = true;
                    }
                    Err(_) => {},
                }
                state_handle.set(updated_state);
            });
        })
    };

    let state_handle = component_state.clone();
    let input_on_submit: Callback<String> = {
        Callback::from(move |text: String| {
            // Disallow sending of message while one is already in flight.
            if MsgSendStatus::Sending.eq(&state_handle.sending_status) {
                return;
            }
            
            let state_handle = state_handle.clone();
            let mut updated_state = state_handle.deref().clone();
            let message = common::ChatMessage {
                id: None,
                room_id: state_handle.selected_room_id.unwrap(),
                sender_id: None,
                body: text,
                time_sent: None
            };

            let message_clone = message.clone();
            let room_id = message.room_id;
            wasm_bindgen_futures::spawn_local(async move {
                // Send message and update state
                match api_service::chat_send_message(&token, message_clone).await {
                    Ok(()) => updated_state.sending_status = MsgSendStatus::Idle,
                    Err(_) => updated_state.sending_status = MsgSendStatus::Failed,
                };

                // If success (back in idle state) add the send message to the local message list
                if let MsgSendStatus::Idle = updated_state.sending_status {
                    match api_service::chat_get_messages(&token, room_id, 0, MSG_WINDOW_SIZE).await {
                        Ok(messages) => {
                            updated_state.selected_room_messages = messages;
                            // reset open chat room state
                            updated_state.selected_room_pos = 0;
                            updated_state.selected_room_exhausted = false;
                        },
                        _ => log!("Failed to retrieve messages after sending one")
                    }
                }
                state_handle.set(updated_state);
            });
        })
    };
    
    let state_handle = component_state.clone();
    let on_new_room_click = {
        Callback::from(move |_: MouseEvent| {
            let mut updated_state = state_handle.deref().clone();
            updated_state.main_panel_mode = MainPanelMode::NewRoom;
            state_handle.set(updated_state);
        })
    };

    let state_handle = component_state.clone();
    let on_new_room_submit = {
        Callback::from(move |_: bool| {
            let mut updated_state = state_handle.deref().clone();
            updated_state.main_panel_mode = MainPanelMode::Messages;
            state_handle.set(updated_state);
        })
    };

    let state_handle = component_state.clone();
    let on_room_name_change = Callback::from(move |new_name: String| {
        let state_handle = state_handle.clone();
        let mut updated_state = state_handle.deref().clone();
        let room_id = updated_state.selected_room_id.unwrap();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(_) = api_service::chat_change_name(&token, room_id, &new_name).await {
                updated_state.selected_room_name = new_name;
                state_handle.set(updated_state);
            }
        });
    });

    // Generate html
    let chat_room_preview_html: Vec<Html> = component_state.chat_room_list.iter()
        .map(|room: &ChatRoom| html! {
            <ChatRoomPreview chat={room.clone()} on_select={on_chat_room_select.clone()} />
        })
        .collect();

    let chat_room_mesages_html: Vec<Html> = component_state.selected_room_messages.iter()
        .map(|message: &common::ChatMessage| html! {
            <ChatMessage message={message.clone()} />
        })
        .collect();

    let chat_room_members_html: Vec<Html> = component_state.selected_room_members.iter()
        .map(|member| {
            let user_id = member.id;
            let remove_member_callback = on_remove_member.clone();
            html! {
                <div class={classes!("user_button_row")}>
                    <UserDetailComponent data={member.clone()} />
                    <Button label={"Remove"} on_click={
                        Callback::from(move |_: MouseEvent| {
                            remove_member_callback.emit(user_id)
                        })
                    } />
                </div>
            }
        })
        .collect();

    html! {
        <>
            <h>{ "Chat rooms" }</h>
            <div class={classes!("row")}>
                <div class={classes!("chat_column", "side")}>
                    <Button label={"Create room"} on_click={Some(on_new_room_click)} />
                    <ListView children={chat_room_preview_html} />
                </div>
                <div class={classes!("chat_column", "middle")}>
                    if let MainPanelMode::Messages = component_state.main_panel_mode {
                        if component_state.selected_room_id.is_some() {
                            <InputField name="" prefill={component_state.selected_room_name.clone()}
                                on_change={on_room_name_change.clone()} />
                            if component_state.selected_room_exhausted {
                                <p>{ "No more messages" }</p>
                            } else {
                                <Button label={ "Load more" } on_click={on_load_more_messages} />
                            }
                            <ListView children={chat_room_mesages_html} />
                            <InputField name={""} on_change={input_on_submit} /> 
                        } else {
                            <p>{ "No chat selected" }</p>
                        }
                    } else {
                        <NewRoomForm notify={on_new_room_submit} />
                    }
                </div>
                <div class={classes!("chat_column", "side")}>
                    if chat_room_members_html.is_empty() {
                        <p>{ "No room selected" }</p>
                    } else {
                        if let MemberPanelMode::ViewMembers = component_state.member_panel_mode {
                            <p>{ "Room members" }</p>
                            <Button label={ "Add Members" } on_click={to_add_members} />
                            <ListView children={chat_room_members_html} />
                        } else {
                            <p>{ "Add members" }</p>
                            <Button label={ "View Members" } on_click={to_view_members} />
                            <UserSearch buttons={vec![
                                ("Add".to_string(), on_add_member.clone())
                            ]} />
                        }
                    }
                </div>
            </div>
        </>
    }
}