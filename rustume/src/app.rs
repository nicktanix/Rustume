use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Index};
use crate::types::Resume;
use crate::components::body::header::NavBar;
use crate::components::body::footer::Footer;
use crate::routes::Route;


pub struct State {
    resume: Resume,
}


pub struct App {
    state: State,

}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let resume = Resume::default();
        Self {
            state: State {
                resume,
            },
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let resume = self.state.resume.clone();
        let render = Router::render(move |switch: Route| match switch {
            Route::Index => html! { <Index resume=resume.clone()/> },
        });

        html! {

                <>
                <NavBar/>
                <Router<Route, ()> render=render/>
                <Footer/>
                </>
        }
    }
}