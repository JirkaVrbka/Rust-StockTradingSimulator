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
            <div class="container-fluid">
                <div class="row">
                    <div class="col-3 text-muted">{"stock"}</div>
                    <div class="col-3 text-muted">{"share"}</div>
                    <div class="col-3 text-muted">{"money"}</div>
                    <div class="col-3 text-muted">{"difference"}</div>
                </div>
                {self.props.portfolios.to_html()}
            </div>
        }
    }
}
