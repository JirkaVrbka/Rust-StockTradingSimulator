use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct Graph {
}

impl Component for Graph {
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
            </>
        }
    }
}
