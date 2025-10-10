use axum::{Router, extract::State, http::StatusCode, response::Json, routing::get};
use serde_json::{Value, json};
use sqlx::PgPool;
use time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use super::auth;

pub fn create_router(pool: PgPool) -> Router {
    let session_store = PostgresStore::new(pool.clone());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    Router::new()
        .route("/api/health", get(health_check))
        .nest("/api/auth", auth::routes())
        .layer(session_layer)
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
