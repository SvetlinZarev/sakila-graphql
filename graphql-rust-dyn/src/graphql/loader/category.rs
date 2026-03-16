use crate::graphql::core::loader::load_one_by_key;
use crate::graphql::model::Category;
use async_graphql::dataloader::Loader;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct CategoryLoader {
    db: Pool,
}

impl CategoryLoader {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

impl Loader<i32> for CategoryLoader {
    type Value = Category;
    type Error = Arc<anyhow::Error>;

    fn load(
        &self,
        keys: &[i32],
    ) -> impl Future<Output = Result<HashMap<i32, Self::Value>, Self::Error>> + Send {
        static QUERY: &str = "SELECT category_id, name FROM category where category_id = ANY($1)";

        async {
            load_one_by_key(&self.db, &QUERY, keys)
                .await
                .map_err(|e| Arc::new(e))
        }
    }
}
