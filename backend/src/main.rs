use anyhow::Context;
use clap::Parser;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tracing::info;
use utils::axum_utils::shutdown_signal;

mod entity;
mod error;
mod route;
mod state;
mod utils;

#[derive(clap::Parser)]
struct AppConfig {
    #[arg(
        long,
        env = "DATABASE_URL",
        default_value = "postgres://profecia:profecia@localhost:5432/profecia"
    )]
    database_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let config = AppConfig::parse();

    let database = Database::connect(&config.database_url).await?;

    // Execute migrations
    database
        .get_schema_registry(module_path!().split("::").next().unwrap())
        .sync(&database)
        .await?;

    let app_state = AppState { database };

    let app = route::router(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Failed to bind to address")?;

    info!("Server started on http://localhost:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
