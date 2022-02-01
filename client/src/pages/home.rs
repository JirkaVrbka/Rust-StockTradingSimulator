use yew::prelude::*;
use crate::components::home_page::{Usage, Graph, History};
use crate::cookie;
use crate::fetcher::{NoProps, ToHtml, immediate_fetcher::ImmediateFetcher};
use utils::json::{StockJSON, PortfolioJSON, StonkerOverviewJSON};
use crate::components::home_page::portfolio;

impl ToHtml for StonkerOverviewJSON {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <div class="flex-fill fs-3">
                <div class="container-fluid ms-3 mt-3">
                    <div class="row">
                        <div class="col-6 pe-4">
                            { self.portfolio.to_html(NoProps) }
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
