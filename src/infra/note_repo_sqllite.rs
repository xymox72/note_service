use tide::utils::async_trait;

use crate::domain::note::{Note, NoteRepository};
#[derive(Clone)]
pub struct SqliteNoteRepository {
    pub pool: sqlx::SqlitePool,
}

impl SqliteNoteRepository {
    #![allow(dead_code)]
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NoteRepository for SqliteNoteRepository {
    async fn create(&self, note: Note) -> tide::Result<()> {
        sqlx::query("INSERT INTO notes (title, content) VALUES (?, ?)")
            .bind(&note.title)
            .bind(&note.content)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn list(&self) -> tide::Result<Vec<Note>> {
        let notes = sqlx::query_as::<_, Note>("SELECT id, title, content FROM notes")
            .fetch_all(&self.pool)
            .await?;
        Ok(notes)
    }

    async fn health_check(&self) -> tide::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
