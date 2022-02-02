use std::fmt::Debug;

use yew::{html, Html};

pub fn graph<L,X,Y>(label: L, x: X, y:Y) -> Html where L: ToString + Debug, X: Debug, Y: Debug {
    html! {
        <div class="col-6 pe-4">
            <h2 class="fw-bolder">{"GRAPH"}</h2>
            <canvas id="lineChart"></canvas>
            <script> {format!("
                const ctxLine = document.getElementById('lineChart').getContext('2d');
                const lineChart = new Chart(ctxLine, {{
                    type: 'line',
                    data: {{
                        labels: {:?},
                        datasets: [{{
                            label: {:?},
                            data: {:?},
                            fill: false,
                            borderColor: 'rgb(75, 192, 192)',
                            tension: 0.4
                        }}]
                    }}
                }});",
                x,
                label.to_string(),
                y
            )}
            </script>
        </div>
    }
}