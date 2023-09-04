use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Index};
use crate::components::body::header::NavBar;
use crate::components::body::footer::Footer;
use crate::routes::Route;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::Index => html! { <Index/> },
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