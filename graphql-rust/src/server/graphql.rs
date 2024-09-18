use async_graphql::dataloader::{DataLoader, HashMapCache};
use async_graphql::{
    http::{create_multipart_mixed_stream, is_accept_multipart_mixed},
    BatchRequest, Executor, Request,
};
use async_graphql_axum::rejection::GraphQLRejection;
use async_graphql_axum::{GraphQLBatchRequest, GraphQLRequest, GraphQLResponse};
use axum::{
    body::{Body, HttpBody},
    extract::FromRequest,
    http::{Request as HttpRequest, Response as HttpResponse},
    response::IntoResponse,
    BoxError,
};
use bytes::Bytes;
use futures_util::{future::BoxFuture, StreamExt};
use rustc_hash::FxBuildHasher;
use std::any::Any;
use std::sync::Arc;
use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Duration,
};
use tower_service::Service;

use crate::config::DataLoaderConfig;
use crate::graphql::loader::{
    ActorFilmIdLoader, ActorLoader, CategoryLoader, FilmActorIdLoader, FilmCategoryIdLoader,
    FilmLoader, LanguageLoader,
};
use crate::server::{AppState, Database};

#[derive(Clone)]
pub struct GraphQL<E> {
    inner: Arc<Inner<E>>,
}

struct Inner<E> {
    config: DataLoaderConfig,
    app_state: AppState,
    executor: E,
}

impl<E> GraphQL<E> {
    pub fn new(config: DataLoaderConfig, app_state: AppState, executor: E) -> Self {
        Self {
            inner: Arc::new(Inner {
                config,
                app_state,
                executor,
            }),
        }
    }
}

impl<B, E> Service<HttpRequest<B>> for GraphQL<E>
where
    B: HttpBody<Data = Bytes> + Send + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
    E: Executor,
{
    type Response = HttpResponse<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HttpRequest<B>) -> Self::Future {
        let inner = self.inner.clone();

        let req = req.map(Body::new);
        Box::pin(async move {
            let is_accept_multipart_mixed = req
                .headers()
                .get("accept")
                .and_then(|value| value.to_str().ok())
                .map(is_accept_multipart_mixed)
                .unwrap_or_default();

            if is_accept_multipart_mixed {
                let mut req = match GraphQLRequest::<GraphQLRejection>::from_request(req, &()).await
                {
                    Ok(req) => req,
                    Err(err) => return Ok(err.into_response()),
                };

                attach_data_loaders(&mut req.0, &inner.config, &inner.app_state.db);
                let stream = inner.executor.execute_stream(req.0, None);
                let body = Body::from_stream(
                    create_multipart_mixed_stream(stream, Duration::from_secs(30))
                        .map(Ok::<_, std::io::Error>),
                );

                Ok(HttpResponse::builder()
                    .header("content-type", "multipart/mixed; boundary=graphql")
                    .body(body)
                    .expect("BUG: invalid response"))
            } else {
                let mut req =
                    match GraphQLBatchRequest::<GraphQLRejection>::from_request(req, &()).await {
                        Ok(req) => req,
                        Err(err) => return Ok(err.into_response()),
                    };

                attach_data_loaders(&mut req.0, &inner.config, &inner.app_state.db);
                Ok(GraphQLResponse(inner.executor.execute_batch(req.0).await).into_response())
            }
        })
    }
}

trait WithData {
    fn insert_with<D: Any + Send + Sync, F: Clone + Fn() -> D>(&mut self, data_factory: F);
}

impl WithData for Request {
    fn insert_with<D: Any + Send + Sync, F: Clone + Fn() -> D>(&mut self, data_factory: F) {
        self.data.insert(data_factory())
    }
}

impl WithData for BatchRequest {
    fn insert_with<D: Any + Send + Sync, F: Clone + Fn() -> D>(&mut self, data_factory: F) {
        match self {
            BatchRequest::Single(r) => r.insert_with(data_factory),
            BatchRequest::Batch(b) => b
                .iter_mut()
                .for_each(|r| r.insert_with(data_factory.clone())),
        }
    }
}

fn attach_data_loaders<R: WithData>(r: &mut R, cfg: &DataLoaderConfig, db: &Database) {
    r.insert_with(|| {
        DataLoader::with_cache(
            LanguageLoader::new(db.db.clone()),
            tokio::task::spawn,
            HashMapCache::<FxBuildHasher>::new(),
        )
        .max_batch_size(cfg.max_batch_size)
        .delay(Duration::from_millis(cfg.default_delay_ms))
    });

    r.insert_with(|| {
        DataLoader::with_cache(
            CategoryLoader::new(db.db.clone()),
            tokio::task::spawn,
            HashMapCache::<FxBuildHasher>::new(),
        )
        .max_batch_size(cfg.max_batch_size)
        .delay(Duration::from_millis(cfg.default_delay_ms))
    });

    r.insert_with(|| {
        DataLoader::with_cache(
            ActorLoader::new(db.db.clone()),
            tokio::task::spawn,
            HashMapCache::<FxBuildHasher>::new(),
        )
        .max_batch_size(cfg.max_batch_size)
        .delay(Duration::from_millis(cfg.default_delay_ms))
    });

    r.insert_with(|| {
        DataLoader::with_cache(
            ActorFilmIdLoader::new(db.db.clone()),
            tokio::task::spawn,
            HashMapCache::<FxBuildHasher>::new(),
        )
        .max_batch_size(cfg.max_batch_size)
        .delay(Duration::from_millis(cfg.default_delay_ms))
    });

    r.insert_with(|| {
        DataLoader::with_cache(
            FilmLoader::new(db.db.clone()),
            tokio::task::spawn,
            HashMapCache::<FxBuildHasher>::new(),
        )
        .max_batch_size(cfg.max_batch_size)
        .delay(Duration::from_millis(cfg.default_delay_ms))
    });

    r.insert_with(|| {
        DataLoader::with_cache(
            FilmCategoryIdLoader::new(db.db.clone()),
            tokio::task::spawn,
            HashMapCache::<FxBuildHasher>::new(),
        )
        .max_batch_size(cfg.max_batch_size)
        .delay(Duration::from_millis(cfg.default_delay_ms))
    });

    r.insert_with(|| {
        DataLoader::with_cache(
            FilmActorIdLoader::new(db.db.clone()),
            tokio::task::spawn,
            HashMapCache::<FxBuildHasher>::new(),
        )
        .max_batch_size(cfg.max_batch_size)
        .delay(Duration::from_millis(cfg.default_delay_ms))
    });
}
