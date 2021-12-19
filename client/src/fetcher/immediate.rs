use std::error::Error;

use utils::json::StonkerJSON;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::styles::{Palette, Size};
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::text::{Text, TextType};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};


#[derive(Debug)]
pub enum FetchMsg {
    ReceiveResponse(Result<StonkerJSON, anyhow::Error>),
}

#[derive(Debug)]
enum Fetch {
    Err(anyhow::Error),
    Data(StonkerJSON),
    Fetching(FetchTask)
}

#[derive(Debug)]
pub struct ImmediateFetcher {
    fetch: Fetch,
    link: ComponentLink<Self>,
}

impl Component for ImmediateFetcher {
    type Message = FetchMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get("http://localhost:8081/stonkers/1")
            .body(Nothing)
            .expect("Could not build request.");
        let callback =
            link
                .callback(|response: Response<Json<Result<StonkerJSON, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    FetchMsg::ReceiveResponse(data)
                });
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            fetch: Fetch::Fetching(task),
            link,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            FetchMsg::ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        self.fetch = Fetch::Data(data);
                    }
                    Err(error) => {
                        self.fetch = Fetch::Err(error);
                    }
                }
                true
            }
        }

    }
    fn view(&self) -> Html {
        match &self.fetch {
            Fetch::Data(ref investor) => {
                html! {
                    <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
                        <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                            <Text plain_text="The Stonker" text_type=TextType::Plain/>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                            <Text plain_text=format!("Name: {}", investor.name) text_type=TextType::Plain/>
                        </Item>
                        <Item layouts=vec!(ItemLayout::ItXs(3)) align_self=AlignSelf::Auto>
                            <Text plain_text=format!("Ballance: {}", investor.balance) text_type=TextType::Plain/>
                        </Item>
                    </Container>
                }
            },
            Fetch::Err(error) => html! {
                <p>{ error.to_string().clone() }</p>
            },
            Fetch::Fetching(_) => html! {
                <Spinner
                    spinner_type=SpinnerType::Circle
                    spinner_size=Size::Medium
                    spinner_palette=Palette::Standard
                />
            }
        }
    }
}