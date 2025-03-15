use tide::{Request, Response, StatusCode};
use crate::infra::note_repo_pg::PgNoteRepository;
use crate::usecase::note_use_case;
use crate::domain::note::{NewNote, NoteRepository};
use sqlx::PgPool;

pub fn with_state(pool: PgPool) -> PgNoteRepository {
    PgNoteRepository::new(pool)
}

pub async fn get_notes(req: Request<PgNoteRepository>) -> tide::Result {
    let repo = req.state();
    let notes = note_use_case::list_notes(repo).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(tide::Body::from_json(&notes)?);
    Ok(res)
}


pub async fn post_note(mut req: Request<PgNoteRepository>) -> tide::Result {
    let new_note: NewNote = req.body_json().await?;
    let repo = req.state();
    note_use_case::create_note(repo, new_note.title, new_note.content).await?;
    Ok(Response::new(StatusCode::Created))
}

pub async fn health_check(req: Request<PgNoteRepository>) -> tide::Result {
    let repo = req.state();
    match repo.health_check().await {
        Ok(_) => Ok(Response::new(StatusCode::Ok)),
        Err(_) => Ok(Response::new(StatusCode::ServiceUnavailable)),
    }
}
