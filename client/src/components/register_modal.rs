use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct RegisterModal {
}

impl Component for RegisterModal {
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
            <span>

                // <!-- Button trigger modal -->
                <button type="button" class="btn btn-danger rounded-3 text-white fs-2 ms-3" data-bs-toggle="modal" data-bs-target="#registerModal">
                  {"Register"}
                </button>

                // <!-- Modal -->
                <div class="modal fade" id="registerModal" tabindex="-1" aria-labelledby="register" aria-hidden="true">
                  <div class="modal-dialog modal-lg">
                    <div class="modal-content">
                      <div class="modal-header border-0">
                        <h1 class="modal-title" id="register">{"Registration"}</h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                      </div>
                      <div class="modal-body">
                        <div class="container-fluid w-75 py-3">
                            <div class="row">
                                <input class="form-control border-0 border-bottom my-2 fs-2" placeholder="name"/>
                            </div>
                            <div class="row">
                                <input type="password" class="form-control border-0 border-bottom my-2 fs-2" placeholder="password"/>
                            </div>
                            <div class="row">
                                <input class="form-control border-0 border-bottom my-2 fs-2" placeholder="Your money"/>
                            </div>
                        </div>
                      </div>
                      <div class="modal-footer border-0">
                        <button type="button" class="btn btn-secondary rounded-3 fs-2" data-bs-dismiss="modal">{"Close"}</button>
                        <button type="button" class="btn btn-info text-white rounded-3 fs-2">{"Register"}</button>
                      </div>
                    </div>
                  </div>
                </div>

            </span>
        }
    }
}
