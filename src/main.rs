mod app;
mod config;
mod domain;
mod infra;
mod usecase;

use app::handlers::{get_notes, health_check, post_note, with_state};
use dotenvy::dotenv;
use infra::note_repo_pg::PgNoteRepository;
use sqlx::postgres::PgPoolOptions;
use tide::Server;

fn main() -> tide::Result<()> {
    async_std::task::block_on(async {
        dotenv().ok();
        tide::log::start();

        let database_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        let repo = PgNoteRepository::new(pool);
        let mut app: Server<_> = tide::with_state(with_state(repo));

        app.at("/notes").get(get_notes);
        app.at("/notes").post(post_note);
        app.at("/health").get(health_check);
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let addr = format!("0.0.0.0:{}", port);
        app.listen(addr).await?;
        Ok(())
    })
}
