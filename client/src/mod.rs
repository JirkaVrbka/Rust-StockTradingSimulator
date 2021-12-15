#![recursion_limit = "1024"]
mod fetcher;

use yew::prelude::*;
use crate::pages::{Home, Stocks, History, Search, News};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::button::Button;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::styles::Size;
use yew_styles::text::{Text, TextType};

extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;

mod pages;

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
            <Container direction=Direction::Column wrap=Wrap::Nowrap>
                <Text
                    class_name="title"
                    text_type=TextType::Plain
                    plain_text="STONKER$"
                    text_size=Size::Big
                    html_text=None
                />
                <Container direction=Direction::Row wrap=Wrap::Nowrap>
                    <Container direction=Direction::Column wrap=Wrap::Wrap>
                        <RouterAnchor<AppRouter>route=AppRouter::Root>
                            <Button
                                class_name="navbar-route"
                                onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(0))>
                                {"Home"}
                            </Button>
                        </RouterAnchor<AppRouter>>
                        <RouterAnchor<AppRouter>route=AppRouter::Stocks>
                            <Button 
                                class_name="navbar-route"
                                onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(1))>
                                {"Stocks"}
                            </Button>
                        </RouterAnchor<AppRouter>>
                        <RouterAnchor<AppRouter>route=AppRouter::History>
                            <Button 
                                class_name="navbar-route"
                                onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(2))>
                                {"History"}
                            </Button>
                        </RouterAnchor<AppRouter>>
                        <RouterAnchor<AppRouter>route=AppRouter::Search>
                            <Button 
                                class_name="navbar-route"
                                onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(3))>
                                {"Search"}
                            </Button>
                        </RouterAnchor<AppRouter>>
                        <RouterAnchor<AppRouter>route=AppRouter::News>
                            <Button 
                                class_name="navbar-route"
                                onclick_signal=self.link.callback(|_| Msg::ChangeNavbarItem(4))>
                                {"News"}
                            </Button>
                        </RouterAnchor<AppRouter>>
                    </Container>
                    <Router<AppRouter, ()>
                        render = Router::render(|switch: AppRouter | {
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
                        } )
                        redirect = Router::redirect(|route: Route<()>| {
                            AppRouter::NotFound(Permissive(Some(route.route)))
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

