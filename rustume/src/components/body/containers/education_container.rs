use yew::prelude::*;
use crate::components::body::cards as body_cards;
use body_cards::education_card::EducationCard;

pub struct EducationContainer {}

impl Component for EducationContainer {
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
            <div class="container mt-5" id="educationContainer">
                <div class="page-header text-white">
                    <h1 id="education">{"Education"}</h1>
                </div>
                <EducationCard/>
            </div>
        }
    }

}