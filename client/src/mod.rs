#![recursion_limit = "1024"]

pub mod fetcher;

use yew::{prelude::*, services::ConsoleService};
use crate::pages::{Login, Logged};

mod cookie;
mod pages;
mod components;
mod dto;

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _prop: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="row fs-8vh bg-gray text-white ps-5 py-2">{"STONKER$"}</div> {
                    match cookie::is_logged().unwrap_or(false) {
                        true => html!{ <Logged/> },
                        false => html!{ <Login/> }
                    }
                }
            </div>
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}

