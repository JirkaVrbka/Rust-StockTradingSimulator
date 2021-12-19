use std::convert::TryInto;

use utils::json::{NewsJSON, StonkerJSON};
use yew::prelude::*;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};
use yew_styles::text::{Text, TextType};
use crate::fetcher::ToHtml;
use crate::fetcher::immediate::ImmediateFetcher;
use yew_styles::styles::{Size, Palette, Style};
use yew_styles::card::Card;

impl ToHtml for NewsJSON {
    fn to_html(&self) -> Html {
        let header = &self.title;
        let body = &self.description;
        let footer = &self.author;
        let palette = match self.effect {
            utils::json::EffectJSON::Fall => Palette::Danger,
            utils::json::EffectJSON::Neutral => Palette::Info,
            utils::json::EffectJSON::Rise => Palette::Success,
        };
        html! {
            <Card
                card_size=Size::Medium
                card_palette=palette
                card_style=Style::Regular
                header=Some(html!{
                    <div>{header}</div>
                })
                body=Some(html!{
                    <div>{body}</div>
                })
                footer=Some(html!{
                    <div>{footer}</div>
                })
            />
        }
    }
}

impl ToHtml for Vec<NewsJSON> {
    fn to_html(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap> {
                self.iter().map(|el| html!{
                    <Item layouts=vec!(ItemLayout::ItXs(self.len().try_into().unwrap())) align_self=AlignSelf::FlexStart>
                        { el.to_html() }
                    </Item>
                }).collect::<Html>()
            } </Container>
        }
    }
}

pub struct NewsPage;

impl Component for NewsPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        NewsPage {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <Text plain_text="News" text_type=TextType::Plain />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <ImmediateFetcher::<Vec<NewsJSON>> port="news"/>
                </Item>
            </Container>
        }
    }
}

