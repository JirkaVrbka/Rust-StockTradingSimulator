use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

use crate::components::company_page::CustomizeModal;

#[derive(Clone, Properties)]
pub struct Props {
    pub id: i32
}

pub struct Company {
    props : Props
}

impl Component for Company {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props: _props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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
