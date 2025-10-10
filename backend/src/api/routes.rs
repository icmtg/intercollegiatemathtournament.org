use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(pool)
}

async fn health_check(State(pool): State<PgPool>) -> Result<Json<Value>, StatusCode> {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => Ok(Json(json!({
            "status": "ok",
            "database": "connected"
        }))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
