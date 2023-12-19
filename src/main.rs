mod consts;

use axum::{
    error_handling::HandleErrorLayer, extract::State, http::StatusCode, response::IntoResponse,
    routing::get, BoxError, Router,
};
use consts::*;
use erye::eyre::bail;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing as t;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{borrow::Cow, net::SocketAddr, sync::Arc, time::Duration};

/// Alias erye::Result
pub(crate) type Result<E> = erye::Result<E>;

pub struct AppState {
    db: DatabaseConnection,
}

/// Configure and start the web server
#[tokio::main]
async fn main() -> Result<()> {
    setup_env();
    setup_tracing();
    erye::install()?;

    let db = setup_database().await?;
    let listener = setup_address().await?;
    let state = Arc::new(AppState { db });
    let app = make_app(state);

    t::info!("Started server on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}

/// Define routes, layers, services
fn make_app(state: Arc<AppState>) -> Router {
    let common_middleware = ServiceBuilder::new()
        // Handle errors from middleware
        .layer(HandleErrorLayer::new(handle_errors))
        // If a layer cannot handle incoming requests, drop the requests
        .load_shed()
        // Total number of concurrent connections
        .concurrency_limit(1024)
        // Maximum connection time
        .timeout(Duration::from_secs(10))
        // Classify HTTP response codes as errors and trace them
        .layer(TraceLayer::new_for_http());

    Router::new()
        .route(
            "/",
            get(|State(_state): State<Arc<AppState>>| async move { "Hello from GET /" }),
        )
        .layer(common_middleware)
        .with_state(state)
}

fn setup_env() {
    let env_file = match std::env::var(RUN_MODE) {
        Ok(mode) => match mode.as_str() {
            PROD => ENV_FILE_PROD,
            DEV => ENV_FILE_DEV,
            _ => ENV_FILE_DEV,
        },
        Err(_) => ENV_FILE_DEV,
    };
    // In production env variable will be loaded via `fly secrets`
    // In development env variables will be loaded locally
    match std::env::var(FLY_APP_NAME).ok() {
        Some(_) => {
            t::info!("Inside fly.io, loading variables from environment")
        }
        None => {
            t::info!("Not running in PROD on Fly.io");
            match dotenvy::from_filename(env_file) {
                Ok(_path) => t::info!("Loaded {env_file} file successfully"),
                Err(e) => t::error!("Failed to load {env_file} file with error = {e:?}"),
            };
        }
    }
}

fn setup_tracing() {
    // Tracing a.k.a logging
    tracing_subscriber::registry()
        // Filter tracing based on the following syntax: target=level
        // DefaultEnv is RUST_LOG
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| LOG_FILTER.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init()
}

async fn setup_address() -> Result<TcpListener> {
    let server_domain =
        std::env::var(SERVER_DOMAIN).expect("The .env file is missing SERVER_DOMAIN");
    let server_port = std::env::var(SERVER_PORT).expect("The .env file is missing SERVER_PORT");
    let address: SocketAddr = format!("{server_domain}:{server_port}")
        .parse()
        .expect("SERVER_DOMAIN:SERVER_PORT is an invalid SocketAddr");
    if let Ok(listener) = TcpListener::bind(address).await {
        return Ok(listener);
    }
    bail!("Failed to connect TcpListener to address = {address}");
}

async fn setup_database() -> Result<DatabaseConnection> {
    let db_url = std::env::var(DATABASE_URL).expect("The .env file is missing DATABASE_URL");
    t::debug!("Database URL = {db_url}");
    let conn = Database::connect(db_url).await;
    t::debug!("Database Connection = {conn:?}");
    // Migrator::up(&conn, None).await.unwrap();
    Ok(conn?)
}

/// Handle any kind of error from the middleware
// TODO: Actually look at the different errors our middleware stack might return
// Examples: https://github.com/tokio-rs/axum/blob/12676aabea1b93e1627768bac9d63c1f471d71cb/examples/key-value-store/src/main.rs#L133-L147
async fn handle_errors(error: BoxError) -> impl IntoResponse {
    // Different middleware from tower have different Error types we should handle here
    // and return the appropriate HTTP status code
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {error}")),
    )
}
