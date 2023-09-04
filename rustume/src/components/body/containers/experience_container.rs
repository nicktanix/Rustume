use yew::prelude::*;
use crate::components::body::cards as body_cards;
use body_cards::experience_card::ExperienceCard;

pub struct ExperienceContainer {}

impl Component for ExperienceContainer {
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
            <div class="container mt-5" id="experienceContainer">
                <div class="page-header text-white">
                    <h1 id="experience">{"Experience"}</h1>
                </div>
                <ExperienceCard/>
            </div>
        }
    }
}