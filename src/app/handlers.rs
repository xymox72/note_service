use crate::domain::note::{NewNote, NoteRepository};
use crate::usecase::note_use_case::{create_note, list_notes};
use tide::{Request, Response, StatusCode};

pub fn with_state<T: NoteRepository>(repo: T) -> T {
    repo
}

pub async fn get_notes<T: NoteRepository>(req: Request<T>) -> tide::Result {
    let repo = req.state();
    let notes = list_notes(repo).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(tide::Body::from_json(&notes)?);
    Ok(res)
}

pub async fn post_note<T: NoteRepository>(mut req: Request<T>) -> tide::Result {
    let new_note: NewNote = req.body_json().await?;
    let repo = req.state();
    create_note(repo, new_note.title, new_note.content).await?;
    Ok(Response::new(StatusCode::Created))
}

pub async fn health_check<T: NoteRepository>(req: Request<T>) -> tide::Result {
    let repo = req.state();
    match repo.health_check().await {
        Ok(_) => Ok(Response::new(StatusCode::Ok)),
        Err(_) => Ok(Response::new(StatusCode::ServiceUnavailable)),
    }
}
