use axum::{Router, extract::State, http::StatusCode, response::Json, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

use super::error::{ApiError, ApiResult};
use crate::models::{CreateUser, User};

const SESSION_USER_ID_KEY: &str = "user_id";

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: Uuid,
    email: String,
    name: Option<String>,
    avatar_url: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            avatar_url: user.avatar_url,
        }
    }
}

async fn register(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<RegisterRequest>,
) -> ApiResult<Json<Value>> {
    if User::find_by_email(&pool, &payload.email).await?.is_some() {
        return Err(ApiError::UserExists);
    }

    let create_user = CreateUser {
        email: payload.email,
        password: payload.password,
        name: payload.name,
    };

    let user = User::create(&pool, create_user).await?;

    session.insert(SESSION_USER_ID_KEY, user.id).await?;

    let user_response: UserResponse = user.into();
    Ok(Json(json!({
        "user": user_response
    })))
}

async fn login(
    State(pool): State<PgPool>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> ApiResult<Json<Value>> {
    let user = User::authenticate(&pool, &payload.email, &payload.password)
        .await?
        .ok_or(ApiError::InvalidCredentials)?;

    session.insert(SESSION_USER_ID_KEY, user.id).await?;

    let user_response: UserResponse = user.into();
    Ok(Json(json!({
        "user": user_response
    })))
}

async fn logout(session: Session) -> ApiResult<StatusCode> {
    session.delete().await?;
    Ok(StatusCode::OK)
}
