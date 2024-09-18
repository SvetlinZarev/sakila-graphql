use crate::config::DatabaseConfig;
use deadpool_postgres::Runtime::Tokio1;
use deadpool_postgres::{CreatePoolError, ManagerConfig, Pool, SslMode};
use std::time::Duration;
use tokio_postgres::NoTls;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_db_pool(cfg: &DatabaseConfig) -> Result<Pool, CreatePoolError> {
    let mut pool_cfg = deadpool_postgres::PoolConfig::new(cfg.max_conn);
    pool_cfg.timeouts.wait = Some(Duration::from_millis(cfg.acquire_timeout));
    pool_cfg.timeouts.create = Some(Duration::from_millis(cfg.create_timeout));

    let mut config = deadpool_postgres::Config::new();
    config.pool = Some(pool_cfg);
    config.port = Some(cfg.port);
    config.host = Some(cfg.host.clone());
    config.user = Some(cfg.user.clone());
    config.password = Some(cfg.pass.clone());
    config.dbname = Some(cfg.db_name.clone());
    config.manager = Some(ManagerConfig::default());
    config.ssl_mode = Some(SslMode::Disable);

    config.create_pool(Some(Tokio1), NoTls)
}

pub fn init_tracing() {
    let default_level = if cfg!(debug_assertions) {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let env_filter = EnvFilter::builder()
        .with_default_directive(default_level.into())
        .from_env_lossy();

    let formatter = fmt::layer()
        .with_line_number(true)
        .with_target(true)
        //.with_thread_names(true)
        //.with_thread_ids(true)
        .with_timer(fmt::time::UtcTime::rfc_3339());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatter)
        .init();
}
