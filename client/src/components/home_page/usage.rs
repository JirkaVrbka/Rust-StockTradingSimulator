use utils::json::UsageJSON;
use yew::{html, Html};

use crate::fetcher::{ToHtml, NoProps};

impl ToHtml for UsageJSON {
    fn to_html(&self, _: NoProps) -> Html {
        html! {
            <>
                <h2 class="fw-bolder">{"USAGE"}</h2>
                <div style="width:15vw;">
                    <canvas id="usageChart"></canvas>
                </div>
                <script>{format!("
                const ctx = document.getElementById('usageChart').getContext('2d');
                const free = {};
                const invested = {};
                const blocked = {};
                const usageChart = new Chart(ctx, {{
                    type: 'doughnut',
                    data: {{
                        labels: [
                            free + '$ Free',
                            invested + '$ Invested',
                            blocked + '$ Blocked'
                        ],
                        datasets: [{{
                            label: 'My First Dataset',
                            data: [free, invested, blocked],
                            backgroundColor: [
                                free >= 0 ? 'rgb(144,203,74)' : '#A12525',
                                'rgb(74,176,187)',
                                'rgb(124,130,151)'
                            ],
                            hoverOffset: 4
                        }}]
                    }},
                    options: {{
                        responsive: true,
                        plugins: {{
                            legend: {{
                                position: 'right',
                                labels: {{
                                    boxHeight: 15,
                                    boxWidth: 15,
                                    padding: 30,
                                    font: {{
                                        size: 20
                                    }}
                                }}
                            }}
                        }}
                    }}
                }});", self.free, self.invested, self.blocked)}</script>
            </>
        }
    }
}