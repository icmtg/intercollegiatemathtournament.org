use axum::{Router, extract::{State, Path}, response::Json, routing::{get, post}};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use super::error::{ApiError, ApiResult};
use crate::models::{CreateParticipant, Event, Participant};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/events", get(list_events))
        .route("/events/:event_id/register", post(register_participant))
        .route("/events/:event_id/participants", get(list_participants))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterParticipantRequest {
    first_name: String,
    last_name: String,
    email: String,
    tshirt_size: String,
    division: String,
    expected_graduation_year: i32,
    university: String,
    resume_url: Option<String>,
    acknowledged_id_requirement: bool,
    acknowledged_filming: bool,
    acknowledged_team_merge: bool,
    interested_in_financial_aid: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ParticipantResponse {
    id: Uuid,
    event_id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    tshirt_size: String,
    division: String,
    expected_graduation_year: i32,
    university: String,
    resume_url: Option<String>,
    acknowledged_id_requirement: bool,
    acknowledged_filming: bool,
    acknowledged_team_merge: bool,
    interested_in_financial_aid: bool,
}

impl From<Participant> for ParticipantResponse {
    fn from(participant: Participant) -> Self {
        Self {
            id: participant.id,
            event_id: participant.event_id,
            first_name: participant.first_name,
            last_name: participant.last_name,
            email: participant.email,
            tshirt_size: participant.tshirt_size,
            division: participant.division,
            expected_graduation_year: participant.expected_graduation_year,
            university: participant.university,
            resume_url: participant.resume_url,
            acknowledged_id_requirement: participant.acknowledged_id_requirement,
            acknowledged_filming: participant.acknowledged_filming,
            acknowledged_team_merge: participant.acknowledged_team_merge,
            interested_in_financial_aid: participant.interested_in_financial_aid,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EventResponse {
    id: Uuid,
    name: String,
    description: Option<String>,
    location: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    registration_open: bool,
}

impl From<Event> for EventResponse {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            name: event.name,
            description: event.description,
            location: event.location,
            start_date: event.start_date.map(|d| d.to_rfc3339()),
            end_date: event.end_date.map(|d| d.to_rfc3339()),
            registration_open: event.registration_open,
        }
    }
}

async fn list_events(State(pool): State<PgPool>) -> ApiResult<Json<Value>> {
    let events = Event::list_open_registrations(&pool).await?;
    let event_responses: Vec<EventResponse> = events.into_iter().map(EventResponse::from).collect();

    Ok(Json(json!({
        "events": event_responses
    })))
}

async fn register_participant(
    State(pool): State<PgPool>,
    Path(event_id): Path<Uuid>,
    Json(payload): Json<RegisterParticipantRequest>,
) -> ApiResult<Json<Value>> {
    // Verify event exists and registration is open
    let event = Event::find_by_id(&pool, event_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    if !event.registration_open {
        return Err(ApiError::BadRequest("Registration is not open for this event".to_string()));
    }

    // Validate division
    if payload.division != "A" && payload.division != "B" {
        return Err(ApiError::BadRequest("Division must be either 'A' or 'B'".to_string()));
    }

    // Validate t-shirt size
    let valid_sizes = ["XS", "S", "M", "L", "XL", "XXL"];
    if !valid_sizes.contains(&payload.tshirt_size.as_str()) {
        return Err(ApiError::BadRequest("Invalid t-shirt size".to_string()));
    }

    let create_participant = CreateParticipant {
        event_id,
        user_id: None, // TODO: Get from session if user is logged in
        first_name: payload.first_name,
        last_name: payload.last_name,
        email: payload.email,
        tshirt_size: payload.tshirt_size,
        division: payload.division,
        expected_graduation_year: payload.expected_graduation_year,
        university: payload.university,
        resume_url: payload.resume_url,
        acknowledged_id_requirement: payload.acknowledged_id_requirement,
        acknowledged_filming: payload.acknowledged_filming,
        acknowledged_team_merge: payload.acknowledged_team_merge,
        interested_in_financial_aid: payload.interested_in_financial_aid,
        additional_data: None,
    };

    let participant = Participant::create(&pool, create_participant).await?;
    let participant_response: ParticipantResponse = participant.into();

    Ok(Json(json!({
        "participant": participant_response
    })))
}

async fn list_participants(
    State(pool): State<PgPool>,
    Path(event_id): Path<Uuid>,
) -> ApiResult<Json<Value>> {
    let participants = Participant::find_by_event(&pool, event_id).await?;
    let participant_responses: Vec<ParticipantResponse> = participants
        .into_iter()
        .map(ParticipantResponse::from)
        .collect();

    Ok(Json(json!({
        "participants": participant_responses
    })))
}
