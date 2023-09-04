use yew::prelude::*;
use crate::components::body::cards as cards;
use cards::summary_card::SummaryCard;

pub struct SummaryContainer {}

impl Component for SummaryContainer {
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
            <div class="container mt-5" id="summaryContainer">
                    <div class="page-header text-white">
                        <h1 id="summary">{"Professional Summary"}</h1>
                    </div>
                    <div class="row">
                        <SummaryCard/>
                    </div>
                </div>
        }
    }
}