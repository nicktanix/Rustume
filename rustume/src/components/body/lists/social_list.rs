use yew::prelude::*;
use crate::types::Contact;
use std::collections::HashMap;

pub struct SocialList {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub contact: Contact,
}


impl Component for SocialList {
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
        let social_list = self.props.contact.social.clone().into_iter();
        // add email, website and phone to the social list
        let mut social_map: HashMap<String, Option<String>> = HashMap::new();
        social_map.insert("Envelope".to_string(), self.props.contact.email.clone());
        social_map.insert("Desktop".to_string(), self.props.contact.website.clone());
        social_map.insert("phone".to_string(), self.props.contact.phone.clone());
        social_map.extend(social_list);

        let social_links: Vec<Html> = social_map.iter().map(|(key, value)| {
        let icon: String = key.to_lowercase();
        let url: String = value.clone().unwrap();
            html! {
               <a href={format!("{}", url)} class="list-group-item list-group-item-action">
                    {format!("{}", url)}
                    <i class=format!("icon-{} float-end", icon)></i>
                </a>
            }
        }).collect();
        html! {
            <div class="list-group list-group-flush mh-100" style="overflow-y: scroll;">
                {social_links}
            </div>
            }
    }
}