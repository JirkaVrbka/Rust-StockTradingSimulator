use yew::prelude::*;
use crate::components::home_page::{Portfolio, Usage, Graph, History};
use crate::cookie;
use crate::fetcher::{NoProps, ToHtml, immediate_fetcher::ImmediateFetcher};
use utils::json::{StockJSON, PortfolioJSON, StonkerOverviewJSON};

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

/*
impl ToHtml for Vec<StockJSON> {
    fn to_html(&self, _: NoProps) -> Html {
        let portfolios = self.iter().map(|stock| {
            let current_price = 10;
            PortfolioDto {
                stock: stock.owner.name.clone(),
                share: (stock.share as f32)/10000_f32,
                difference: 100. * ((current_price as f32)/(stock.bought_for as f32) - 1.),
                money: current_price - stock.bought_for,
            }
        }).collect::<Vec<PortfolioDto>>();
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
*/

impl ToHtml for StonkerOverviewJSON {
    fn to_html(&self, props: NoProps) -> Html {
        html! {
            <div class="flex-fill fs-3">
                <div class="container-fluid ms-3 mt-3">
                    <div class="row">
                        <div class="col-6 pe-4">
                            <Portfolio portfolios=self.portfolio.clone()/>
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
    stonker_addr: anyhow::Result<String>
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home {
            stonker_addr: cookie::get_login().map(|id| format!("stonkers/{}/overview", id)),
            link
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.stonker_addr {
            Ok(addr) => {
                html! {
                    //<ImmediateFetcher::<Vec<StockJSON>> port="l/stonkers/stocks" extra=NoProps/>
                    <ImmediateFetcher::<StonkerOverviewJSON,_,String> port=addr.clone() extra=NoProps/>
                }
            },
            Err(e) => html! {
                <p> { format!("Cookie ID Error: {:?}", e) } </p>
            }
        }
    }
}
