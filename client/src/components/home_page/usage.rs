use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct Usage {
}

impl Component for Usage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <h2 class="fw-bolder">{"USAGE"}</h2>
                        <div class="chart-container" style="position: relative; height:40vh; width:40vw">
                            <canvas id="usageChart"></canvas>
                        </div>
            <script>{"
            const ctx = document.getElementById('usageChart').getContext('2d');
            const usageChart = new Chart(ctx, {
                type: 'doughnut',
                data: {
                    labels: [
                        '86$ Free',
                        '202$ Invested',
                        '79$ Blocked'
                    ],
                    datasets: [{
                        label: 'My First Dataset',
                        data: [86, 202, 79],
                        backgroundColor: [
                            'rgb(144,203,74)',
                            'rgb(74,176,187)',
                            'rgb(124,130,151)'
                        ],
                        hoverOffset: 4
                    }]
                },
                options: {
                    responsive: true,
                    plugins: {
                        legend: {
                            position: 'right',
                            labels: {
                                boxHeight: 15,
                                boxWidth: 15,
                                padding: 30,
                                font: {
                                    size: 20
                                }
                            }
                        }
                    }
                }
            });
            usageChart.canvas.parentNode.style.height = '50%';
            usageChart.canvas.parentNode.style.width = '70%';
            "}</script>
            </>
        }
    }
}
