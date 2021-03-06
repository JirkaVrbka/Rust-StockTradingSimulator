use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, Credentials, FetchTask, Request, Response, FetchOptions},
};
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::styles::{Palette, Size};
use super::{NoProps, ExtraProps};

use super::ToHtml;

impl<L: Clone + yew::Component, E: Clone + PartialEq> PartialEq for ExtraProps<L, E> {
    fn eq(&self, other: &Self) -> bool {
        self.extra == other.extra
    }
}

#[derive(Debug, Clone, Properties)]
pub struct FetchProps<T: Clone + PartialEq, S: Clone + ToString=&'static str> {
    pub port: S,
    pub extra: T,
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
pub struct ImmediateFetcher<T: 'static + ToHtml<F>, F: 'static + Clone + PartialEq=NoProps, S: 'static + Clone + ToString=&'static str> {
    fetch: Fetch<T>,
    link: ComponentLink<Self>,
    props: FetchProps<F, S>,
}

impl<T: 'static + ToHtml<F>, F: 'static + Clone + PartialEq, S: 'static + Clone + ToString> Component for ImmediateFetcher<T, F, S> {
    type Message = FetchMsg<T>;
    type Properties = FetchProps<F, S>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get(format!("http://localhost:8081/{}", props.port.to_string()))
            .body(Nothing)
            .expect("Could not build request.");
        let options = FetchOptions {
            credentials: Some(Credentials::Include),
            ..FetchOptions::default()
        };
        let callback =
            link
                .callback(|response: Response<Json<Result<T, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    FetchMsg::ReceiveResponse(data)
                });
        let task = FetchService::fetch_with_options(request, options, callback).expect("failed to start request");
        Self {
            fetch: Fetch::Fetching(task),
            props,
            link,
        }
    }
    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.extra != props.extra {
            self.props.extra = props.extra;
            true
        } else {
            false
        }
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
            Fetch::Data(ref data) => {
                data.to_html(self.props.extra.clone())
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