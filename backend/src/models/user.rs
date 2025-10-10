use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

impl User {
    fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(&self.password_hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub async fn create(
        pool: &PgPool,
        create_user: CreateUser,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let password_hash = Self::hash_password(&create_user.password)?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, name, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, name, avatar_url, password_hash, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            create_user.email,
            create_user.name,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, name, avatar_url, password_hash, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, name, avatar_url, password_hash, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn authenticate(
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let user = Self::find_by_email(pool, email).await?;

        match user {
            Some(user) if user.verify_password(password)? => Ok(Some(user)),
            _ => Ok(None),
        }
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET name = $2, avatar_url = $3, updated_at = NOW()
            WHERE id = $1
            RETURNING id, email, name, avatar_url, password_hash, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            id,
            name,
            avatar_url
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_password(
        pool: &PgPool,
        id: Uuid,
        new_password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let password_hash = Self::hash_password(new_password)?;

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password_hash = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, email, name, avatar_url, password_hash, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            id,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
