use chrono::SecondsFormat;
use common::LoginTokenInfo;
use yew::prelude::*;


#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub info: LoginTokenInfo
}

#[function_component(TokenInfo)]
pub fn token_info(props: &Props) -> Html {
    let is_requester = match props.info.is_requester {
        true  => "Yes",
        false => "No",
    };
    html! {
        <div>
            <p>{ "Device name: "}{props.info.user_agent.clone()}</p>
            <p>{ "Initial login: "}{props.info.time_set.to_rfc3339_opts(SecondsFormat::Secs, true)}</p>
            <p>{ "This device: "}{is_requester}</p>
        </div>
    }
}