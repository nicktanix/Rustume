use yew::prelude::*;

use crate::components::body::cards::profile_card::ProfileCard;
use crate::components::body::cards::contact_card::ContactCard;

pub struct ProfileContactContainer {}

impl Component for ProfileContactContainer {
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
            <div class="row mt-5" id="profileContactContainer">
                <ProfileCard/>
                <ContactCard/>
            </div>
         }
    }
}