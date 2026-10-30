use axum::{Json, Router, extract::State, http::StatusCode, response::Redirect, routing::get};
use serde::Serialize;
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn mascarade_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

#[derive(Serialize, FromRow)]
struct Score {
    character: String,
    player: String,
    score: Option<i32>,
    guild: i32,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let pool = mascarade_pool().await;

    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/list") }))
        .route("/list", get(list_chars))
        .with_state(pool);

    let listener =
        tokio::net::TcpListener::bind(env::var("CONNECT_URL").expect("CONNECT URL must be set"))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_chars(State(pool): State<PgPool>) -> Result<Json<Vec<Score>>, StatusCode> {
    let scores =
        sqlx::query_as::<_, Score>("SELECT character, player, score, guild FROM characters")
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                tracing::error!("query failed: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    Ok(Json(scores))
}
