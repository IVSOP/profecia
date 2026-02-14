use std::sync::Arc;

use anyhow::Context;
use blockchain_client::{DEFAULT_RPC_HTTP, ProfeciaClient};
use clap::Parser;
use sea_orm::{Database, DatabaseConnection};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, signer::Signer};
use tokio::net::TcpListener;
use tracing::info;
use utils::axum_utils::shutdown_signal;

mod entity;
mod error;
mod route;
mod solana_integration;
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
    #[arg(
        long,
        env = "RPC_URL",
        default_value = DEFAULT_RPC_HTTP
    )]
    rpc_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub database: DatabaseConnection,
    pub solana: Arc<ProfeciaClient>,
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

    let solana = ProfeciaClient::new(&config.rpc_url)?;

    let _sig = solana
        .rpc_client
        .request_airdrop(&solana.admin_wallet.pubkey(), LAMPORTS_PER_SOL * 10)
        .await?;

    let app_state = AppState {
        database,
        solana: Arc::new(solana),
    };

    {
        let snapshot_state = app_state.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = snapshot_state.record_market_snapshots().await {
                    tracing::error!("Failed to record market snapshots: {}", e);
                } else {
                    tracing::debug!("Recorded market snapshots");
                }
            }
        });
    }

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
