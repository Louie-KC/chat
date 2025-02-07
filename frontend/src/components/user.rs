use common::UserInfo;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub data: UserInfo,
    #[prop_or_default]
    pub on_select: Option<Callback<u64>>
}

#[function_component(UserDetailComponent)]
pub fn user_widget(props: &Props) -> Html {
    let on_select = {
        let user_id = props.data.id;
        let on_select = props.on_select.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(callback) = on_select.clone() {
                callback.emit(user_id)
            }
        })
    };

    html! {
        <div onclick={on_select} class={classes!("chat", "member")}>
            <p>{ props.data.username.clone() }</p>
        </div>
    }
}