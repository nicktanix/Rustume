use yew::prelude::*;
use crate::types::{Information, Location};
use crate::components::body::lists::profile_preference_list::ProfilePreferenceList;


pub struct ProfileCard {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub information: Information,
    pub location: Location
}

impl Component for ProfileCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let information = &self.props.information;
        let preferences = &self.props.information.preferences;
        let trajectory = &self.props.information.current_trajectory;

        // macro that takes a Option<String> and returns a String
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
            <div class="col-lg-6 mh-100" id="profileCard">
                <div class="card mb-6" id="whoAmICard">
                    <h3 class="card-header text-white">{format_str!(&information.name)}</h3>
                    <div class="card-body">
                        <h6 class="card-subtitle text-muted">{format_str!(&information.current_job_title)}</h6>
                    </div>
                    <img src={format_str!(&information.photo)} class="d-block user-select-none" width="auto" height="200"
                        focusable="false" role="img" alt="..."/>
                    <div class="card-body">
                        <p class="card-text">{format_str!(&information.message)}</p>
                    </div>
                        <ProfilePreferenceList preferences={preferences.clone()} trajectory={trajectory.clone()} />
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