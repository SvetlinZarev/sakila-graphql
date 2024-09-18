use axum::http::{HeaderValue, Request};
use tower_http::request_id::{MakeRequestId, RequestId};

pub const HEADER_X_REQUEST_ID: &'static str = "x-request-id";

#[derive(Clone)]
pub struct RequestIdFactory {}

impl RequestIdFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl MakeRequestId for RequestIdFactory {
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        if request.headers().contains_key(HEADER_X_REQUEST_ID) {
            return None;
        }

        let mut buf = [0u8; uuid::fmt::Hyphenated::LENGTH];
        let request_id = uuid::Uuid::new_v4().hyphenated().encode_upper(&mut buf);
        let header_value = HeaderValue::from_str(request_id).unwrap();

        Some(RequestId::new(header_value))
    }
}
