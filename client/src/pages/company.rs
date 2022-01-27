use utils::json::StonkerJSON;
use yew::prelude::*;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};
use yew_styles::text::{Text, TextType};
use crate::fetcher::immediate::ImmediateFetcher;

use crate::components::home_page::{Graph, History};

pub struct Company;

impl Component for Company {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Company {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="flex-fill fs-3">
                <div class="container-fluid ms-3 mt-3">
                    <div class="row">
                        <div class="col-6 pe-4"><Graph/></div>
                        <div class="col-6 ps-4"><History/></div>
                     </div>
                </div>
            </div>
            </>
            // <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
            //     <Item layouts=vec!(ItemLayout::ItXs(2)) align_self=AlignSelf::Auto>
            //         <Text plain_text="History" text_type=TextType::Plain />
            //     </Item>
            //     <Item layouts=vec!(ItemLayout::ItXs(2)) align_self=AlignSelf::Auto>
            //         <ImmediateFetcher::<StonkerJSON> port="stonkers/1"/>
            //     </Item>
            // </Container>
        }
    }
}
