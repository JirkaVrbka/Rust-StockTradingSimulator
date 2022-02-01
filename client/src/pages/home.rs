use yew::prelude::*;
use crate::components::home_page::{Portfolio, Usage, Graph, History};
use crate::dto::PortfolioDto;
use crate::fetcher::{NoProps, ToHtml, immediate_fetcher::ImmediateFetcher};
use utils::json::StockJSON;

/* Work in progress
impl ToHtml for StockJSON {
    fn to_html(&self, _: NoProps) -> Html {
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
}*/

impl ToHtml for Vec<StockJSON> {
    fn to_html(&self, _: NoProps) -> Html {
        let portfolios = vec![
            PortfolioDto { stock : "Netflix".to_string(),  share: 0.1, difference: -2.0, money: -5},
            PortfolioDto { stock : "Amazon".to_string(),  share: 12.0, difference: 22.0, money: 112},
            PortfolioDto { stock : "PizzaGuy".to_string(),  share: 5.0, difference: -12.0, money: -70},
            PortfolioDto { stock : "Total".to_string(),  share: -1.0, difference: 7.0, money: 37}];
        html! {
            <div class="flex-fill fs-3">
                <div class="container-fluid ms-3 mt-3">
                    <div class="row">
                        <div class="col-6 pe-4">
                            <Portfolio portfolios=portfolios/>
                         </div>
                        <div class="col-6 ps-4">
                            <Usage/>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col-6 pe-4"><Graph/></div>
                        <div class="col-6 ps-4"><History/></div>
                     </div>
                </div>
            </div>
            /*
            self.iter().take(8).map(|el| html!{
                { el.to_html(NoProps) }
            }).collect::<Html>()
            */
        }
    }
}

pub struct Home {
    link: ComponentLink<Self>,
    portfolios: Vec<PortfolioDto>
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home {
            link,
            portfolios: vec![
                PortfolioDto { stock : "Netflix".to_string(),  share: 0.1, difference: -2.0, money: -5},
                PortfolioDto { stock : "Amazon".to_string(),  share: 12.0, difference: 22.0, money: 112},
                PortfolioDto { stock : "PizzaGuy".to_string(),  share: 5.0, difference: -12.0, money: -70},
                PortfolioDto { stock : "Total".to_string(),  share: -1.0, difference: 7.0, money: 37}]
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <ImmediateFetcher::<Vec<StockJSON>> port="l/stonkers/stocks" extra=NoProps/>
        }
    }
}
