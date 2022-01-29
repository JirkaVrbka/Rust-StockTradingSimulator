use utils::json::NewsJSON;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew_styles::layouts::item::ItemLayout;
use yew_styles::styles::{Size, Palette};
use crate::fetcher::{ToHtml, NoProps};
use crate::fetcher::immediate_fetcher::ImmediateFetcher;
use serde::{Deserialize, Serialize};
use crate::fetcher::button_login::ButtonLogin;
use yew_styles::forms::form_input::{FormInput, InputType};
use yew_styles::layouts::{item::Item, container::{Container, Direction, Wrap}};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Credentials {
    name: String,
    password: String
}

pub enum LoginMsg {
    name(String),
    password(String)
}

pub struct Login {
    link: ComponentLink<Self>,
    credentials: Credentials
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            link,
            credentials: Credentials {
                name: String::new(),
                password: String::new()
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LoginMsg::name(name) => self.credentials.name = name,
            LoginMsg::password(password) => self.credentials.password = password,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(1))>
                    <b> {"LOGIN"} </b>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(1))>
                    <FormInput
                        input_type=InputType::Text
                        input_palette=Palette::Standard
                        input_size=Size::Medium
                        oninput_signal = self.link.callback(|e: InputData| LoginMsg::name(e.value))
                        placeholder="name"
                    />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(1))>
                    <FormInput
                        input_type=InputType::Text
                        input_palette=Palette::Standard
                        input_size=Size::Medium
                        id="form-input-test"
                        oninput_signal = self.link.callback(|e: InputData| LoginMsg::password(e.value))
                        placeholder="password"
                    />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(1))>
                    <ButtonLogin::<Credentials> port="login" data=self.credentials.clone() text="Login"/>
                </Item>
            </Container>
        }
    }
}

