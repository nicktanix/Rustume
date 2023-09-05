use yew::prelude::*;

use crate::components::body::cards::profile_card::ProfileCard;
use crate::components::body::cards::contact_card::ContactCard;
use crate::types::Profile;

pub struct ProfileContactContainer {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub profile: Profile,
}


impl Component for ProfileContactContainer {
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
        let profile = &self.props.profile;
        let location = &self.props.profile.contact.location;
        html! { 
            <div class="row mt-5" id="profileContactContainer">
                <ProfileCard information={&profile.information}, location={location}/>
                <ContactCard contact={&profile.contact}/>
            </div>
         }
    }
}