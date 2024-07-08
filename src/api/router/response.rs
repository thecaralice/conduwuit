use axum::response::{IntoResponse, Response};
use bytes::BytesMut;
use conduit::Error;
use http::StatusCode;
use http_body_util::Full;
use ruma::api::{client::uiaa::UiaaResponse, OutgoingResponse};

#[derive(Clone)]
pub struct RumaResponse<T>(pub T);

impl From<Error> for RumaResponse<UiaaResponse> {
	fn from(t: Error) -> Self { Self(t.into()) }
}

impl<T: OutgoingResponse> IntoResponse for RumaResponse<T> {
	fn into_response(self) -> Response {
		match self.0.try_into_http_response::<BytesMut>() {
			Ok(res) => res.map(BytesMut::freeze).map(Full::new).into_response(),
			Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
		}
	}
}