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
pub struct ImmediateFetcher {
    fetch_task: FetchTask,
    stonker: Option<StonkerJSON>,
    link: ComponentLink<Self>,
    error: Option<String>,
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
            fetch_task: task,
            stonker: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            FetchMsg::ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.stonker = Some(location);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                true
            }
        }
    }
    fn view(&self) -> Html {
        match self.stonker {
            Some(ref investor) => {
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
            }
            None => {
                match self.error {
                    Some(ref error) => html! {
                        <p>{ error.clone() }</p>
                    },
                    None => html! {
                        <Spinner
                            spinner_type=SpinnerType::Circle
                            spinner_size=Size::Medium
                            spinner_palette=Palette::Standard
                        />
                    },
                }
            }
        }
    }
}