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
                  <div class="modal-dialog">
                    <div class="modal-content">
                      <div class="modal-header">
                        <h5 class="modal-title" id="customize">{"Command"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                      </div>
                      <div class="modal-body">
                        <div class="container-fluid">
                            <div class="row"></div>
                            <div class="row">{"20$"}</div>
                            <div class="row">{"5%"}</div>
                        </div>
                      </div>
                      <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                        <button type="button" class="btn btn-primary">{"Save changes"}</button>
                      </div>
                    </div>
                  </div>
                </div>

            </div>
        }
    }
}
