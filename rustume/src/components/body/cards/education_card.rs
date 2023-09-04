use yew::prelude::*;

pub struct EducationCard {}

impl Component for EducationCard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        EducationCard {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card mb-4" id="educationCard">
                <h2 class="card-header text-white">{"University of Wikipedia and Websearch"}<span
                        class="badge bg-success float-end">{"Graduated
                        2012"}</span>
                </h2>
                <div class="card-body">
                    <div class="card-body">
                        <h4 class="card-title text-white">{"Associates in Weeb Engineering"}</h4>
                        <p class="card-text"><em>{"2008 - 2012"}</em></p>
                        <div class="row">
                            <div class="col-md-6">
                                <ul class="card-text">
                                    <li>{"Graduated top 5% of class"}
                                    </li>
                                    <li>{"Cuma Sum Laude."}</li>
                                    <li>{"Deans List."}</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
