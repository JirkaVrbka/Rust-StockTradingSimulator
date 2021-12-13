#![recursion_limit = "1024"]
use anyhow::Error;
use yew::{
    format::Json,
    prelude::*,
    services::{ConsoleService, websocket::{WebSocketService, WebSocketStatus, WebSocketTask}}
};
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::{
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Style},
    button::Button,
};
use yew_styles::layouts::{
    container::{AlignItems, Container, Direction, Mode, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::Size;
use yew_styles::text::{Header, Text, TextType};
extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

pub struct Home {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    text: String,
    server_data: String
}

pub enum HomeMsg {
    Connect,
    Disconnected,
    Ignore,
    TextInput(String),
    SendText,
    Received(Result<String, Error>),
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws: None,
            link: link,
            text: String::new(),
            server_data: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            HomeMsg::Connect => {
                ConsoleService::log("Connecting");
                let cbout = self.link.callback(|data| HomeMsg::Received(data));
                let cbnot = self.link.callback(|input| {
                    ConsoleService::log(&format!("Notification: {:?}", input));
                    match input {
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            HomeMsg::Disconnected
                        }
                        _ => HomeMsg::Ignore,
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
            HomeMsg::Disconnected => {
                self.ws = None;
                true
            }
            HomeMsg::Ignore => {
                false
            }
            HomeMsg::TextInput(e) => {
                self.text = e;
                true
            }
            HomeMsg::SendText => {
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
            HomeMsg::Received(Ok(s)) => {
                self.server_data.push_str(&format!("{}\n", &s));
                true
            }
            HomeMsg::Received(Err(s)) => {
                self.server_data.push_str(&format!("Error when reading from server: {}\n", &s.to_string()));
                true
            }
        }
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // connect button
                <p><button onclick=self.link.callback(|_| HomeMsg::Connect)>{ "Connect" }</button></p><br/>
                // text showing whether we're connected or not
                <p>{ "Connected: "}{ !self.ws.is_none() } </p><br/>
                // input box for sending text
                <p><input type="text" value=self.text.clone() oninput=self.link.callback(|e: InputData| HomeMsg::TextInput(e.value))/></p><br/>
                // button for sending text
                <p><button onclick=self.link.callback(|_| HomeMsg::SendText)>{ "Send" }</button></p><br/>
                // text area for showing data from the server
                <p><textarea value=self.server_data.clone()></textarea></p><br/>
            </div>
        }
    }
}
