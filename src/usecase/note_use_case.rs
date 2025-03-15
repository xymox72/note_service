use crate::domain::note::{Note, NoteRepository};

pub async fn create_note<R: NoteRepository>(repo: &R, title: String, content: String) -> tide::Result<()> {
    let note = Note {
        id: 0, 
        title,
        content,
    };
    repo.create(note).await
}

pub async fn list_notes<R: NoteRepository>(repo: &R) -> tide::Result<Vec<Note>> {
    let notes =repo.list().await?;

    Ok(notes)
}