use crate::server::GRAPHQL_ENDPOINT;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use axum::response::IntoResponse;

pub async fn playground() -> impl IntoResponse {
    axum::response::Html(playground_source(GraphQLPlaygroundConfig::new(
        GRAPHQL_ENDPOINT,
    )))
}

pub async fn graphiql() -> impl IntoResponse {
    axum::response::Html(GraphiQLSource::build().endpoint(GRAPHQL_ENDPOINT).finish())
}
