use std::ops::Deref;

use common::{UserAssociationUpdate, UserAssociations};
use yew::prelude::*;
use yew_router::prelude::Redirect;
use yewdux::use_store;

use crate::{
    api_service,
    components::user::UserDetailComponent,
    router::Route,
    store::Store
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
    let on_remove_association = Callback::from(move |friend_user_id: u64| {
        let delete_association = UserAssociationUpdate {
            other_user_id: friend_user_id,
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
                            .map(|user| html! {
                                <UserDetailComponent data={user.clone()} />
                            })
                            .collect::<Html>()
                    }
                    </ul>
                }
            </div>
            <div>
                <h>{ "Friends: Click to remove" }</h>
                <ul>
                {
                    component_state.associations.friends.iter()
                    .map(|user| html! {
                        <UserDetailComponent data={user.clone()}
                            on_select={Some(on_remove_association.clone())} />
                    })
                    .collect::<Html>()
                }
                </ul>
            </div>
            <div>
                <h>{ "Awaiting Response: Click to remove" }</h>
                <ul>
                {
                    component_state.associations.unaccepted_requests.iter()
                    .map(|user| html! {
                        <UserDetailComponent data={user.clone()}
                            on_select={Some(on_remove_association.clone())} />
                    })
                    .collect::<Html>()
                }
                </ul>
            </div>
            <div>
                <h>{ "Blocked by you: Click to remove" }</h>
                <ul>
                {
                    component_state.associations.blocked.iter()
                        .map(|user| html! {
                            <UserDetailComponent data={user.clone()}
                                on_select={Some(on_remove_association.clone())} />
                        })
                        .collect::<Html>()
                }
                </ul>
            </div>
        </>
    }
}