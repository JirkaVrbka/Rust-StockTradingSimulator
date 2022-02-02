use utils::json::{CompanyDetailJSON, StockJSON};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use crate::components::graph;
use chrono::Datelike;
use crate::fetcher::{ToHtml, NoProps};

use crate::components::company_page::CustomizeModal;

impl ToHtml for StockJSON {
    fn to_html(&self, props: NoProps) -> Html {
        todo!()
    }
}

impl ToHtml for Vec<StockJSON> {
    fn to_html(&self, props: NoProps) -> Html {
        todo!();
    }
}

impl ToHtml for CompanyDetailJSON {
    fn to_html(&self, _: NoProps) -> Html {

        html! {
            <div>
                <div class="flex-fill fs-3">
                    <div class="container-fluid ms-3 mt-3">
                        <div class="row"> {
                            graph("Company money",
                                self.value_history.iter()
                                    .map(|history| format!("{}-{}-{}",
                                        history.datetime.year(),
                                        history.datetime.month(),
                                        history.datetime.day()))
                                    .collect::<Vec<String>>(),
                                self.value_history.iter().map(|history| history.value).collect::<Vec<i32>>()
                        )}
                            //<div class="col-6 pe-4"><Graph/></div>
                            //<div class="col-6 ps-4"><History/></div>
                        </div>
                        <div class="row justify-content-start">
                            <div class="col-auto">
                                <button class="btn btn-success rounded-4 text-white fs-1">{"BUY FOR 9$"}</button>
                            </div>
                            <div class="col-auto">
                                <button class="btn btn-danger rounded-4 text-white fs-1">{"SELL FOR 7$"}</button>
                            </div>
                            <div class="col-auto">
                                <CustomizeModal/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
