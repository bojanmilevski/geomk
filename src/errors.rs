use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Response Error")]
	Response,

	#[error("Signup error")]
	Signup,

	#[error("Login error")]
	Login,

	#[error("Auth token error")]
	AuthToken,

	#[error("Request Error")]
	Request(#[from] reqwest::Error),

	#[error("Serde Error")]
	SerdeJson(#[from] serde_json::Error),

	#[error("Sqlx Error")]
	Sqlx(#[from] sqlx::Error),

	#[error("Axum Error")]
	Axum(#[from] axum::Error),

	#[error("IO Error")]
	IO(#[from] std::io::Error),

	#[error("Axum HTTP Error")]
	AxumHTTP(#[from] axum::http::Error),
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		eprintln!("ERROR: {:#?}", self);

		(StatusCode::INTERNAL_SERVER_ERROR, "SERVER_ERROR").into_response()
	}
}
