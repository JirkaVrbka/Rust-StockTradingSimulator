use stylist::css;
use utils::json::{StonkerJSON, CompanyJSON};
use yew::prelude::*;
use yew_styles::layouts::container::{Container, Direction, Wrap};
use yew_styles::layouts::item::{AlignSelf, Item, ItemLayout};
use yew_styles::text::{Text, TextType};
use yew_styles::forms::form_input::FormInput;
use yew_styles::button::Button;
use crate::fetcher::ToHtml;
use crate::fetcher::immediate_fetcher::{ImmediateFetcher, ExtraProps};
use crate::components::home_page::{Graph, History};

impl ToHtml<ExtraProps<Search, String>> for CompanyJSON {
    fn to_html(&self, props: ExtraProps<Search, String>) -> Html {
        let id = self.id.clone();
        html! {
            <div class="row my-1 ms-4">
                <div class="col-6">{self.performer.name.clone()}</div>
                <div class="col-2">{self.name.clone()}</div>
                <div class="col-2">
                 <Button
                    onclick_signal=props.link.callback(move |_| SearchMsg::Select(id))
                    class_name="py-0 fs-6 btn btn-outline-info">
                        {"More"}
                    </Button>
                </div>
            </div>
        }
    }
}

impl ToHtml<ExtraProps<Search, String>> for Vec<CompanyJSON> {
    fn to_html(&self, props: ExtraProps<Search, String>) -> Html {
        html! {
            <div>
                <div class="container-fluid ms-4 mt-4">
                    <div class="row">
                        <div class="col-9">
                            <label>
                                <FormInput
                                    class_name="border search-input fs-1 px-4 py-2"
                                    oninput_signal=props.link.callback(|e: InputData| SearchMsg::Search(e.value))
                                    placeholder="Hledat"
                                />
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
                        <div class="col-6 fst-italic text-muted">{"name"}</div>
                        <div class="col-2 fst-italic text-muted">{"stock"}</div>
                        <div class="col-2 fst-italic text-muted"></div>
                    </div> {
                        self.iter()
                            .filter(|company|
                                if props.extra.is_empty() {
                                    true
                                } else {
                                    format!("{} {}",
                                        company.name.to_lowercase(),
                                        company.performer.name.to_lowercase(),
                                    ).contains(&props.extra.to_lowercase())
                                }
                            )
                            .take(8)
                            .map(|el| html!{
                                { el.to_html(props.clone()) }
                            }).collect::<Html>()
                    }
                </div>
            </div>
        }
    }
}

pub enum SearchMsg {
    Search(String),
    Select(i32),
}

#[derive(Debug, Clone)]
pub struct Search {
    link: ComponentLink<Self>,
    search: String,
    selected: Option<i32>,
}

impl Component for Search {
    type Message = SearchMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Search {
            link,
            search: String::new(),
            selected: None
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            SearchMsg::Search(s) => { self.search = s; true },
            SearchMsg::Select(s) => { self.selected = Some(s); true },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.selected.is_none() {
            let extras = ExtraProps{link: self.link.clone(), extra: self.search.clone()};
            html! {
                <Container direction=Direction::Column wrap=Wrap::Wrap class_name="align-item">
                    <Item layouts=vec!(ItemLayout::ItXs(2)) align_self=AlignSelf::Auto>
                        <ImmediateFetcher::<Vec<CompanyJSON>, ExtraProps<Self, String>>
                            port="companies" extra=extras/>
                    </Item>
                </Container>
            }
        } else {
            html!{
                <div class="flex-fill fs-3">
                    <div class="container-fluid ms-3 mt-3">
                        <div class="row">
                            <div class="col-6 pe-4"><Graph/></div>
                            <div class="col-6 ps-4"><History/></div>
                        </div>
                        <div class="row justify-content-start">
                            <div class="col-auto">
                                <button class="btn btn-success rounded-4 text-white fs-1">{"BUY FOR 9$"}</button>
                            </div>
                            <div class="col-auto">
                                <button class="btn btn-danger rounded-4 text-white fs-1">{"SELL FOR 7$"}</button>
                            </div>
                            <div class="col-auto">
                                <button class="btn btn-info rounded-4 text-white fs-1">{"CUSTOMIZE"}</button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
    }
}
