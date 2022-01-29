pub mod button_poster;
pub mod button_login;
pub mod immediate_fetcher;

use utils::json::StonkerJSON;
use yew::html::Html;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct NoProps;

pub trait ToHtml<T=NoProps>: 'static + for<'de> serde::Deserialize<'de> {
    fn to_html(&self, props: T) -> Html;
}

use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::text::{Text, TextType};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};

impl ToHtml for StonkerJSON {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <Text plain_text="The Stonker" text_type=TextType::Plain/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <Text plain_text=format!("Name: {}", self.name) text_type=TextType::Plain/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <Text plain_text=format!("Ballance: {}", self.balance) text_type=TextType::Plain/>
                </Item>
            </Container>
        }
    }
}