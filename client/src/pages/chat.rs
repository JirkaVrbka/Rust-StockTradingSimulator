use anyhow::Error;
use uuid::Uuid;
use yew::{
    format::Json,
    prelude::*,
    services::{ConsoleService, websocket::{WebSocketService, WebSocketStatus, WebSocketTask}}
};

use crate::cookie;
extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

pub struct Chat {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    text: String,
    change_room: String,
    server_data: String
}

pub enum ChatMsg {
    Connect,
    Disconnected,
    Ignore,
    TextInput(String),
    SendText,
    SwitchRoom,
    CreateRoom,
    RoomInput(String),
    Received(Result<String, Error>),
}

impl Chat {
    fn join(&mut self) {
        ConsoleService::log("Connecting");
        let cbout = self.link.callback(|data| ChatMsg::Received(data));
        let conto = self.link.callback(|input| {
            ConsoleService::log(&format!("Notification: {:?}", input));
            match input {
                WebSocketStatus::Closed | WebSocketStatus::Error => {
                    ChatMsg::Disconnected
                }
                _ => ChatMsg::Ignore,
            }
        });
        self.ws = match cookie::get_login() {
            Ok(login) => WebSocketService::connect_text(
                    format!("ws://127.0.0.1:8081/chat/{}/{}", self.change_room, login).as_str(),
                    cbout,
                    conto
                ).ok(),
            Err(e) => { ConsoleService::log(format!("{:?}", e).as_str()); None}
        };
    }
}

fn get_global_lobby() -> String {
    "c05554ae-b4ee-4976-ac05-97aaf3c98a23".to_string()
}

fn get_custom_lobby() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

impl Component for Chat {
    type Message = ChatMsg;
    type Properties = ();


    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut me = Self {
            ws: None,
            link: link,
            text: String::new(),
            change_room: get_global_lobby(),
            server_data: String::new(),
        };
        me.join();
        me
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
            ChatMsg::RoomInput(e) => {
                self.change_room = e;
                false
            }
            ChatMsg::SwitchRoom => {
                self.join();
                true
            }
            ChatMsg::CreateRoom => {
                self.change_room = get_custom_lobby();
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
                    <button type="button" class="btn btn-primary me-3" data-bs-toggle="modal" onclick=self.link.callback(|_| ChatMsg::CreateRoom) data-bs-target="#createRoomModal">
                        {{"Create room"}}
                    </button>
                    <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#joinRoomModal">
                        {{"Join room"}}
                    </button>
                </div>

                <div class="modal" id="joinRoomModal">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h4 class="modal-title">{{"Join room"}}</h4>
                                <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                            </div>

                            <div class="modal-body">
                                <label>{{"Room Code (ask other stonkers to give you one)"}}</label>
                                <input class="form-control" placeholder=get_global_lobby() oninput=self.link.callback(|e: InputData| ChatMsg::RoomInput(e.value))/>
                            </div>

                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{{"Close"}}</button>
                                <button type="button" class="btn btn-primary" data-bs-dismiss="modal" onclick=self.link.callback(|_| ChatMsg::SwitchRoom)>{{"Confirm"}}</button>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="modal" id="createRoomModal">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h4 class="modal-title">{{"Create room"}}</h4>
                                <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                            </div>

                            <div class="modal-body">
                                <label>{{"Room Code (share this with other stonkers):"}}</label>
                                <p class="fw-bold"> {{ self.change_room.clone() }} </p>
                            </div>

                            <div class="modal-footer">
                                <button type="button" class="btn btn-danger" data-bs-dismiss="modal">{{"Close"}}</button>
                                <button type="button" class="btn btn-primary" data-bs-dismiss="modal" onclick=self.link.callback(|_| ChatMsg::SwitchRoom)>{{"Confirm"}}</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        }
    }
}