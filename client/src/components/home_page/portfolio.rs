use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use crate::dto::PortfolioDto;
use crate::fetcher::ToHtml;


#[derive(Clone, Properties)]
pub struct Props {
    pub portfolios : Vec<PortfolioDto>
}

pub struct Portfolio {
    pub props : Props
}

impl Component for Portfolio {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
             <h2 class="fw-bolder">{"PORTFOLIO"}</h2>
                        <div class="container-fluid g-0">
                            <div class="row text-secondary fst-italic">
                                <div class="col-3">{"stock"}</div>
                                <div class="col-3">{"share"}</div>
                                <div class="col-3">{"money"}</div>
                                <div class="col-3">{"difference"}</div>
                            </div>
                {self.props.portfolios.to_html()}
            </div>
            </>
        }
    }
}
