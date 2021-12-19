
use utils::json::NewsJSON;
use yew::prelude::*;
use yew_styles::styles::{Size, Palette, Style};
use yew_styles::card::Card;

#[derive(Clone, Properties)]
pub struct NewsProps {
    pub data: NewsJSON
}

pub struct NewsComponent {
    json: NewsJSON,
    link: ComponentLink<Self>,
}

impl Component for NewsComponent {
    type Message = ();
    type Properties = NewsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NewsComponent{json: props.data, link}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let header = &self.json.title;
        let body = &self.json.description;
        let footer = &self.json.author;
        let palette = match self.json.effect {
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