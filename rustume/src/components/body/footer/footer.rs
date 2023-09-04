use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
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
               <footer class="bg-dark text-center py-3 d-flex justify-content-between">  
                    <div class="flex-column ms-5">
                        <p class="text-white-50">{"Made with "}<i class="icon-heart"></i>{" by John Doe"}</p>
                        <p class="text-white-50">{"All wrongs reserved."}</p>
                    </div>
                    <div class="flex-column">
                        <figure class="text-center">
                            <blockquote class="blockquote">
                                <p class="mb-0">{"Any sufficiently advanced technology is indistinguishable from magic."}</p>
                            </blockquote>
                            <figcaption class="blockquote-footer">
                               {" Arthur C. Clarke "}<a href="https://en.wikipedia.org/wiki/Clarke%27s_three_laws"><cite
                                        title="Source Title">{"Clarke's Third Law"}</cite></a>
                            </figcaption>
                        </figure>
                    </div>
                    <div class="flex-column me-5">
                        <p class="text-white-50">{"Source code available on "}<a href="">{"GitHub"}</a></p>
                        <p class="text-white-50">{"Licensed under GNU GPLv3"}</p>
                    </div>
                </footer>
        }
    }
}
