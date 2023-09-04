use crate::types::SkillItem;

// use anyhow::Error;
// use yew::format::Json;
use yew::prelude::*;
// use yew::services::fetch::FetchTask;


pub struct SkillCard {
    props: Props
}


#[derive(Properties, Clone)]
pub struct Props {
    pub skill: SkillItem,
}


impl Component for SkillCard {
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
        let skills =  match &self.props.skill.skill_items {
            Some(skills) => {
                html!  {
                    <>
                        { for skills.iter().map(|skill| {
                            html! { <span class="badge rounded-pill bg-info" style="min-width: 5rem;">{skill}</span> }
                        })}
                    </>
                }
            }
            None => {
                // Handle the case where skills is None (empty)
                html! { <></> }
            }
        };

        html! {
            <div class="col mb-4" id=format!("{}SkillsContainer", self.props.skill.skill_type_camel_case())>
                <div class="card" style="min-height: 5rem;" id="skill_card">
                    <div class="card-body">
                        <h5 class="card-title text-white">{&self.props.skill.skill_type}</h5>
                        <div class="card-text">
                            { skills }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}