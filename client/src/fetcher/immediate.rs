use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::styles::{Palette, Size};

use super::ToHtml;

#[derive(Clone, Properties)]
pub struct FetchProps {
    pub port: &'static str
}

#[derive(Debug)]
pub enum FetchMsg<T> {
    ReceiveResponse(Result<T, anyhow::Error>),
}

#[derive(Debug)]
enum Fetch<T> {
    Err(anyhow::Error),
    Data(T),
    Fetching(FetchTask)
}

#[derive(Debug)]
pub struct ImmediateFetcher<T: ToHtml> {
    fetch: Fetch<T>,
}

impl<T: ToHtml> Component for ImmediateFetcher<T> {
    type Message = FetchMsg<T>;
    type Properties = FetchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get(format!("http://localhost:8081/{}", props.port))
            .body(Nothing)
            .expect("Could not build request.");
        let callback =
            link
                .callback(|response: Response<Json<Result<T, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    FetchMsg::ReceiveResponse(data)
                });
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            fetch: Fetch::Fetching(task),
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
                investor.to_html()
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