use utils::json::NewsJSON;
use yew::prelude::*;
use crate::fetcher::{ToHtml, NoProps};
use crate::fetcher::immediate_fetcher::ImmediateFetcher;
use serde::{Deserialize, Serialize};
use crate::fetcher::button_login::ButtonLogin;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Credentials {
    name: String,
    password: String
}

pub struct Login;

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Login {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let creds = Credentials{
            name: "Loretta Lindsey".to_string(),
            password: "X@C3l".to_string()
        };
        html! {
            <ButtonLogin::<Credentials> port="login" data=creds text="Login as Loretta"/>
        }
    }
}

