use utils::json::StonkerJSON;
use yew::prelude::*;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};
use yew_styles::text::{Text, TextType};
use crate::fetcher::immediate::ImmediateFetcher;

pub struct Search;

impl Component for Search {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Search {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="container-fluid ms-4 mt-4">
                    <div class="row">
                        <div class="col-9">
                            <label>
                                <input class="border search-input fs-1 px-4 py-2" placeholder="Search"/>
                            </label>
                        </div>
                        <div class="col fs-4">
                            <div class="form-check form-switch">
                                <input class="form-check-input" type="checkbox" id="switchForOffered"/>
                                <label class="form-check-label" for="switchForOffered">{"Offered"}</label>
                            </div>
                            <div class="form-check form-switch">
                                <input class="form-check-input" type="checkbox" id="switchForCheapest"/>
                                <label class="form-check-label" for="switchForCheapest">{"Cheapest"}</label>
                            </div>
                        </div>
                    </div>

                    <div class="row mt-4 pt-3 ms-4">
                        <div class="col-4 fst-italic text-muted">{"name"}</div>
                        <div class="col-2 fst-italic text-muted">{"stock"}</div>
                        <div class="col-2 fst-italic text-muted">{"change"}</div>
                        <div class="col-2 fst-italic text-muted">{"percentage"}</div>
                    </div>

                    <div class="row my-2 ms-4 fs-3">
                        <div class="col-4">{"Netscape"}</div>
                        <div class="col-2">{"10"}</div>
                        <div class="col-2 text-success">{"+2$"}</div>
                        <div class="col-2 text-success">{"+1%"}</div>
                    </div>

                    <div class="row my-2 ms-4 fs-3">
                        <div class="col-4">{"Networld"}</div>
                        <div class="col-2">{"10"}</div>
                        <div class="col-2 text-danger">{"-4$"}</div>
                        <div class="col-2 text-danger">{"-3%"}</div>
                    </div>

                    <div class="row my-2 ms-4 fs-3">
                        <div class="col-4">{"Netscape"}</div>
                        <div class="col-2">{"15"}</div>
                        <div class="col-2 text-success">{"+5$"}</div>
                        <div class="col-2 text-success">{"+8%"}</div>
                    </div>
                </div>

            // <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
            //     <Item layouts=vec!(ItemLayout::ItXs(2)) align_self=AlignSelf::Auto>
            //         <Text plain_text="Search" text_type=TextType::Plain />
            //     </Item>
            //     <Item layouts=vec!(ItemLayout::ItXs(2)) align_self=AlignSelf::Auto>
            //         <ImmediateFetcher::<StonkerJSON> port="stonkers/1"/>
            //     </Item>
            // </Container>
            </div>
        }
    }
}
