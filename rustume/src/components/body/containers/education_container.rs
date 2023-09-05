use yew::prelude::*;
use crate::components::body::cards as body_cards;
use body_cards::education_card::EducationCard;
use crate::types::Education;


pub struct EducationContainer {
    props: Props,
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub education: Option<Vec<Education>>,
}


impl Component for EducationContainer {
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
        let education_elements: Vec<Html> = self.props.education.clone().unwrap().iter().map(|education| {
            html! {
                <EducationCard education=education.clone()/>
            }
        }).collect();
        html! {
            <div class="container mt-5" id="educationContainer">
                <div class="page-header text-white">
                    <h1 id="education">{"Education"}</h1>
                </div>
                {education_elements}
            </div>
        }
    }

}