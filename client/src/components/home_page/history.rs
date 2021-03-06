use utils::json::{StonkerHistoryJSON, CommandTypesJSON};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::components::graph;
use crate::fetcher::{ToHtml, NoProps};

pub struct MoneyProp(pub i32);

fn command_to_str(kind: CommandTypesJSON) -> &'static str {
    match kind {
        CommandTypesJSON::Sell => "Sell",
        CommandTypesJSON::SellIfHigh => "Sell if high",
        CommandTypesJSON::SellIfLow => "Sell if low",
        CommandTypesJSON::BuyIfLow => "Buy if low",
    }
}

impl ToHtml for StonkerHistoryJSON {
    fn to_html(&self, _: NoProps) -> Html {
        let (style, sign) = if self.money >= 0 { ("success", '+') } else { ("failure", '-') };
        html! {
            <div class="row my-3">
                <div class="col-3">{self.day.clone()}</div>
                <div class="col-3">{command_to_str(self.action.clone())}</div>
                <div class="col-3">{self.stock.clone()}</div>
                <div class=format!("col-3 text-{}", style)>{format!("{}{}$", sign, self.money)}</div>
            </div>
        }
    }
}

impl ToHtml<MoneyProp> for Vec<StonkerHistoryJSON> {
    fn to_html(&self, money: MoneyProp) -> Html {
        html! {
            <div class="row">
                <div class="col-6 pe-4"> {
                    graph(
                        "Your money",
                        self.iter().rev().map(|history| history.day.clone()).collect::<Vec<String>>(),
                        self.iter().fold(vec![money.0], |mut acc, val| {
                            acc.push(acc.last().unwrap() - val.money);
                            acc
                        }).into_iter().rev().skip(1).collect::<Vec<i32>>()
                    )}
                </div>
                <div class="col-6 ps-4">
                    <h2 class="fw-bolder">{"HISTORY"}</h2>
                    <div class="row text-secondary fst-italic">
                        <div class="col-3">{"day"}</div>
                        <div class="col-3">{"action"}</div>
                        <div class="col-3">{"stock"}</div>
                        <div class="col-3">{"money"}</div>
                    </div>

                    <div class="container-fluid g-0">
                        { self.iter().take(10).map(|history| history.to_html(NoProps)).collect::<Html>() }
                    </div>
                </div>
            </div>
        }
    }
}
