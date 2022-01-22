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
                        self.text = String::new();
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
                <div class="container mt-3">
                    <textarea class="form-control" value=self.server_data.clone() rows="20" cols="80" readonly=true disabled=true/>
                    <div class="input-group mb-3">
                        <input class="form-control" type="text" value=self.text.clone() oninput=self.link.callback(|e: InputData| ChatMsg::TextInput(e.value))/>
                        <div class="input-group-append">
                            <button class="btn btn-primary" onclick=self.link.callback(|_| ChatMsg::SendText)>{ "Send" }</button>
                        </div>
                    </div>
                    <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#myModal">
                        {{"Open modal"}}
                    </button>
                </div>

                <div class="modal" id="myModal">
                    <div class="modal-dialog">
                        <div class="modal-content">

                        <div class="modal-header">
                            <h4 class="modal-title">{{"Modal Heading"}}</h4>
                            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                        </div>

                        <div class="modal-body">
                            {{"Modal body.."}}
                        </div>

                        <div class="modal-footer">
                            <button type="button" class="btn btn-danger" data-bs-dismiss="modal">{{"Close"}}</button>
                        </div>

                        </div>
                    </div>
                </div>
            </div>

        }
    }
}