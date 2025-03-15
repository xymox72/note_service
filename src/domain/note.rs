use serde::{Deserialize, Serialize};
use tide::utils::async_trait;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct NewNote{
    pub title: String,
    pub content: String,
}

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn create(&self, note: Note) -> tide::Result<()>;
    async fn list(&self) -> tide::Result<Vec<Note>>;
    async fn health_check(&self) -> tide::Result<()>;
}
