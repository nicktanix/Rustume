use yew::prelude::*;
use crate::types::Experience;

pub struct ExperienceCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub experience: Experience,
}

impl Component for ExperienceCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ExperienceCard {
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
        let experience = &self.props.experience;
        //if endata is None, or empty string then current is true
        let current = match &experience.end_date {
            Some(end_date) => {
                if end_date.is_empty() {
                    true
                } else {
                    false
                }
            }
            None => true,
        };
        let binding = String::new();
        macro_rules! format_str {
            ($x:expr) => {
                match $x {
                    Some(s) => s.to_owned(), // Clone the String in the Some arm
                    None => binding.clone(), // Clone the binding String in the None arm
                }
            };
        }
        html! {
            <div class="card mb-4" id="experienceCard">
                <h2 class="card-header text-white">{format_str!(&experience.title)}
                {if current {
                    html! {<span class="badge bg-success float-end">{"Current"}</span>}
                } else {
                    html! {}
                }}  
                </h2>
                <div class="card-body">
                    <div class="card-body">
                        <h4 class="card-title text-white">{format_str!(&experience.company)}</h4>
                        {if current {
                            html! {
                                <p class="card-text">
                                    <em>
                                        {format!("{} - {}", format_str!(&experience.start_date), "Present")}
                                    </em>
                                </p>
                            }
                        } else {
                            html!{
                                <p class="card-text">
                                    <em>
                                        {format!("{} - {}", format_str!(&experience.start_date), format_str!(&experience.end_date))}
                                    </em>
                                </p>
                                }
                            }
                        }
                        <div class="row">
                            <div class="col">
                                <ul class="card-text" style="column-count: 2;">
                                    {for experience.highlights.iter().map(|highlight| {
                                        html! {
                                            <li>{highlight}</li>
                                            }
                                        })
                                    }
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}