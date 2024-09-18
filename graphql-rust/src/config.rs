use figment::providers::{Env, Serialized};
use figment::Figment;
use serde::{Deserialize, Serialize};

pub const SPLIT_AT_DOUBLE_UNDERSCORE: &str = "__";

pub fn load<'a, PREFIX, SPLIT, CFG>(prefix: PREFIX, split: SPLIT) -> figment::error::Result<CFG>
where
    PREFIX: AsRef<str>,
    SPLIT: AsRef<str>,
    CFG: Default + Serialize + Deserialize<'a>,
{
    Figment::from(Serialized::defaults(CFG::default()))
        .merge(Env::prefixed(prefix.as_ref()).split(split.as_ref()))
        .extract()
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
