use utils::json::{CompanyDetailJSON, StockJSON};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use crate::components::graph;
use chrono::Datelike;
use crate::fetcher::{ToHtml, NoProps};

use crate::components::company_page::CustomizeModal;

impl ToHtml for StockJSON {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <div class="row my-3">
                <div class="col-3">{self.id.to_string()}</div>
                <div class="col-3">{self.owner.name.clone()}</div>
                <div class="col-3">{format!("{} %", (self.share as f64) / 10000.0)}</div>
            </div>
        }
    }
}

impl ToHtml for Vec<StockJSON> {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <div class="col-6 ps-4">
                <h2 class="fw-bolder">{"STOCKS"}</h2>
                <div class="container-fluid g-0">
                    <div class="row text-secondary fst-italic">
                        <div class="col-3">{"id"}</div>
                        <div class="col-3">{"owner"}</div>
                        <div class="col-3">{"share"}</div>
                    </div> {
                    self.iter().take(12).map(|el| html!{
                        { el.to_html(NoProps) }
                    }).collect::<Html>()
                } </div>
            </div>
        }
    }
}

impl ToHtml for CompanyDetailJSON {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <div class="col-6 pe-4">
                {graph("Company money",
                    self.value_history.iter()
                        .map(|history| format!("{}-{}-{}",
                            history.datetime.year(),
                            history.datetime.month(),
                            history.datetime.day()))
                        .collect::<Vec<String>>(),
                    self.value_history.iter().map(|history| history.value).collect::<Vec<i32>>()
                )}
                <div class="row justify-content-start">
                    <div class="col-auto">
                        <button class="btn btn-success rounded-4 text-white fs-1">{"BUY CHEAPEST"}</button>
                    </div>
                    <div class="col-auto">
                        <button class="btn btn-danger rounded-4 text-white fs-1">{"SELL NOW"}</button>
                    </div>
                    <div class="col-auto">
                        <CustomizeModal/>
                    </div>
                </div>
            </div>
        }
    }
}
