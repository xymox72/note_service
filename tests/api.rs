use note_service::app::handlers::{get_notes, post_note, with_state};
use note_service::infra::note_repo_pg::PgNoteRepository;
use sqlx::postgres::PgPoolOptions;
use tide::{http::{Method, Request, Url}, Server};
use tide::Response;
use serde_json::json;



async fn setup_app() -> Server<PgNoteRepository> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // Очистка базы перед тестом
    sqlx::query("DELETE FROM notes").execute(&pool).await.unwrap();

    let state = with_state(pool);
    let mut app = tide::with_state(state);

    app.at("/notes")
        .post(post_note)
        .get(get_notes);

    app
}

#[async_std::test]
async fn test_create_and_list_notes() {
    let  app = setup_app().await;

    // POST /notes
    let mut post_req = Request::new(
        Method::Post,
        Url::parse("http://localhost/notes").unwrap(),
    );
    post_req.set_body(json!({
        "title": "Integration Note",
        "content": "This is from test"
    }));

    let post_res: Response = app.respond(post_req).await.unwrap();
    assert_eq!(post_res.status(), 201);

    // GET /notes
    let get_req = Request::new(
        Method::Get,
        Url::parse("http://localhost/notes").unwrap(),
    );
    let mut get_res: Response = app.respond(get_req).await.unwrap();
    assert_eq!(get_res.status(), 200);

    let notes: Vec<note_service::domain::note::Note> = get_res.take_body().into_json().await.unwrap();
    assert!(
        !notes.is_empty(),
        "Expected at least one note in response"
    );
    assert!(
        notes.iter().any(|n| n.title == "Integration Note"),
        "Note with title 'Integration Note' not found in response: {:#?}",
        notes
    );
}

#[async_std::test]
async fn test_create_note_invalid_payload() {
    let  app = setup_app().await;


    let mut post_req = Request::new(
        Method::Post,
        Url::parse("http://localhost/notes").unwrap(),
    );
    post_req.set_body(json!({
        "content": "Missing title"
    }));

    let post_res: Response = app.respond(post_req).await.unwrap();
    assert_eq!(post_res.status(), 422, "Expected 422 Unprocessable Entity, got: {}", post_res.status());
}  
