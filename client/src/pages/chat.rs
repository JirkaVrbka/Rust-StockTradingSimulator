use anyhow::Error;
use yew::{
    format::Json,
    prelude::*,
    services::{ConsoleService, websocket::{WebSocketService, WebSocketStatus, WebSocketTask}}
};
extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

pub struct Chat {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    text: String,
    server_data: String
}

pub enum ChatMsg {
    Connect,
    Disconnected,
    Ignore,
    TextInput(String),
    SendText,
    Received(Result<String, Error>),
}

impl Component for Chat {
    type Message = ChatMsg;
    type Properties = ();


    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::log("Connecting");
        let cbout = link.callback(|data| ChatMsg::Received(data));
        let cbnot = link.callback(|input| {
            ConsoleService::log(&format!("Notification: {:?}", input));
            match input {
                WebSocketStatus::Closed | WebSocketStatus::Error => {
                    ChatMsg::Disconnected
                }
                _ => ChatMsg::Ignore,
            }
        });
        Self {
            ws: WebSocketService::connect_text("ws://127.0.0.1:8081/c05554ae-b4ee-4976-ac05-97aaf3c98a23", cbout, cbnot).ok(),
            link: link,
            text: String::new(),
            server_data: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ChatMsg::Connect => {
                true
            }
            ChatMsg::Disconnected => {
                self.ws = None;
                true
            }
            ChatMsg::Ignore => {
                false
            }
            ChatMsg::TextInput(e) => {
                self.text = e;
                true
            }
            ChatMsg::SendText => {
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
            ChatMsg::Received(Ok(s)) => {
                self.server_data.push_str(&format!("{}\n", &s));
                true
            }
            ChatMsg::Received(Err(s)) => {
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
                // text area for showing data from the server
                <p><textarea value=self.server_data.clone()></textarea></p><br/>
                // input box for sending text
                <p><input type="text" value=self.text.clone() oninput=self.link.callback(|e: InputData| ChatMsg::TextInput(e.value))/></p><br/>
                // button for sending text
                <p><button onclick=self.link.callback(|_| ChatMsg::SendText)>{ "Send" }</button></p><br/>
            </div>
        }
    }
}