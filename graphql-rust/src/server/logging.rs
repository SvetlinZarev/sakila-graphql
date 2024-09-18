use crate::server::request_id::HEADER_X_REQUEST_ID;
use axum::http::Request;
use tower_http::trace::{MakeSpan, OnRequest};
use tracing::{Level, Span};

#[derive(Debug, Default, Clone)]
pub struct CustomMakeSpan {}

impl CustomMakeSpan {
    pub fn new() -> Self {
        Self {}
    }
}

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        tracing::span!(
            Level::INFO,
            "context",
            request_id = request
                .headers()
                .get(HEADER_X_REQUEST_ID)
                .map(|h| h.to_str().unwrap_or("N/A"))
                .unwrap_or("N/A")
        )
    }
}

#[derive(Clone, Debug)]
pub struct CustomOnRequest {
    enabled: bool,
    log_headers: bool,
}

impl Default for CustomOnRequest {
    fn default() -> Self {
        Self {
            enabled: true,
            log_headers: false,
        }
    }
}

impl CustomOnRequest {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(unused)]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[allow(unused)]
    pub fn log_headers(mut self, log_headers: bool) -> Self {
        self.log_headers = log_headers;
        self
    }
}

impl<B> OnRequest<B> for CustomOnRequest {
    fn on_request(&mut self, request: &Request<B>, _: &Span) {
        if !self.enabled {
            return;
        }

        if self.log_headers {
            tracing::event!(
                Level::INFO,
                method = %request.method(),
                uri = %request.uri(),
                headers = ?request.headers(),
                "started processing request"
            );
        } else {
            tracing::event!(
                Level::INFO,
                method = %request.method(),
                uri = %request.uri(),
                "started processing request"
            );
        }
    }
}
