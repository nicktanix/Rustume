use yew::prelude::*;
use crate::components::body::cards as body_cards;
use body_cards::skill_card::SkillCard;

pub struct SkillsContainer {}

impl Component for SkillsContainer {
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
            <div class="container mt-5" id="skillsContainer">
                    <div class="page-header text-white">
                        <h1 id="skills">{"Skills"}</h1>
                    </div>
                    <div class="container">
                        <div class="row row-cols-1 row-cols-md-2 row-cols-lg-3">
                            <SkillCard/>
                            <SkillCard/>
                            <SkillCard/>
                            <SkillCard/>
                            <SkillCard/>
                            <SkillCard/>
                        </div>
                    </div>
                </div>
        }
    }
}