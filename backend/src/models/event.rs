use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub registration_open: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateEvent {
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub registration_open: bool,
}

impl Event {
    pub async fn create(
        pool: &PgPool,
        create_event: CreateEvent,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Event,
            r#"
            INSERT INTO events (name, description, location, start_date, end_date, registration_open)
            VALUES ($1, $2, $3, $4::timestamptz, $5::timestamptz, $6)
            RETURNING id, name, description, location, start_date as "start_date?: _", end_date as "end_date?: _", registration_open, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            create_event.name,
            create_event.description,
            create_event.location,
            create_event.start_date as _,
            create_event.end_date as _,
            create_event.registration_open
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Event,
            r#"
            SELECT id, name, description, location, start_date as "start_date?: _", end_date as "end_date?: _", registration_open, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM events
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn list_open_registrations(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Event,
            r#"
            SELECT id, name, description, location, start_date as "start_date?: _", end_date as "end_date?: _", registration_open, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM events
            WHERE registration_open = true
            ORDER BY start_date DESC NULLS LAST
            "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: String,
        description: Option<String>,
        location: Option<String>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        registration_open: bool,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Event,
            r#"
            UPDATE events
            SET name = $2, description = $3, location = $4, start_date = $5::timestamptz, end_date = $6::timestamptz, registration_open = $7, updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, description, location, start_date as "start_date?: _", end_date as "end_date?: _", registration_open, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            id,
            name,
            description,
            location,
            start_date as _,
            end_date as _,
            registration_open
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM events
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
