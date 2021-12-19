pub mod news;
use utils::json::{NewsJSON, StonkerJSON};
use yew::prelude::*;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};
use yew_styles::text::{Text, TextType};
use crate::fetcher::immediate::ImmediateFetcher;
use news::NewsComponent;

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
        let data = NewsJSON {
            id: 1,
            title: "News title".to_string(),
            description: "News description".to_string(),
            author: "News author".to_string(),
            created_at: chrono::NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            effect: utils::json::EffectJSON::Neutral,
            company: utils::json::CompanyJSON {
                id: 0,
                name: "Company".to_string(),
                performer: StonkerJSON{
                    id: 0,
                    name: "Company representative".to_string(),
                    balance: 5000,
                    blocked_balance: 0,
                    invested_balance: 0,
                }
            },
        };
        html! {
            <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <Text plain_text="News" text_type=TextType::Plain />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <NewsComponent data=data/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                    <ImmediateFetcher::<StonkerJSON> port="stonkers/1"/>
                </Item>
            </Container>
        }
    }
}

