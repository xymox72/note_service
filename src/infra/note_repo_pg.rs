use crate::domain::note::{Note, NoteRepository};
use sqlx::PgPool;
use tide::utils::async_trait;

#[derive(Clone)]
pub struct PgNoteRepository {
    pool: PgPool,
}

impl PgNoteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NoteRepository for PgNoteRepository {
    async fn create(&self, note: Note) -> tide::Result<()> {
        sqlx::query("INSERT INTO notes (title, content) VALUES ($1, $2)")
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
