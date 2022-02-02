use utils::json::NewsJSON;
use yew::prelude::*;
use crate::fetcher::{ToHtml, NoProps};
use crate::fetcher::immediate_fetcher::ImmediateFetcher;

impl ToHtml for NewsJSON {
    fn to_html(&self, _: NoProps) -> Html {
        let color = match self.effect {
            utils::json::EffectJSON::Fall => "bg-danger-custom",
            utils::json::EffectJSON::Neutral => "bg-basic-custom",
            utils::json::EffectJSON::Rise => "bg-success-custom",
        };
        html! {
            <div class="col-3">
                <div class={classes!("card", "m-3", "rounded-3", color)}>
                    <div class="card-title fs-4 ps-3 pt-3">
                        { self.title.clone() }
                    </div>
                    <div class="card-body">
                        { self.description.clone() }
                    </div>
                    <div class="card-footer text-end border-0 mt-3">
                        { self.author.clone() }
                    </div>
                </div>
            </div>
        }
    }
}

impl ToHtml for Vec<NewsJSON> {
    fn to_html(&self, _: NoProps) -> Html {
        // Notice that we skip all the old news
        html! {
            self.iter().take(8).map(|el| html!{
                { el.to_html(NoProps) }
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
                    <ImmediateFetcher::<Vec<NewsJSON>> port="news" extra=NoProps/>
                </div>
            </div>
        }
    }
}

