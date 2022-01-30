use utils::json::AuthJSON;
use yew::{
    prelude::*,
    services::{fetch::{FetchService, FetchTask, Request, Response, Credentials as ServiceCredentials, FetchOptions}, ConsoleService}, format::{Json, Text}, virtual_dom::VNode,
};
use yew_styles::{spinner::{Spinner, SpinnerType}, layouts::{item::ItemLayout, container::Direction}};
use yew_styles::styles::{Palette, Size};
use yew_styles::button::Button;
use super::Logged;
use yew_styles::layouts::{container::Container, item::Item};
use yew_styles::forms::form_input::FormInput;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use yew_styles::forms::form_input::{InputType};
use yew_styles::layouts::{container::Wrap};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Credentials {
    name: String,
    password: String
}

#[derive(Debug)]
pub enum LoginMsg {
    Post,
    ReceiveResponse(Result<AuthJSON, anyhow::Error>),
    name(String),
    password(String)
}

#[derive(Debug)]
pub enum LoginProcess {
    Err(anyhow::Error),
    Ok,
    Wait,
    Posting(FetchTask),
}

#[derive(Debug)]
pub struct Login {
    post: LoginProcess,
    link: ComponentLink<Self>,
    credentials: Credentials
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            post: LoginProcess::Wait,
            credentials: Credentials {
                name: String::new(),
                password: String::new()
            },
            link,
        }
    }
    fn change(&mut self, props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::name(name) => self.credentials.name = name,
            LoginMsg::password(password) => self.credentials.password = password,
            LoginMsg::Post => {
                let val = serde_json::to_value(self.credentials.clone()).unwrap();
                let request = Request::post(format!("http://localhost:8081/login").as_str())
                    .header("Content-Type", "application/json")
                    .body(Json(&val))
                    .expect("Could not build request.");
                let options = FetchOptions {
                    credentials: Some(ServiceCredentials::Include),
                    ..FetchOptions::default()
                };
                let callback = self.link
                    .callback(|response: Response<Json<Result<AuthJSON, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        LoginMsg::ReceiveResponse(data)
                    });
                self.post = LoginProcess::Posting(FetchService::fetch_with_options(request, options, callback).expect("failed to start post"));
            },
            LoginMsg::ReceiveResponse(response) => {
                match response {
                    Ok(AuthJSON::Ok) => {
                        self.post = LoginProcess::Ok;
                    },
                    Ok(err) => {
                        self.post = LoginProcess::Err(anyhow::anyhow!("{:?}", err));
                    },
                    Err(error) => {
                        self.post = LoginProcess::Err(error);
                    }
                }
            }
        }
        true
    }
    fn view(&self) -> Html {
        let show_form = |html: VNode| html!{
            <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(1))>
                    <div class="fw-bold fs-1">
                    <b> {"LOGIN"} </b>
                    </div>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(1))>
                    <FormInput
                        input_type=InputType::Text
                        input_palette=Palette::Standard
                        input_size=Size::Medium
                        oninput_signal = self.link.callback(|e: InputData| LoginMsg::name(e.value))
                        placeholder="name"
                        class_name="fs-2 border-0 border-bottom my-2"
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
                        class_name="fs-2 border-0 border-bottom my-2"
                    />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(1))>
                    { html }
                </Item>
            </Container>
        };
        match &self.post {
            LoginProcess::Wait => show_form(html! {
                <Button onclick_signal=self.link.callback(|_| LoginMsg::Post) class_name="mt-3 btn btn-info fs-2 text-white">
                    { "Login" }
                </Button>
            }),
            LoginProcess::Ok => html!{
                <Logged/>
            },
            LoginProcess::Err(error) => show_form(html! {
                <div>
                    <p style="color:red">{ error.to_string().clone() }</p>
                    <Button onclick_signal=self.link.callback(|_| LoginMsg::Post) class_name="mt-2 btn btn-info fs-2 text-white">
                        { "Login" }
                    </Button>
                </div>
            }),
            LoginProcess::Posting(_) => show_form(html! {
                <Spinner
                    spinner_type=SpinnerType::Circle
                    spinner_size=Size::Medium
                    spinner_palette=Palette::Standard
                />
            })
        }
    }
}