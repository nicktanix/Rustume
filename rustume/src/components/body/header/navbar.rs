use yew::prelude::*;

pub struct NavBar {}

impl Component for NavBar {
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
            <header>
                <nav class="navbar navbar-expand-lg bg-primary" data-bs-theme="dark">
                    <div class="container">

                        <img class="navbar-brand d-inline-block align-text-top" src="/img/rustume.png" alt="" height="50"/>
                        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav"
                            aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                            <span class="navbar-toggler-icon"></span>
                        </button>
                        <div class="collapse navbar-collapse" id="navbarNav">
                            <ul class="navbar-nav ml-auto">
                                <li class="nav-item">
                                    <a class="nav-link" href="#summary">{"Summary"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link" href="#skills">{"Skills"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link" href="#experience">{"Experience"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link" href="#education">{"Education"}</a>
                                </li>
                            </ul>
                            <ul class="navbar-nav ms-md-auto">
                                <li class="nav-item dropdown">
                                    <a class="nav-link dropdown-toggle" data-bs-toggle="dropdown" href="#" role="button"
                                        aria-haspopup="true" aria-expanded="true">{"Download"}</a>
                                    <div class="dropdown-menu" data-bs-popper="static">
                                        <a class="dropdown-item" href="#">{"PDF"}</a>
                                        <a class="dropdown-item" href="#">{"Word"}</a>
                                        <a class="dropdown-item" href="#">{"Raw Text"}</a>
                                        <div class="dropdown-divider"></div>
                                        <a class="dropdown-item" href="#">{"YAML"}</a>
                                        <a class="dropdown-item" href="#">{"JSON"}</a>
                                    </div>
                                </li>
                            </ul>

                        </div>
                    </div>
                </nav>
            </header>
        }
    }
}
