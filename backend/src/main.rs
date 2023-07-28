use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Form, Json, Router,
};
use axum_error::Result;
mod models;
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;
#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;
    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create_todo))
        .route("/delete/:id", post(delete))
        .route("/update", post(update_todo))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address = SocketAddr::from(([0, 0, 0, 0], 3458));

    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<models::Todo>>> {
    let todos = sqlx::query_as!(models::Todo, "SELECT * FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;

    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<SqlitePool>,
    Form(todo): Form<models::NewTodo>,
) -> Result<Redirect> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect> {
    sqlx::query!("DELETE FROM todos WHERE id= ?", id)
        .execute(&pool)
        .await?;

    Ok(Redirect::to("http://localhost:5173"))
}

async fn update_todo(
    State(pool): State<SqlitePool>,
    Form(todo): Form<models::Todo>,
) -> Result<Redirect> {
    sqlx::query!(
        "UPDATE todos SET description = ? , done = ? WHERE id = ? ",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}
