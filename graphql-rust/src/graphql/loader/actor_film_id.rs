use crate::graphql::core::loader::load_many_by_key;
use async_graphql::dataloader::Loader;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct ActorFilmIdLoader {
    db: Pool,
}

impl ActorFilmIdLoader {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

impl Loader<i32> for ActorFilmIdLoader {
    type Value = Vec<i32>;
    type Error = Arc<anyhow::Error>;

    fn load(
        &self,
        keys: &[i32],
    ) -> impl Future<Output = Result<HashMap<i32, Self::Value>, Self::Error>> + Send {
        async {
            load_many_by_key(
                &self.db,
                "SELECT actor_id as __loader_key, film_id as __value FROM film_actor WHERE actor_id = ANY($1)",
                keys,
            )
            .await
            .map_err(|e| Arc::new(e))
        }
    }
}
