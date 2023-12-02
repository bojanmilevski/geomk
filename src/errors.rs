use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Response Error")]
	Response,

	#[error("Request Error")]
	Request(#[from] reqwest::Error),

	#[error("Serde Error")]
	SerdeJson(#[from] serde_json::Error),

	#[error("Sqlx Error")]
	Sqlx(#[from] sqlx::Error),

	#[error("Axum Error")]
	Axum(#[from] axum::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		eprintln!("ERROR");

		(StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_ERROR").into_response()
	}
}
