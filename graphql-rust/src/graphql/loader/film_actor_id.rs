use crate::graphql::core::loader::load_many_by_key;
use async_graphql::dataloader::Loader;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct FilmActorIdLoader {
    db: Pool,
}

impl FilmActorIdLoader {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

impl Loader<i32> for FilmActorIdLoader {
    type Value = Vec<i32>;
    type Error = Arc<anyhow::Error>;

    fn load(
        &self,
        keys: &[i32],
    ) -> impl Future<Output = Result<HashMap<i32, Self::Value>, Self::Error>> + Send {
        async {
            load_many_by_key(
                &self.db,
                "SELECT film_id as __loader_key, actor_id as __value FROM film_actor WHERE film_id = ANY($1)",
                keys,
            )
            .await
            .map_err(|e| Arc::new(e))
        }
    }
}
