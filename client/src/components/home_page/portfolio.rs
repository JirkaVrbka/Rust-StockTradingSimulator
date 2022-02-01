use utils::json::PortfolioJSON;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties, classes};
use crate::fetcher::{ToHtml, NoProps};

impl ToHtml for PortfolioJSON {
    fn to_html(&self, _:NoProps) -> Html {
        let money = self.money;
        let stock = &self.stock;
        let share = self.share as f32 / 10000_f32;
        let diff = self.difference as f32 / 10000_f32;
        let money_color = if money < 0 {"text-danger"} else {"text-success"};
        let diff_color = if diff < 0.0 {"text-danger"} else {"text-success"};
        html! {
             <div class="row my-3">
                    <div class="col-3">{stock}</div>
                    <div class="col-3">{ if share > 0.0 {share.to_string() + "%"} else {"-".to_string()} }</div>
                    <div class={classes!("col-3", money_color.clone())}>{money}{"$"}</div>
                    <div class={classes!("col-3", diff_color.clone())}>{diff}{"%"}</div>
                </div>
        }
    }
}

impl ToHtml for Vec<PortfolioJSON> {
    fn to_html(&self, _:NoProps) -> Html {
        html! {
                self.iter().map(|el| html!{
                        { el.to_html(NoProps) }
                }).collect::<Html>()
        }
    }
}

#[derive(Clone, Properties)]
pub struct Props {
    pub portfolios : Vec<PortfolioJSON>
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
                {self.props.portfolios.to_html(NoProps)}
            </div>
            </>
        }
    }
}
