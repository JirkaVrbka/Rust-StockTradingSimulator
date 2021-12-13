#![recursion_limit = "1024"]
mod fetcher;

use anyhow::Error;
use yew::{
    format::Json,
    prelude::*,
    services::{ConsoleService, websocket::{WebSocketService, WebSocketStatus, WebSocketTask}}
};
use crate::pages::{About, Home};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::{
    styles::{Style},
    button::Button,
};
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::styles::Size;
use yew_styles::text::{Text, TextType};
extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

mod pages;

struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    text: String,
    server_data: String,
    navbar_items: Vec<bool>,
}

enum Msg {
    Connect,
    Disconnected,
    Ignore,
    TextInput(String),
    SendText,
    Received(Result<String, Error>),
    ChangeNavbarItem(usize),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/about!"]
    AboutPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws: None,
            navbar_items: vec![true, false],
            link: link,
            text: String::new(),
            server_data: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Connect => {
                ConsoleService::log("Connecting");
                let cbout = self.link.callback(|data| Msg::Received(data));
                let cbnot = self.link.callback(|input| {
                    ConsoleService::log(&format!("Notification: {:?}", input));
                    match input {
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Msg::Disconnected
                        }
                        _ => Msg::Ignore,
                    }
                });
                if self.ws.is_none() {
                    // this will connect us to the default chat lobby, 
                    // later we can replace this to private lobby 
                    // or graph lobby where chat messages will be buy and sell commands
                    let task = WebSocketService::connect_text("ws://127.0.0.1:8081/c05554ae-b4ee-4976-ac05-97aaf3c98a23", cbout, cbnot);
                    self.ws = Some(task.unwrap());
                }
                true
            }
            Msg::Disconnected => {
                self.ws = None;
                true
            }
            Msg::Ignore => {
                false
            }
            Msg::TextInput(e) => {
                self.text = e;
                true
            }
            Msg::SendText => {
                match self.ws {
                    Some(ref mut task) => {
                        task.send(Json(&self.text));
                        true
                    }
                    None => {
                        false
                    }
                }
            }
            Msg::Received(Ok(s)) => {
                self.server_data.push_str(&format!("{}\n", &s));
                true
            }
            Msg::Received(Err(s)) => {
                self.server_data.push_str(&format!("Error when reading from server: {}\n", &s.to_string()));
                true
            }
            Msg::ChangeNavbarItem(index) => {
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }
                self.navbar_items[index] = true;
                true
            }
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Text
                    class_name="title"
                    text_type=TextType::Plain
                    plain_text="STONKER$"
                    text_size=Size::Big
                    html_text=None
                />
                <Container direction=Direction::Row wrap=Wrap::Wrap>
                    <Container direction=Direction::Column wrap=Wrap::Wrap>
                        <RouterAnchor<AppRouter>route=AppRouter::RootPath>
                            <Button
                                class_name="navbar-route"
                                onclick_signal = self.link.callback(|_| Msg::ChangeNavbarItem(0))>
                                {"Home"}
                            </Button>
                        </RouterAnchor<AppRouter>>
                        <RouterAnchor<AppRouter>route=AppRouter::AboutPath>
                            <Button 
                                class_name="navbar-route"
                                button_style=Style::Regular
                                onclick_signal = self.link.callback(|_| Msg::ChangeNavbarItem(1))>
                                {"About"}
                            </Button>
                        </RouterAnchor<AppRouter>>
                    </Container>
                    <Router<AppRouter, ()>
                        render = Router::render(|switch: AppRouter | {
                            match switch {
                                AppRouter::RootPath => html!{
                                    <Home/>
                                },
                                AppRouter::AboutPath => html!{
                                    <About/>
                                },
                                AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        } )
                        redirect = Router::redirect(|route: Route<()>| {
                            AppRouter::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </Container>
            </Container>
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}

