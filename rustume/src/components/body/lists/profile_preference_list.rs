use yew::prelude::*;

use std::collections::HashMap;

use crate::types::Preferences;


pub struct ProfilePreferenceList {
    props: Props,
}


#[derive(Properties, Clone)]
pub struct Props {
    pub preferences: Preferences,
    pub trajectory: Option<Vec<String>>,
}


impl Component for ProfilePreferenceList {
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
        let preferences = &self.props.preferences.clone().into_iter();
        let preference_map: HashMap<String, Option<Vec<String>>> = preferences.clone().collect();
        let trajectory = &self.props.trajectory.clone().unwrap();
        //add trajectory to the preference map
        
        let prefences_elements: Vec<Html> = preference_map.iter().map(|(key, value)| {
            let items: Vec<Html> = value.clone().unwrap().iter().map(|item| {
                html! {
                    <span class="badge rounded-pill bg-info float-end" style="min-width: 5rem;">{item}</span>
                }
            }).collect();
            // Title Case the key
            let key = key.split('_').map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }).collect::<Vec<String>>().join(" ");
            html! {
                <li class="list-group-item">{key}
                    {items}
                </li>
            }
        }).collect();
        let trajectory_items: Vec<Html> = trajectory.iter().map(|item| {
            html! {
                <span class="badge rounded-pill bg-success float-end" style="min-width: 5rem;">{item}</span>
            }
        }).collect();
        html! {
            <ul class="list-group list-group-flush">
                <li class="list-group-item">{"Current Trajectory"}
                    {trajectory_items}
                </li>
                {prefences_elements}
            </ul>
        }
    }
}