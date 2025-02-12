use std::ops::Deref;

use common::{UserAssociationUpdate, UserAssociations};
use yew::prelude::*;
use yew_router::prelude::Redirect;
use yewdux::use_store;

use crate::{
    api_service,
    components::{
        button::Button,
        user::UserDetailComponent
    },
    router::Route,
    store::Store,
    widgets::user_search::UserSearch
};

#[derive(PartialEq, Clone)]
struct State {
    associations: UserAssociations
}

impl Default for State {
    fn default() -> Self {
        Self { associations: UserAssociations {
                friends: Vec::with_capacity(0),
                incoming_requests: Vec::with_capacity(0),
                unaccepted_requests: Vec::with_capacity(0),
                blocked: Vec::with_capacity(0)
            }
        }
    }
}

#[function_component(AssociationsPage)]
pub fn associations_page() -> Html {
    let (store, _) = use_store::<Store>();

    if store.user.is_none() {
        return html! {
            <Redirect<Route> to={Route::Home}/>
        }
    }
    let token = store.user.clone().unwrap().token;

    let component_state = use_state_eq(|| State::default());

    let state_handle = component_state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(associations) = api_service::user_get_associations(&token).await {
            let mut updated_state = state_handle.deref().clone();
            updated_state.associations = associations;
            state_handle.set(updated_state);
        }
    });

    let state_handle = component_state.clone();
    let on_add_friend_association = Callback::from(move |user_id: u64| {
        let friend_association = UserAssociationUpdate {
            other_user_id: user_id,
            association_type: common::UserAssociationType::Friend
        };
        let state_handle = state_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(_) = api_service::user_associate(&token, friend_association).await {
                if let Ok(associations) = api_service::user_get_associations(&token).await {
                    let mut updated_state = state_handle.deref().clone();
                    updated_state.associations = associations;
                    state_handle.set(updated_state);
                }
            }
        });
    });

    let state_handle = component_state.clone();
    let on_block_association = Callback::from(move |user_id: u64| {
        let block_association = UserAssociationUpdate {
            other_user_id: user_id,
            association_type: common::UserAssociationType::Block
        };
        let state_handle = state_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(_) = api_service::user_associate(&token, block_association).await {
                if let Ok(associations) = api_service::user_get_associations(&token).await {
                    let mut updated_state = state_handle.deref().clone();
                    updated_state.associations = associations;
                    state_handle.set(updated_state);
                }
            }
        });
    });
    
    let state_handle = component_state.clone();
    let on_remove_association = Callback::from(move |user_id: u64| {
        let delete_association = UserAssociationUpdate {
            other_user_id: user_id,
            association_type: common::UserAssociationType::Remove
        };
        let state_handle = state_handle.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(_) = api_service::user_associate(&token, delete_association).await {
                if let Ok(associations) = api_service::user_get_associations(&token).await {
                    let mut updated_state = state_handle.deref().clone();
                    updated_state.associations = associations;
                    state_handle.set(updated_state);
                }
            }
        });
    });

    let search_list_buttons = vec![
        ("Add friend".to_string(), on_add_friend_association.clone()),
        ("Block".to_string(), on_block_association.clone())
    ];

    html! {
        <>
            <div>
                <h>{ "Friend Requests" }</h>
                if component_state.associations.incoming_requests.is_empty() {
                    <p>{ "None" }</p>
                } else {
                    <ul>
                    {
                        component_state.associations.incoming_requests.iter()
                            .map(|user| {
                                let user_id = user.id;
                                let accept_callback = on_add_friend_association.clone();
                                let block_callback = on_block_association.clone();
                                html! {
                                    <div class={classes!("user_button_row")}>
                                        <UserDetailComponent data={user.clone()} />
                                        <Button label={"Accept"} on_click={
                                            Callback::from(move |_: MouseEvent| {
                                                accept_callback.emit(user_id)
                                            })
                                        } />
                                        <Button label={"Block"} on_click={
                                            Callback::from(move |_: MouseEvent| {
                                                block_callback.emit(user_id)
                                            })
                                        } />
                                    </div>
                                }
                            })
                            .collect::<Html>()
                    }
                    </ul>
                }
            </div>
            <div>
                <h>{ "Friends" }</h>
                if component_state.associations.friends.is_empty() {
                    <p>{ "None" }</p>
                } else {
                    <ul>
                    {
                        component_state.associations.friends.iter()
                        .map(|user| {
                            let user_id = user.id;
                            let remove_callback = on_remove_association.clone();
                            let block_callback = on_block_association.clone();
                            html! {
                                <div class={classes!("user_button_row")}>
                                    <UserDetailComponent data={user.clone()} />
                                    <Button label={"Remove"} on_click={
                                        Callback::from(move |_: MouseEvent| {
                                            remove_callback.emit(user_id)
                                        })
                                    } />
                                    <Button label={"Block"} on_click={
                                        Callback::from(move |_: MouseEvent| {
                                            block_callback.emit(user_id)
                                        })
                                    } />
                                </div>
                            }
                        })
                        .collect::<Html>()
                    }
                    </ul>
                }
                </div>
                <div>
                <h>{ "Awaiting Response" }</h>
                if component_state.associations.unaccepted_requests.is_empty() {
                    <p>{ "None" }</p>
                } else {
                    <ul>
                    {
                        component_state.associations.unaccepted_requests.iter()
                        .map(|user| {
                            let user_id = user.id;
                            let remove_callback = on_remove_association.clone();
                            html! {
                                <div class={classes!("user_button_row")}>
                                    <UserDetailComponent data={user.clone()} />
                                    <Button label={"Remove"} on_click={
                                        Callback::from(move |_: MouseEvent| {
                                            remove_callback.emit(user_id)
                                        })
                                    } />
                                </div>
                            }
                        })
                        .collect::<Html>()
                    }
                    </ul>
                }
                </div>
                <div>
                <h>{ "Blocked by you" }</h>
                if component_state.associations.blocked.is_empty() {
                    <p>{ "None" }</p>
                } else {
                    <ul>
                    {
                        component_state.associations.blocked.iter()
                            .map(|user| {
                                let user_id = user.id;
                                let remove_callback = on_remove_association.clone();
                                html! {
                                    <div class={classes!("user_button_row")}>
                                        <UserDetailComponent data={user.clone()} />
                                        <Button label={"Remove"} on_click={
                                            Callback::from(move |_: MouseEvent| {
                                                remove_callback.emit(user_id)
                                            })
                                        } />
                                    </div>
                                }
                            })
                            .collect::<Html>()
                    }
                    </ul>
                }
            </div>
            <hr />
            <div>
                <h>{ "Search users" }</h>
                <UserSearch buttons={search_list_buttons} />
            </div>
        </>
    }
}