use yew::prelude::*;

pub struct ProfileCard {}

impl Component for ProfileCard {
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
            <div class="col-lg-6 mh-100" id="profileCard">
                <div class="card mb-6" id="whoAmICard">
                    <h3 class="card-header text-white">{"John Doe"}</h3>
                    <div class="card-body">
                        <h6 class="card-subtitle text-muted">{"Senior Web Developer"}</h6>
                    </div>
                    <img src="/img/coder.webp" class="d-block user-select-none" width="auto" height="200"
                        focusable="false" role="img" alt="..."/>
                    <div class="card-body">
                        <p class="card-text">{"Some quick example text to build on the card title and make up the bulk of
                            the card's
                            content."}</p>
                    </div>
                    <ul class="list-group list-group-flush">
                        <li class="list-group-item">{"Current Trajectory"}
                            <span class="badge bg-warning rounded-pill float-end">{"Searching"}</span>
                            <span class="badge bg-success rounded-pill float-end">{"Not Searching"}</span>
                            <span class="badge bg-warning-subtle rounded-pill float-end text-dark">{"Open to Discuss"}
                            </span>
                        </li>
                        <li class="list-group-item">{"Location Preference"}
                            <span class="badge bg-light rounded-pill float-end">{"Remote"}</span>
                            <span class="badge bg-light rounded-pill float-end">{"Able to relocate"}</span>
                            <span class="badge bg-light rounded-pill float-end">{"In Office: Phoenix, AZ"}</span>
                        </li>
                        <li class="list-group-item">{"Term Preference"}
                            <span class="badge bg-light rounded-pill float-end">{"Full Time"}</span>
                            <span class="badge bg-light rounded-pill float-end">{"W2"}</span>
                            <span class="badge bg-light rounded-pill float-end">{"C2C"}</span>
                            <span class="badge bg-light rounded-pill float-end">{"C2H"}</span>

                        </li>
                        <li class="list-group-item">{"Position Preference"}
                            <span class="badge bg-light rounded-pill float-end">{"Software Engineer"}</span>
                            <span class="badge bg-light rounded-pill float-end">{"Frontend Engineer"}</span>
                            <span class="badge bg-light rounded-pill float-end">{"Backend Engineer"}</span>
                        </li>
                    </ul>
                    <div class="card-body">
                        <a href="#" class="card-link">{"Card link"}</a>
                        <a href="#" class="card-link">{"Another link"}</a>
                    </div>
                    <div class="card-footer text-muted">
                        {"Updated: 2 days ago"}
                    </div>
                </div>
            </div>
         }
    }
}