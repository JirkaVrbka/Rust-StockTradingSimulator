use utils::json::StonkerHistoryJSON;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::fetcher::{ToHtml, NoProps};

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

                    <div class="container-fluid g-0">
                        <div class="row text-secondary fst-italic">
                            <div class="col-auto">{"day"}</div>
                            <div class="col-3">{"action"}</div>
                            <div class="col-3">{"stock"}</div>
                            <div class="col-3">{"money"}</div>
                        </div>
                        <div class="row my-3">
                            <div class="col-auto">{"6.1"}</div>
                            <div class="col-3">{"Buy"}</div>
                            <div class="col-3">{"Netflix"}</div>
                            <div class="col-3 text-danger">{"-16$"}</div>
                        </div>
                        <div class="row my-3">
                            <div class="col-auto">{"4.1"}</div>
                            <div class="col-3">{"Delayed sell"}</div>
                            <div class="col-3">{"Amazon"}</div>
                            <div class="col-3 text-success">{"+40$"}</div>
                        </div>
                        <div class="row my-3">
                            <div class="col-auto">{"2.1"}</div>
                            <div class="col-3">{"Delayed buy"}</div>
                            <div class="col-3">{"Disney"}</div>
                            <div class="col-3 text-danger">{"-50$"}</div>
                        </div>
                        <div class="row my-3">
                            <div class="col-auto">{"1.1"}</div>
                            <div class="col-3">{"Sell"}</div>
                            <div class="col-3">{"Netflix"}</div>
                            <div class="col-3 text-success">{"+22$"}</div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
