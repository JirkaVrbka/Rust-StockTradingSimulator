#![recursion_limit = "1024"]

use yew::prelude::*;
use crate::pages::{Home, Login, Chat, Search, News};
use crate::components::{Counter, NavElement};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::button::Button;

extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

pub struct Logged {
    link: ComponentLink<Self>,
    navbar_items: Vec<bool>,
}

pub enum Msg {
    ChangeNavbarItem(usize),
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    Root,
    #[to = "/home!"]
    Home,
    #[to = "/search!"]
    Search,
    #[to = "/news!"]
    News,
    #[to = "/chat!"]
    Chat,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
}


impl Component for Logged {
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
                <div class="d-flex flex-row h-100">
                    <div class="d-flex flex-column fs-1 bg-dark-gray text-white pe-5 ps-2" >
                        <NavElement name="Home" icon="bi bi-house-door" link="/home"/>
                        <NavElement name="Search" icon="bi bi-search" link="/search"/>
                        <NavElement name="News" icon="bi bi-graph-up" link="/news"/> // also try: bi-newspaper
                        <NavElement name="Chat" icon="bi bi-people-fill" link="/chat"/>
                    </div>
                    <div class="w-100">
                        <Router<AppRouter, ()>
                            render=Router::render(|switch: AppRouter | {
                                match switch {
                                    AppRouter::Root => html!{
                                        <Home/>
                                    },
                                    AppRouter::Home => html!{
                                        <Home/>
                                    },
                                    AppRouter::Search => html!{
                                        <Search/>
                                    },
                                    AppRouter::News => html!{
                                        <News/>
                                    },
                                    AppRouter::Chat => html!{
                                        <Chat/>
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
            </div>
        }
    }
}
