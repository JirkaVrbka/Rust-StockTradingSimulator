use yew::{Callback, Component, ComponentLink, Html, html, Properties};

#[derive(Clone, Properties)]
pub struct Props {
    pub name: String,
    pub icon: String,
    pub link: String
}

pub struct NavElement {
    props : Props
}

impl Component for NavElement {
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
            <a class="mt-3 no-highlight d-flex align-items-center" href=self.props.link.clone()>
                            <span class="text-secondary me-2">
                                <i class=self.props.icon.clone()></i>
                            </span>
                            <div class="">{ self.props.name.clone() }</div>
            </a>
        }
    }
}