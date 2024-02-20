use dotenv::dotenv;
use clap::Parser;
use sqlx::SqlitePool;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use e_t_::config::Config;
use e_t_::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "e_t_=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing server...");
    dotenv().ok();
    let config = Config::parse();
    let db;
    if config.database == String::from("pg") { db = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?; } else { db = SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?; }
    app::serve(config, db).await?;
    Ok(())
}