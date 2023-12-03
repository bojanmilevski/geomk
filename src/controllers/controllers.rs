use crate::errors::errors::Result;
use axum::http::Response;
use axum::response::Html;

pub async fn index_controller() -> Result<Html<String>> {
	Ok(Html(tokio::fs::read_to_string("src/static/index.html").await?))
}

pub async fn map_controller() -> Result<Html<String>> {
	Ok(Html(tokio::fs::read_to_string("src/static/map.html").await?))
}

pub async fn map_script_controller() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("src/static/map.js").await?)?)
}

pub async fn request_script_controller() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("src/static/request.js").await?)?)
}

pub async fn stylesheet_controller() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/css")
		.body(tokio::fs::read_to_string("src/static/style.css").await?)?)
}
