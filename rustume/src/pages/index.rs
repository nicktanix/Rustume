use yew::prelude::*;
use crate::components::body::containers as body_containers;
use crate::types::Resume;
use crate::api;

use body_containers::education_container::EducationContainer;
use body_containers::experience_container::ExperienceContainer;
use body_containers::profile_contact_container::ProfileContactContainer;
use body_containers::summary_container::SummaryContainer;
use body_containers::skills_container::SkillsContainer;


use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;


pub struct State {
    resume: Resume,
    get_resume_error: Option<Error>,
    get_resume_loaded: bool,

}


#[derive(Properties, Clone)]
pub struct Props {
    pub resume: Resume,
}

pub struct Index {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    get_resume_task: Option<FetchTask>,
}

pub enum Msg {
    GetResume,
    GetResumeSuccess(Resume),
    GetResumeError(Error),
}

impl Component for Index {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let resume = Resume::default();

        link.send_message(Msg::GetResume);


        Self {
            props,
            state: State {
                resume,
                get_resume_error: None,
                get_resume_loaded: false,
            },
            link,
            get_resume_task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetResume => {
                self.state.get_resume_loaded = false;
                let handler = self.link.callback(
                    move |response: api::FetchResponse<Resume>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(resume) => Msg::GetResumeSuccess(resume),
                            Err(error) => Msg::GetResumeError(error),
                        }
                    },
                );
                self.get_resume_task = Some(api::get_resume(handler));
                true
            }
            Msg::GetResumeSuccess(resume) => {
                self.state.resume = resume;
                self.state.get_resume_loaded = true;
                true
            }
            Msg::GetResumeError(error) => {
                self.state.get_resume_error = Some(error);
                self.state.get_resume_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if !self.state.get_resume_loaded {
            return html! {
                <div class="container">
                    <div class="spinner-border text-primary" role="status">
                        <span class="visually-hidden">{"Loading..."}</span>
                    </div>
                </div>
            };
        } else if let Some(error) = &self.state.get_resume_error {
            return html! {
                <div class="container">
                    <div class="alert alert-danger" role="alert">
                        {format!("Error: {}", error)}
                    </div>
                </div>
            };
        } else {
            let resume = &self.state.resume;
            let profile = &resume.profile;
            let skills = &resume.skills;
            let experience = &resume.experience;
            let education = &resume.education;
            let summary = &resume.summary;
            html! {
                 <div class="container">
                     <ProfileContactContainer profile={profile}/>
                     <hr/>
                     <SummaryContainer summary={summary}/>
                     <SkillsContainer skills={skills}/>
                     <ExperienceContainer experience={experience}/>
                     <EducationContainer education={education}/>
                     <hr/>
                 </div>
            }
        }
    }
}


