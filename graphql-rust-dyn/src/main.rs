use graphql_rust_dyn::config;
use graphql_rust_dyn::config::ServiceConfig;
use graphql_rust_dyn::graphql::build_schema;
use graphql_rust_dyn::init::{init_db_pool, init_tracing};
use graphql_rust_dyn::server::{start_server, AppState};
use std::error::Error;

const PREFIX: &str = "CFG";
const CFG_SEPARATOR: &str = "__";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_tracing();

    let cfg = config::load::<_, _, ServiceConfig>(PREFIX, CFG_SEPARATOR)?;

    let db = init_db_pool(&cfg.db)?;
    let state = AppState::new(db.clone());
    let schema = build_schema(state.clone());

    start_server(cfg, state, schema).await?;

    tracing::info!("Closing connection pool");
    db.close();

    tracing::info!("Server stopped");
    Ok(())
}
