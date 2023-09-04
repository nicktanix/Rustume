use yew::prelude::*;
use crate::components::body::containers as body_containers;

use body_containers::education_container::EducationContainer;
use body_containers::experience_container::ExperienceContainer;
use body_containers::profile_contact_container::ProfileContactContainer;
use body_containers::summary_container::SummaryContainer;
use body_containers::skills_container::SkillsContainer;
pub struct Index {}

pub enum Msg {}

impl Component for Index {
    type Message = Msg;
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
            <div class="container">
                <ProfileContactContainer/>
                <hr/>
                <SummaryContainer/>
                <SkillsContainer/>
                <ExperienceContainer/>
                <EducationContainer/>
                <hr/>
            </div>
       }
    }
}


