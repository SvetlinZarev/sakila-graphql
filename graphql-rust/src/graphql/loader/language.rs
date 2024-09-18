use crate::graphql::core::loader::load_one_by_key;
use crate::graphql::model::Language;
use async_graphql::dataloader::Loader;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct LanguageLoader {
    db: Pool,
}

impl LanguageLoader {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

impl Loader<i32> for LanguageLoader {
    type Value = Language;
    type Error = Arc<anyhow::Error>;

    fn load(
        &self,
        keys: &[i32],
    ) -> impl Future<Output = Result<HashMap<i32, Self::Value>, Self::Error>> + Send {
        async {
            load_one_by_key(
                &self.db,
                "SELECT language_id, name FROM language WHERE language_id = ANY($1)",
                keys,
            )
            .await
            .map_err(|e| Arc::new(e))
        }
    }
}
