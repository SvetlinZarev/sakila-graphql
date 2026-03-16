use crate::graphql::core::query::query;
use crate::graphql::model::{Actor, ActorFilter, Film, FilmFilter};
use crate::server::AppState;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Executor, Object, Schema};

mod core;
pub mod loader;
mod model;

pub fn build_schema(state: AppState) -> impl Executor {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(state.db.db.clone())
        .finish()
}

pub struct Query;

#[Object]
impl Query {
    async fn actors<'a>(
        &self,
        ctx: &Context<'a>,
        filter: Option<ActorFilter>,
    ) -> async_graphql::Result<Vec<Actor>> {
        Ok(query(ctx, &filter, None).await?)
    }

    async fn films<'a>(
        &self,
        ctx: &Context<'a>,
        filter: Option<FilmFilter>,
    ) -> async_graphql::Result<Vec<Film>> {
        Ok(query(ctx, &filter, None).await?)
    }
}
