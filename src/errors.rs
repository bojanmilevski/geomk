use thiserror::Error;

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
}
