use yew::prelude::*;

pub struct ExperienceCard {}

impl Component for ExperienceCard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ExperienceCard {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card mb-4" id="experienceCard">
                <h2 class="card-header text-white">{"Senior Web Developer "}<span
                        class="badge bg-success float-end">{"Current"}</span>
                </h2>
                <div class="card-body">
                    <div class="card-body">
                        <h4 class="card-title text-white">{"ABC Tech"}</h4>
                        <p class="card-text"><em>{"January 2020 - Present"}</em></p>
                        <div class="row">
                            <div class="col-md-6">
                                <ul class="card-text">
                                    <li>{"Designed and developed responsive websites using HTML, CSS, and
                                        JavaScript."}
                                    </li>
                                    <li>{"Collaborated with cross-functional teams to deliver high-quality web
                                        applications."}
                                    </li>
                                    <li>{"Optimized website performance and improved load times."}</li>
                                </ul>
                            </div>
                            <div class="col-md-6">
                                <ul class="card-text">
                                    <li>{"Designed and developed responsive websites using HTML, CSS, and
                                        JavaScript."}
                                    </li>
                                    <li>{"Collaborated with cross-functional teams to deliver high-quality web
                                        applications."}
                                    </li>
                                    <li>{"Optimized website performance and improved load times."}</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}