use utils::json::NewsJSON;
use yew::prelude::*;
use crate::fetcher::ToHtml;
use crate::fetcher::immediate::ImmediateFetcher;

use crate::components::{NewsCard};

impl ToHtml for NewsJSON {
    fn to_html(&self) -> Html {
        let header = self.title.clone();
        let body = self.description.clone();
        let footer = self.author.clone();
        let palette = match self.effect {
            utils::json::EffectJSON::Fall => "bg-danger",
            utils::json::EffectJSON::Neutral => "",
            utils::json::EffectJSON::Rise => "bg-success",
        }.to_string();
        html! {
            <NewsCard title=header text=body author=footer color=palette/>
        }
    }
}

impl ToHtml for Vec<NewsJSON> {
    fn to_html(&self) -> Html {
        // Notice that we skip all the old news
        html! {
            self.iter().take(8).map(|el| html!{
                { el.to_html() }
            }).collect::<Html>()
        }
    }
}

pub struct News;

impl Component for News {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        News {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container-fluid">
                <div class="row">
                    <h1 class="text-left">{"News"}</h1>
                </div>
                <div class="row cols-3">
                    <ImmediateFetcher::<Vec<NewsJSON>> port="news"/>
                </div>
            </div>
        }
    }
}

