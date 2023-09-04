use yew::prelude::*;

pub struct SkillCard {}

impl Component for SkillCard {
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
            <div class="col mb-4" id="skill_card_container">
                <div class="card" style="min-height: 5rem;" id="skill_card">
                    <div class="card-body">
                        <h5 class="card-title text-white">{"Programming Languages"}</h5>
                        <div class="card-text">
                            <span class="badge rounded-pill bg-info" style="min-width: 5rem;">{"Python"}</span>
                            <span class="badge rounded-pill bg-info" style="min-width: 5rem;">{"JavaScript"}</span>
                            <span class="badge rounded-pill bg-info" style="min-width: 5rem;">{"Rust"}</span>
                            <span class="badge rounded-pill bg-info" style="min-width: 5rem;">{"Powershell"}</span>
                            <span class="badge rounded-pill bg-info" style="min-width: 5rem;">{"Go"}</span>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}