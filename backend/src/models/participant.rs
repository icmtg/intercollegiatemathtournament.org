use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Participant {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub tshirt_size: String,
    pub division: String,
    pub expected_graduation_year: i32,
    pub university: String,
    pub resume_url: Option<String>,
    pub acknowledged_id_requirement: bool,
    pub acknowledged_filming: bool,
    pub acknowledged_team_merge: bool,
    pub interested_in_financial_aid: bool,
    pub additional_data: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateParticipant {
    pub event_id: Uuid,
    pub user_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub tshirt_size: String,
    pub division: String,
    pub expected_graduation_year: i32,
    pub university: String,
    pub resume_url: Option<String>,
    pub acknowledged_id_requirement: bool,
    pub acknowledged_filming: bool,
    pub acknowledged_team_merge: bool,
    pub interested_in_financial_aid: bool,
    pub additional_data: Option<JsonValue>,
}

impl Participant {
    pub async fn create(
        pool: &PgPool,
        create_participant: CreateParticipant,
    ) -> Result<Self, sqlx::Error> {
        let additional_data = create_participant.additional_data.unwrap_or(JsonValue::Object(Default::default()));

        sqlx::query_as!(
            Participant,
            r#"
            INSERT INTO participants (
                event_id, user_id, first_name, last_name, email,
                tshirt_size, division, expected_graduation_year, university, resume_url,
                acknowledged_id_requirement, acknowledged_filming, acknowledged_team_merge,
                interested_in_financial_aid, additional_data
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING id, event_id, user_id, first_name, last_name, email,
                      tshirt_size, division, expected_graduation_year, university, resume_url,
                      acknowledged_id_requirement, acknowledged_filming, acknowledged_team_merge,
                      interested_in_financial_aid, additional_data, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            create_participant.event_id,
            create_participant.user_id,
            create_participant.first_name,
            create_participant.last_name,
            create_participant.email,
            create_participant.tshirt_size,
            create_participant.division,
            create_participant.expected_graduation_year,
            create_participant.university,
            create_participant.resume_url,
            create_participant.acknowledged_id_requirement,
            create_participant.acknowledged_filming,
            create_participant.acknowledged_team_merge,
            create_participant.interested_in_financial_aid,
            additional_data
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT id, event_id, user_id, first_name, last_name, email,
                   tshirt_size, division, expected_graduation_year, university, resume_url,
                   acknowledged_id_requirement, acknowledged_filming, acknowledged_team_merge,
                   interested_in_financial_aid, additional_data, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM participants
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_event(pool: &PgPool, event_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT id, event_id, user_id, first_name, last_name, email,
                   tshirt_size, division, expected_graduation_year, university, resume_url,
                   acknowledged_id_requirement, acknowledged_filming, acknowledged_team_merge,
                   interested_in_financial_aid, additional_data, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM participants
            WHERE event_id = $1
            ORDER BY created_at DESC
            "#,
            event_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_user_and_event(
        pool: &PgPool,
        user_id: Uuid,
        event_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Participant,
            r#"
            SELECT id, event_id, user_id, first_name, last_name, email,
                   tshirt_size, division, expected_graduation_year, university, resume_url,
                   acknowledged_id_requirement, acknowledged_filming, acknowledged_team_merge,
                   interested_in_financial_aid, additional_data, created_at as "created_at: _", updated_at as "updated_at: _"
            FROM participants
            WHERE user_id = $1 AND event_id = $2
            "#,
            user_id,
            event_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM participants
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
