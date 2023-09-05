use yew::prelude::*;
use crate::components::body::cards as body_cards;
use body_cards::experience_card::ExperienceCard;
use crate::types::{Experience};


pub struct ExperienceContainer {
    props: Props,
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub experience: Option<Vec<Experience>>,
}


impl Component for ExperienceContainer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let experience_elements: Vec<Html> = self.props.experience.clone().unwrap().iter().map(|experience| {
            html! {
                <ExperienceCard experience=experience.clone()/>
            }
        }).collect();
        html! {
            <div class="container mt-5" id="experienceContainer">
                <div class="page-header text-white">
                    <h1 id="experience">{"Experience"}</h1>
                </div>
                {experience_elements}
            </div>
        }
    }
}