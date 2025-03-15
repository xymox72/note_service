use note_service::app::handlers::{get_notes, post_note};
use note_service::domain::note::Note;
use note_service::infra::note_repo_sqllite::SqliteNoteRepository;
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use tide::Response;
use tide::{
    Server,
    http::{Method, Request, Url},
};

async fn setup_app() -> Server<SqliteNoteRepository> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(":memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, content TEXT NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();

    let repo = SqliteNoteRepository::new(pool);
    let mut app = tide::with_state(repo);

    app.at("/notes").get(get_notes);
    app.at("/notes").post(post_note);

    app
}

#[async_std::test]
async fn test_create_and_list_notes() {
    let app = setup_app().await;

    // POST /notes
    let mut post_req = Request::new(Method::Post, Url::parse("http://localhost/notes").unwrap());
    post_req.set_body(json!({
        "title": "Integration Note",
        "content": "This is from test"
    }));

    let post_res: tide::Response = app.respond(post_req).await.unwrap();
    assert_eq!(post_res.status(), 201);

    // GET /notes
    let get_req = Request::new(Method::Get, Url::parse("http://localhost/notes").unwrap());
    let mut get_res: tide::Response = app.respond(get_req).await.unwrap();
    assert_eq!(get_res.status(), 200);

    let notes: Vec<Note> = get_res.take_body().into_json().await.unwrap();
    assert!(notes.iter().any(|n| n.title == "Integration Note"));
}

#[async_std::test]
async fn test_create_note_invalid_payload() {
    let app = setup_app().await;

    let mut post_req = Request::new(Method::Post, Url::parse("http://localhost/notes").unwrap());
    post_req.set_body(json!({
        "content": "Missing title"
    }));

    let post_res: Response = app.respond(post_req).await.unwrap();
    assert_eq!(
        post_res.status(),
        422,
        "Expected 422 Unprocessable Entity, got: {}",
        post_res.status()
    );
}
