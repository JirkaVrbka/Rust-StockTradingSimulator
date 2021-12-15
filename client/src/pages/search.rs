use yew::prelude::*;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::text::{Text, TextType};
use crate::fetcher::FetchServiceExample;

pub struct Search;

impl Component for Search {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Search {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap class_name="content">
            <Text
                plain_text="Search"
                text_type=TextType::Plain
            />
            <FetchServiceExample/>
            </Container>
        }
    }
}
