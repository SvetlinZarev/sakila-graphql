use config::Config;
use serde::{Deserialize, Serialize};

pub fn load<'a, PREFIX, SPLIT, CFG>(prefix: PREFIX, split: SPLIT) -> anyhow::Result<ServiceConfig>
where
    PREFIX: AsRef<str>,
    SPLIT: AsRef<str>,
    CFG: Default + Serialize + Deserialize<'a>,
{
    let defaults = CFG::default();
    let defaults = serde_json::to_string(&defaults)?;

    let cfg = Config::builder()
        .add_source(config::File::from_str(&defaults, config::FileFormat::Json))
        .add_source(config::Environment::with_prefix(prefix.as_ref()).separator(split.as_ref()))
        .build()?;

    Ok(cfg.try_deserialize()?)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub db: DatabaseConfig,
    pub server: ServerConfig,
    pub data_loader: DataLoaderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLoaderConfig {
    pub default_delay_ms: u64,
    pub max_batch_size: usize,
}

impl Default for DataLoaderConfig {
    fn default() -> Self {
        Self {
            default_delay_ms: 10,
            max_batch_size: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub request_timeout: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            request_timeout: 10_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub pass: String,
    pub db_name: String,
    pub host: String,
    pub port: u16,
    pub max_conn: usize,
    pub create_timeout: u64,
    pub acquire_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            user: "postgres".to_string(),
            pass: "password".to_string(),
            db_name: "postgres".to_string(),
            host: "127.0.0.1".to_string(),
            port: 5432,
            max_conn: 16,
            create_timeout: 5_000,
            acquire_timeout: 5_000,
        }
    }
}
