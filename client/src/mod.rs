#![recursion_limit = "1024"]

pub mod fetcher;

use yew::prelude::*;
use crate::pages::{Home, Stocks, History, Search, News};
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
    #[to = "/stocks!"]
    Stocks,
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
            <div class="fs-big title ps-5">{"STONKER$"}</div>
                <div class="container-fluid">
                    <div class="row">
                    <div class="col-2 navigation">
                        <NavElement name="Home" icon="bi bi-house-door" link="/"/>
                        <NavElement name="Stocks" icon="bi bi-file-earmark-text" link="/stocks"/>
                        <NavElement name="History" icon="bi bi-graph-up" link="/history"/>
                        <NavElement name="Search" icon="bi bi-search" link="/search"/>
                        <NavElement name="News" icon="bi bi-exclamation-lg" link="/news"/>
                        <div class="row">
                            //
                            //     <RouterAnchor<AppRouter>route=AppRouter::Stocks>
                            //             <Button
                            //                 class_name="navbar-route btn fs-1 text-white"
                            //                 onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(1))>
                            //                 {"Stocks"}
                            //             </Button>
                            //         </RouterAnchor<AppRouter>>
                            //         <RouterAnchor<AppRouter>route=AppRouter::History>
                            //             <Button
                            //                 class_name="navbar-route btn fs-1 text-white"
                            //                 onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(2))>
                            //                 {"History"}
                            //             </Button>
                            //         </RouterAnchor<AppRouter>>
                            //         <RouterAnchor<AppRouter>route=AppRouter::Search>
                            //             <Button
                            //                 class_name="navbar-route btn fs-1 text-white"
                            //                 onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(3))>
                            //                 {"Search"}
                            //             </Button>
                            //         </RouterAnchor<AppRouter>>
                            //         <RouterAnchor<AppRouter>route=AppRouter::News>
                            //             <Button
                            //                 class_name="navbar-route btn fs-1 text-white"
                            //                 onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(4))>
                            //                 {"News"}
                            //             </Button>
                            //         </RouterAnchor<AppRouter>>
                        </div>
                    </div>
                    <div class="col mt-3">
                                <Router<AppRouter, ()>
                                    render=Router::render(|switch: AppRouter | {
                                        match switch {
                                            AppRouter::Root => html!{
                                                <Home/>
                                            },
                                            AppRouter::Stocks => html!{
                                                <Stocks/>
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
            </div>
            </div>
        }
    }
}

pub fn main() {
    yew::start_app::<Model>();
}

