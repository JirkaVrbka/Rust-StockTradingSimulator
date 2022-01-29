use yew::prelude::*;
use crate::components::home_page::{Portfolio, Usage, Graph, History};
use crate::dto::PortfolioDto;

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
            <div class="flex-fill fs-3">
                <div class="container-fluid ms-3 mt-3">
                    <div class="row">
                        <div class="col-6 pe-4">
                            <Portfolio portfolios=self.portfolios.clone()/>
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
