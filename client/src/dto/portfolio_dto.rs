use yew::{classes, Html, html};
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
    fn to_html(&self, _:()) -> Html {
        html! {
                self.iter().map(|el| html!{
                        { el.to_html(()) }
                }).collect::<Html>()
        }
    }
}

impl ToHtml for PortfolioDto {
    fn to_html(&self, _:()) -> Html {
        let money = self.money;
        let stock = &self.stock;
        let share = self.share;
        let diff = self.difference;
        let money_color = if money < 0 {"text-danger"} else {"text-success"};
        let diff_color = if diff < 0.0 {"text-danger"} else {"text-success"};
        html! {
             <div class="row my-3">
                    <div class="col-3">{stock}</div>
                    <div class="col-3">{ if share > 0.0 {share.to_string() + "%"} else {"-".to_string()} }</div>
                    <div class={classes!("col-3", money_color.clone())}>{money}{"$"}</div>
                    <div class={classes!("col-3", diff_color.clone())}>{diff}{"%"}</div>
                </div>
        }
    }
}