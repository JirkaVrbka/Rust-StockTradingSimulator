use utils::json::CompanyDetailJSON;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use crate::fetcher::{ToHtml, NoProps};

use crate::components::company_page::CustomizeModal;

impl ToHtml for CompanyDetailJSON {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <div>
                <div class="flex-fill fs-3">
                    <div class="container-fluid ms-3 mt-3">
                        <div class="row">
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
