use yew::prelude::*;

pub struct SummaryCard {
    props: Props,
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub summary: String,
}

impl Component for SummaryCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card mb-4" id="summaryCard">
                <div class="card-body">
                    <p class="card-text">
                        {&self.props.summary}
                    </p>
                </div>
            </div>
        }
    }
}