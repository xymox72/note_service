use note_service::usecase::note_use_case::{create_note, list_notes};
use note_service::domain::note::{Note, NoteRepository};
use async_trait::async_trait;
use tide::Result;


struct MockRepo;

#[async_trait]
impl NoteRepository for MockRepo {
    async fn create(&self, note: Note) -> Result<()> {
        assert_eq!(note.title, "Test title");
        assert_eq!(note.content, "Test content");
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Note>> {
        Ok(vec![
            Note { id: 1, title: "Note 1".into(), content: "Content 1".into() },
            Note { id: 2, title: "Note 2".into(), content: "Content 2".into() },
        ])
    }

    async fn health_check(&self) -> Result<()>{
        Ok(())
    }
}

#[async_std::test]
async fn test_create_note() -> Result<()> {
    let repo = MockRepo;
    create_note(&repo, "Test title".into(), "Test content".into()).await
}

#[async_std::test]
async fn test_list_notes() -> Result<()> {
    let repo = MockRepo;
    let notes = list_notes(&repo).await?;
    assert_eq!(notes.len(), 2);
    Ok(())
}

