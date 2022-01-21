use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct History {
}

impl Component for History {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {  }
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
            <h2 class="fw-bolder">{"HISTORY"}</h2>

                        <div class="container-fluid g-0">
                            <div class="row text-secondary fst-italic">
                                <div class="col-auto">{"day"}</div>
                                <div class="col-3">{"action"}</div>
                                <div class="col-3">{"stock"}</div>
                                <div class="col-3">{"money"}</div>
                            </div>
                            <div class="row my-3">
                                <div class="col-auto">{"6.1"}</div>
                                <div class="col-3">{"Buy"}</div>
                                <div class="col-3">{"Netflix"}</div>
                                <div class="col-3 text-danger">{"-16$"}</div>
                            </div>
                            <div class="row my-3">
                                <div class="col-auto">{"4.1"}</div>
                                <div class="col-3">{"Delayed sell"}</div>
                                <div class="col-3">{"Amazon"}</div>
                                <div class="col-3 text-success">{"+40$"}</div>
                            </div>
                            <div class="row my-3">
                                <div class="col-auto">{"2.1"}</div>
                                <div class="col-3">{"Delayed buy"}</div>
                                <div class="col-3">{"Disney"}</div>
                                <div class="col-3 text-danger">{"-50$"}</div>
                            </div>
                            <div class="row my-3">
                                <div class="col-auto">{"1.1"}</div>
                                <div class="col-3">{"Sell"}</div>
                                <div class="col-3">{"Netflix"}</div>
                                <div class="col-3 text-success">{"+22$"}</div>
                            </div>
                        </div>
            </>
        }
    }
}
