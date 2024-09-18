use crate::graphql::core::loader::load_one_by_key;
use crate::graphql::model::Film;
use async_graphql::dataloader::Loader;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct FilmLoader {
    db: Pool,
}

impl FilmLoader {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

impl Loader<i32> for FilmLoader {
    type Value = Film;
    type Error = Arc<anyhow::Error>;

    fn load(
        &self,
        keys: &[i32],
    ) -> impl Future<Output = Result<HashMap<i32, Self::Value>, Self::Error>> + Send {
        async {
            load_one_by_key(&self.db, "SELECT film_id, title, description, length, language_id, original_language_id FROM film WHERE film_id = ANY($1)", keys)
                .await
                .map_err(|e| Arc::new(e))
        }
    }
}
