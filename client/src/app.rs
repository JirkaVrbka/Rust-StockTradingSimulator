use crate::pages::{About, Home};
use stylist::css;
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

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
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

pub enum Msg {
    ChangeNavbarItem(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            navbar_items: vec![true, false],
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeNavbarItem(index) => {
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }

                self.navbar_items[index] = true;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // Sidebar workaround until it is properly implemented inside yew-styles
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
