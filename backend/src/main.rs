use anyhow::Result;
use backend::{api, db, models::{CreateEvent, Event}};
use tower_sessions_sqlx_store::PostgresStore;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    tracing::info!("Connecting to database...");
    let pool = db::create_pool(&database_url).await?;

    tracing::info!("Running migrations...");
    db::run_migrations(&pool).await?;
    tracing::info!("Migrations completed successfully!");

    tracing::info!("Running session store migration...");
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;
    tracing::info!("Session store migration completed!");

    tracing::info!("Checking for default event...");
    ensure_default_event(&pool).await?;

    let app = api::create_router(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn ensure_default_event(pool: &sqlx::PgPool) -> Result<()> {
    // Check if any events exist
    let event_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM events")
        .fetch_one(pool)
        .await?;

    if event_count == 0 {
        tracing::info!("No events found, creating default ICMT 2026 event...");

        // TODO: Update these placeholder values with actual event details
        let create_event = CreateEvent {
            name: "ICMT 2026".to_string(),
            description: Some("TODO: Add event description here".to_string()),
            location: Some("TODO: Add event location here".to_string()),
            start_date: None,
            end_date: None,
            registration_open: true,
        };

        Event::create(pool, create_event).await?;
        tracing::info!("Default event 'ICMT 2026' created successfully!");
    } else {
        tracing::info!("Events already exist, skipping default event creation");
    }

    Ok(())
}
