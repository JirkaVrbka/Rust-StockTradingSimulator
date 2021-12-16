use utils::json::StonkerJSON;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::styles::{Palette, Size};
use yew_styles::button::Button;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::text::{Text, TextType};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};


#[derive(Debug)]
pub enum FetchMsg {
    GetStonker,
    ReceiveResponse(Result<StonkerJSON, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    fetch_task: Option<FetchTask>,
    stonker: Option<StonkerJSON>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl FetchServiceExample {
    fn view_stonker_data(&self) -> Html {
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
                if self.fetch_task.is_none() {
                    html! {
                        <Button onclick_signal=self.link.callback(|_| FetchMsg::GetStonker)>
                            { "Who is the Stonker?" }
                        </Button>
                    }
                } else {
                    html! { }
                }
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! {  
                <Spinner
                    spinner_type=SpinnerType::Circle
                    spinner_size=Size::Medium
                    spinner_palette=Palette::Standard
                /> 
            }
        } else {
            html! { }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

impl Component for FetchServiceExample {
    type Message = FetchMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            stonker: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use FetchMsg::*;

        match msg {
            GetStonker => {
                // 1. build the request
                let request = Request::get("http://localhost:8081/stonkers/1")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<StonkerJSON, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            FetchMsg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.stonker = Some(location);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the Stonker instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_stonker_data() }
                { self.view_error() }
            </>
        }
    }
}