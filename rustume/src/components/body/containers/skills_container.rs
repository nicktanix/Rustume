use yew::prelude::*;
use crate::components::body::cards as body_cards;
use body_cards::skill_card::SkillCard;
use crate::types::SkillItem;

// use anyhow::Error;
// use yew::format::Json;
// use yew::services::fetch::FetchTask;


pub struct SkillsContainer {
    props: Props
}

#[derive(Properties, Clone)]
pub struct Props {
    pub skills: Option<Vec<SkillItem>>,
}

impl Component for SkillsContainer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let skill_cards = match &self.props.skills {
            Some(skills) => {
                html! {
                    <>
                        { for skills.iter().map(|skill| {
                            html! { <SkillCard skill=skill/> }
                        })}
                    </>
                }
            }
            None => {
                // Handle the case where skills is None (empty)
                html! { <div>{"No skills available"}</div> }
            }
        };

        html! {
            <div class="container">
                <div class="row row-cols-1 row-cols-md-2 row-cols-lg-3">
                    { skill_cards }
                </div>
            </div>
        }
    }
}