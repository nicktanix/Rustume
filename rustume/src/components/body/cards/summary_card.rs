use yew::prelude::*;

pub struct SummaryCard {}

impl Component for SummaryCard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card mb-4" id="summaryCard">
                <div class="card-body">
                    <p class="card-text">
                    {" A passionate and skilled web developer with over 5 years of experience in front-end and
                        back-end
                        web
                        development. Proficient in HTML, CSS, JavaScript, and various web development frameworks."}
                    </p>
                </div>
            </div>
        }
    }
}