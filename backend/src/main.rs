use anyhow::Result;
use backend::db;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Connecting to database...");
    let pool = db::create_pool(&database_url).await?;

    println!("Running migrations...");
    db::run_migrations(&pool).await?;
    println!("Migrations completed successfully!");

    println!("Backend initialized!");
    Ok(())
}
