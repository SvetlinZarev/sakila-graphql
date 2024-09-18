use crate::config::ServiceConfig;
use async_graphql::Executor;
use axum::routing::{get, post_service};
use axum::Router;
use std::error::Error;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::ServiceBuilderExt;
use tracing::Level;

mod api_explorer;
mod app_state;
mod graphql;
mod logging;
mod request_id;
mod shutdown;

use crate::server::api_explorer::{graphiql, playground};
use crate::server::graphql::GraphQL;
use crate::server::logging::{CustomMakeSpan, CustomOnRequest};
pub use app_state::{AppState, Database};

const GRAPHQL_ENDPOINT: &'static str = "/graphql";
const PLAYGROUND_ENDPOINT: &'static str = "/playground";
const GRAPHIQL_ENDPOINT: &'static str = "/graphiql";

pub async fn start_server(
    config: ServiceConfig,
    state: AppState,
    schema: impl Executor,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let timeout_layer = TimeoutLayer::new(Duration::from_millis(config.server.request_timeout));
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(CustomMakeSpan::new())
        .on_request(CustomOnRequest::new())
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let services = ServiceBuilder::new()
        .catch_panic()
        .set_x_request_id(request_id::RequestIdFactory::new())
        .layer(tracing_layer)
        .propagate_x_request_id()
        .layer(timeout_layer);

    let gql_service = GraphQL::new(config.data_loader, state.clone(), schema);

    let router = Router::new()
        .route(PLAYGROUND_ENDPOINT, get(playground))
        .route(GRAPHIQL_ENDPOINT, get(graphiql))
        .route(GRAPHQL_ENDPOINT, post_service(gql_service))
        .layer(services)
        .with_state(state);

    tracing::info!(port = config.server.port, "starting TCP listener");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.server.port)).await?;

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await?;

    Ok(())
}
