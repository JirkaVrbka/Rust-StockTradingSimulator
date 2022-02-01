use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct CustomizeModal {
}

impl Component for CustomizeModal {
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
            <div>

                // <!-- Button trigger modal -->
                <button type="button" class="btn btn-info rounded-4 text-white fs-1" data-bs-toggle="modal" data-bs-target="#customizeModal">
                  {"CUSTOMIZE"}
                </button>

                // <!-- Modal -->
                <div class="modal fade" id="customizeModal" tabindex="-1" aria-labelledby="customize" aria-hidden="true">
                  <div class="modal-dialog modal-lg">
                    <div class="modal-content">
                      <div class="modal-header">
                        <h5 class="modal-title" id="customize">{"Command"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                      </div>
                      <div class="modal-body">
                        <div class="container-fluid">
                            <div class="row">
                                <div class="dropdown fs-2">
                                  <button class="btn btn-outline-secondary dropdown-toggle w-100 border-0 border-bottom fs-2" type="button" id="buySellType" data-bs-toggle="dropdown" aria-expanded="false">
                                    {"Buy if low"}
                                  </button>
                                  <ul class="dropdown-menu" aria-labelledby="buySellType">
                                    <li><a class="dropdown-item fs-2" href="#">{"Buy if low"}</a></li>
                                    <li><a class="dropdown-item fs-2" href="#">{"Sell if high"}</a></li>
                                  </ul>
                                </div>
                            </div>

                            <div class="row">
                                <input class="form-control border-0 border-bottom my-2 fs-2" placeholder="2$"/>
                            </div>
                            <div class="row">
                                <input class="form-control border-0 border-bottom my-2 fs-2" placeholder="5%"/>
                            </div>
                        </div>
                      </div>
                      <div class="modal-footer">
                        <button type="button" class="btn btn-secondary rounded-3 fs-2" data-bs-dismiss="modal">{"Close"}</button>
                        <button type="button" class="btn btn-info text-white rounded-3 fs-2">{"Confirm"}</button>
                      </div>
                    </div>
                  </div>
                </div>

            </div>
        }
    }
}
