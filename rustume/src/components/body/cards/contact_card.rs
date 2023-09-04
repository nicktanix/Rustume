use yew::prelude::*;

pub struct ContactCard {}

impl Component for ContactCard {
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
            <div class="col-lg-6 mh-100" id="contactCard">
                <div class="card lg-6 h-100">
                    <h3 class="card-header text-white">{"Contact Me"}</h3>
                    <div class="card-body">
                        <p class="card-text">{"If you have a job description or position and would like to reach out, send
                            me and email or use the contact method below that works best for you."}</p>
                        <div class="accordion" id="accordionContact">
                            <div class="accordion-item">
                                <h2 class="accordion-header" id="contactForm">
                                    <button class="accordion-button" type="button" data-bs-toggle="collapse"
                                        data-bs-target="#collapseContactForm" aria-expanded="true"
                                        aria-controls="collapseContactForm">
                                        {"Contact Form"}
                                    </button>
                                </h2>
                                <div id="collapseContactForm" class="accordion-collapse collapse"
                                    aria-labelledby="contactForm" data-bs-parent="#accordionContact" style="">
                                    <div class="accordion-body">
                                        <form action="#" method="post" enctype="multipart/form-data">
                                            <div class="form-group">
                                                <label for="subject">{"Subject:"}</label>
                                                <input type="text" class="form-control" id="subject" name="subject"
                                                    required=true/>
                                            </div>
                                            <div class="form-group">
                                                <label for="senderEmail">{"Sender Email:"}</label>
                                                <input type="email" class="form-control" id="senderEmail"
                                                    name="senderEmail" required=true/>
                                            </div>
                                            <div class="form-group">
                                                <label for="jobListingUrl">{"Job Listing (URL):"}</label>
                                                <input type="url" class="form-control" id="jobListingUrl"
                                                    name="jobListingUrl" required=true/>
                                            </div>
                                            <div class="form-group">
                                                <label for="message">{"Message Body:"}</label>
                                                <textarea class="form-control" id="message" name="message" rows="4"
                                                    required=true></textarea>
                                            </div>
                                            <div class="form-group">
                                                <label for="formFile" class="form-label mt-4">{"Default file input
                                                    example"}</label>
                                                <input class="form-control" type="file" id="formFile"/>
                                            </div>
                                            <button type="submit" class="btn btn-primary">{"Submit"}</button>
                                        </form>
                                    </div>
                                </div>
                            </div>
                            <div class="accordion-item">
                                <h2 class="accordion-header" id="contactInfo">
                                    <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse"
                                        data-bs-target="#collaspeContactInfo" aria-expanded="false"
                                        aria-controls="collaspeContactInfo">
                                        {"Contact Info"}
                                    </button>
                                </h2>
                                <div id="collaspeContactInfo" class="accordion-collapse collapse show"
                                    aria-labelledby="contactInfo" data-bs-parent="#accordionContact">
                                    <div class="accordion-body" style="overflow-y: scroll; max-height: 350px">
                                        <div class="list-group list-group-flush mh-100" style="overflow-y: scroll;">
                                            <a href="#" class="list-group-item list-group-item-action  border-info">
                                                {"www.johndoe.com"}
                                                <span class="badge bg-primary rounded-pill float-end"><i
                                                        class="icon-desktop"></i></span>
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
         }
    }
}