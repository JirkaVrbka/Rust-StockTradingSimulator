#![recursion_limit = "1024"]

pub mod fetcher;

use yew::prelude::*;
use crate::pages::{Home, Chat, History, Search, News};
use crate::components::{Counter, NavElement};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::button::Button;

extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

mod pages;
mod components;
mod dto;

struct Model {
    link: ComponentLink<Self>,
    navbar_items: Vec<bool>,
}

enum Msg {
    ChangeNavbarItem(usize),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    Root,
    #[to = "/chat!"]
    Chat,
    #[to = "/history!"]
    History,
    #[to = "/search!"]
    Search,
    #[to = "/news!"]
    News,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            navbar_items: vec![true, false, false, false, false],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
            <div>
            <div class="row fs-8vh bg-gray text-white ps-5 py-2">{"STONKER$"}</div>

                <div class="d-flex flex-row h-100">
                    <div class="d-flex flex-column fs-1 bg-dark-gray text-white pe-5 ps-2" >
                        <NavElement name="Home" icon="bi bi-house-door" link="/"/>
                        <NavElement name="Chat" icon="bi bi-file-earmark-text" link="/chat"/>
                        <NavElement name="History" icon="bi bi-graph-up" link="/history"/>
                        <NavElement name="Search" icon="bi bi-search" link="/search"/>
                        <NavElement name="News" icon="bi bi-exclamation-lg" link="/news"/>
                    </div>

                 <Router<AppRouter, ()>
                                    render=Router::render(|switch: AppRouter | {
                                        match switch {
                                            AppRouter::Root => html!{
                                                <Home/>
                                            },
                                            AppRouter::Chat => html!{
                                                <Chat/>
                                            },
                                            AppRouter::History => html!{
                                                <History/>
                                            },
                                            AppRouter::Search => html!{
                                                <Search/>
                                            },
                                            AppRouter::News => html!{
                                                <News/>
                                            },
                                            AppRouter::NotFound(Permissive(None)) => html!{"Page not found"},
                                            AppRouter::NotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                                        }
                                    })
                                    redirect=Router::redirect(|route: Route<()>| {
                                        AppRouter::NotFound(Permissive(Some(route.route)))
                                    })
                                />

                </div>
            </div>
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}

