use crate::graphql::core::loader::load_one_by_key;
use crate::graphql::model::Actor;
use async_graphql::dataloader::Loader;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub struct ActorLoader {
    db: Pool,
}

impl ActorLoader {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

impl Loader<i32> for ActorLoader {
    type Value = Actor;
    type Error = Arc<anyhow::Error>;

    fn load(
        &self,
        keys: &[i32],
    ) -> impl Future<Output = Result<HashMap<i32, Self::Value>, Self::Error>> + Send {
        async {
            load_one_by_key(
                &self.db,
                "SELECT actor_id, first_name, last_name FROM actor WHERE actor_id = ANY($1)",
                keys,
            )
            .await
            .map_err(|e| Arc::new(e))
        }
    }
}
