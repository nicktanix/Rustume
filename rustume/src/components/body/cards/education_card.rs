use yew::prelude::*;
use crate::types::Education;


pub struct EducationCard {
    props: Props,
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub education: Education,
}

impl Component for EducationCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        EducationCard {
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
        let education = self.props.education.clone();
        let current: bool = education.graduated == false && education.end_date.is_empty();
        html! {
            <div class="card mb-4" id="educationCard">
                <h2 class="card-header text-white">{education.institution}
                {if current {
                    html! {<span class="badge bg-success float-end">{"Current"}</span>}
                } else if education.graduated{
                    html! {<span class="badge bg-success float-end">{format!("Graduated {}", education.graduation_date)}</span>}
                } else {
                    html! {}
                }}
                </h2>
                <div class="card-body">
                    <div class="card-body">
                        <h4 class="card-title text-white">{education.degree}</h4>
                        {if current {
                            html! {
                                <h5 class="card-subtitle mb-2 text-muted">{"Expected Graduation: "} {education.graduation_date}</h5>
                            }
                        } else if education.graduated{
                            html! {
                                <h5 class="card-subtitle mb-2 text-muted">{"Graduated: "} {education.graduation_date}</h5>
                            }
                        } else {
                            html! {
                                <p class="card-text"><em>{format!("{} - {}", education.start_date, education.end_date)}</em></p>
                        
                            }
                        }}
                        <div class="row">
                            <div class="col">
                                <ul class="card-text" style="column-count: 2;">
                                    {for education.highlights.iter().map(|highlight| {
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
