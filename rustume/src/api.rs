use crate::types::Resume;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_resume(callback: FetchCallback<Resume>) -> FetchTask {
    let req = Request::get("/json/resume/resume.json")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}
