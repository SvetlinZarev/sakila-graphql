use graphql_rust::config;
use graphql_rust::config::{ServiceConfig, SPLIT_AT_DOUBLE_UNDERSCORE};
use graphql_rust::graphql::build_schema;
use graphql_rust::init::{init_db_pool, init_tracing};
use graphql_rust::server::{start_server, AppState};
use std::error::Error;

const PREFIX: &str = "CFG__";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_tracing();

    let cfg: ServiceConfig = config::load(PREFIX, SPLIT_AT_DOUBLE_UNDERSCORE)?;

    let db = init_db_pool(&cfg.db)?;
    let state = AppState::new(db.clone());
    let schema = build_schema(state.clone());

    start_server(cfg, state, schema).await?;

    tracing::info!("Closing connection pool");
    db.close();

    tracing::info!("Server stopped");
    Ok(())
}
