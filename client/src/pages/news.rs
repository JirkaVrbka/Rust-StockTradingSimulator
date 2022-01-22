use std::convert::TryInto;

use utils::json::NewsJSON;
use yew::prelude::*;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};
use yew_styles::text::{Text, TextType};
use crate::fetcher::ToHtml;
use crate::fetcher::immediate::ImmediateFetcher;
use yew_styles::styles::{Size, Palette, Style};
use yew_styles::card::Card;

use crate::components::{NewsCard};

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
            <div class="col">
                <div class="card h-100 m-2 p-2 bg-warning">
                    <div class="card-body">
                        <h5 class="card-title">{header}</h5>
                        <p class="card-text">{body}</p>
                    </div>
                    <div class="card-footer text-end border-0 bg-warning">{footer}</div>
                </div>
            </div>
        }
    }
}

impl ToHtml for Vec<NewsJSON> {
    fn to_html(&self) -> Html {
        html! {
                self.iter().map(|el| html!{
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
            <div>
                <h1 class="text-center">{"News"}</h1>
                <div class="d-flex flex-wrap w-100">
                    <NewsCard title="bla bla" text="a tak dale" author="the dog" color="bg-info"/>
                    <NewsCard title="bla bla" text="a tak dale" author="the dog" color="bg-info"/>
                    <NewsCard title="bla bla" text="a tak dale" author="the dog" color="bg-info"/>
                </div>
            </div>
            // <div class="container-fluid">
            //     <div class="row">
            //         <h1 class="text-center">{"News"}</h1>
            //     </div>
            //     <div class="row cols-3">
            //         <ImmediateFetcher::<Vec<NewsJSON>> port="news"/>
            //     </div>
            // </div>
        }
    }
}

