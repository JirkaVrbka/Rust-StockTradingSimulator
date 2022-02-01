use utils::json::{StonkerHistoryJSON, CommandTypesJSON};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::fetcher::{ToHtml, NoProps};

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

impl ToHtml for Vec<StonkerHistoryJSON> {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <div class="row">
                <div class="col-6 pe-4">
                    <h2 class="fw-bolder">{"GRAPH"}</h2>
                    <div class="chart-container" style="position: relative; height:40vh; width:40vw">
                        <canvas id="lineChart"></canvas>
                    </div>

                    <script> {"
                        const ctxLine = document.getElementById('lineChart').getContext('2d');
                        const lineChart = new Chart(ctxLine, {
                            type: 'line',
                            data: {
                                labels: ['1.1','2.1','3.1','4.1','5.1'],
                                datasets: [{
                                    label: 'Your money',
                                    data: [65, 59, 80, 81, 56],
                                    fill: false,
                                    borderColor: 'rgb(75, 192, 192)',
                                    tension: 0.4
                                }]
                            }
                        });
                        usageChart.canvas.parentNode.style.height = '50%';
                        usageChart.canvas.parentNode.style.width = '70%';"}
                    </script>
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
                        { self.iter().map(|history| history.to_html(NoProps)).collect::<Html>() }
                    </div>
                </div>
            </div>
        }
    }
}
