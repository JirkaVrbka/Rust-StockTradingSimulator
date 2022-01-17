use yew::{Html, html};
use crate::fetcher::ToHtml;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioDto{
    pub(crate) stock: String,
    pub(crate) share: f32,
    pub(crate) money: i32,
    pub(crate) difference: f32
}


impl ToHtml for Vec<PortfolioDto> {
    fn to_html(&self) -> Html {
        html! {
                self.iter().map(|el| html!{
                        { el.to_html() }
                }).collect::<Html>()
        }
    }
}

impl ToHtml for PortfolioDto {
    fn to_html(&self) -> Html {
        let money = self.money;
        let stock = &self.stock;
        let share = self.share;
        let diff = self.difference;
        html! {
             <div class="row">
                    <div class="col-3 text-muted">{money}</div>
                    <div class="col-3 text-muted">{stock}</div>
                    <div class="col-3 text-muted">{money}</div>
                    <div class="col-3 text-muted">{diff}</div>
                </div>
        }
    }
}