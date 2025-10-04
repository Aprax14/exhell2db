use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::app_state::{AppState, Env};

pub mod app_state;

fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .with_state(app_state)
        .route("/", get(|| async { "Hello, World!" }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt().init();

    let env = envy::from_env::<Env>().context("bad environment variables")?;
    let app_state = Arc::new(AppState::new(&env));

    let routes = routes(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("build TCP listener");
    tracing::info!("Starting HTTP server at http://localhost:8080");
    axum::serve(
        listener,
        routes.layer(TraceLayer::new_for_http()).into_make_service(),
    )
    .await
    .context("run webserver")?;

    Ok(())
}
