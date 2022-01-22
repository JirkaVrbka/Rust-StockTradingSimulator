use yew::{Callback, Component, ComponentLink, Html, html, Properties, classes};

#[derive(Clone, Properties)]
pub struct Props {
    pub title: String,
    pub text: String,
    pub author: String,
    pub color: String
}

pub struct NewsCard {
    props : Props
}

impl Component for NewsCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
                <div class="col-3">
                    <div class={classes!("card", "m-3", "rounded-3", self.props.color.clone())}>
                        <div class="card-title fs-4 ps-3 pt-3">
                            { self.props.title.clone() }
                        </div>
                        <div class="card-body">
                            { self.props.text.clone() }
                        </div>
                        <div class="card-footer text-end border-0 mt-3">
                            { self.props.author.clone() }
                        </div>
                    </div>
                </div>
        }
    }
}