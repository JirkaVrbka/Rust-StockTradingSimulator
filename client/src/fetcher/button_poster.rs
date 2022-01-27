use yew::{
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response}, format::Json,
};
use yew_styles::spinner::{Spinner, SpinnerType};
use yew_styles::styles::{Palette, Size};
use yew_styles::button::Button;

#[derive(Debug)]
pub enum PostMsg {
    Post,
    ReceiveResponse(Result<(), anyhow::Error>),
}

#[derive(Debug)]
pub enum Post {
    Err(anyhow::Error),
    Ok,
    Wait,
    Posting(FetchTask),
}

#[derive(Debug, Clone, Properties)]
pub struct PostProps<T: 'static + Clone + PartialEq + serde::Serialize> {
    pub port: &'static str,
    pub data: T,
    pub text: &'static str,
}

#[derive(Debug)]
pub struct ButtonPoster<T: 'static + Clone + PartialEq + serde::Serialize> {
    post: Post,
    props: PostProps<T>,
    link: ComponentLink<Self>,
}

impl<T: 'static + Clone + PartialEq + serde::Serialize> Component for ButtonPoster<T> {
    type Message = PostMsg;
    type Properties = PostProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            post: Post::Wait,
            props,
            link,
        }
    }
    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.data != props.data {
            self.props.data = props.data;
            true
        } else {
            false
        }
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            PostMsg::Post => {
                let val = serde_json::to_value(self.props.data.clone()).unwrap();
                let request = Request::post(format!("http://localhost:8081/{}", self.props.port).as_str())
                    .header("Content-Type", "application/json")
                    .body(Json(&val))
                    .expect("Could not build request.");
                let callback = self.link
                    .callback(|response: Response<Json<Result<(), anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        PostMsg::ReceiveResponse(data)
                    });
                self.post = Post::Posting(FetchService::fetch(request, callback).expect("failed to start post"));
                true
            },
            PostMsg::ReceiveResponse(response) => {
                match response {
                    Ok(()) => {
                        self.post = Post::Ok;
                    }
                    Err(error) => {
                        self.post = Post::Err(error);
                    }
                }
                true
            }
        }
    }
    fn view(&self) -> Html {
        match &self.post {
            Post::Wait => {
                html! {
                    <Button onclick_signal=self.link.callback(|_| PostMsg::Post)>
                        { self.props.text }
                    </Button>
                }
            }
            Post::Ok => {
                html! {
                    <p> { "Success" } </p>
                }
            },
            Post::Err(error) => html! {
                <p>{ error.to_string().clone() }</p>
            },
            Post::Posting(_) => html! {
                <Spinner
                    spinner_type=SpinnerType::Circle
                    spinner_size=Size::Medium
                    spinner_palette=Palette::Standard
                />
            }
        }
    }
}