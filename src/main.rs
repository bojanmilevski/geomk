mod database;
mod errors;
mod filter;
mod map_data;
mod osm_api;
mod pipe;
mod routers;

use axum::http::Method;
use axum::routing::get_service;
use axum::routing::post;
use axum::Router;
use errors::Result;
use routers::handle_request;
use std::net::SocketAddr;
use tower_http::cors;
use tower_http::services::ServeDir;

fn static_routes() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}

fn routes() -> Router {
	Router::new().route("/request", post(handle_request))
}

#[tokio::main]
async fn main() -> Result<()> {
	let cors = cors::CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_headers(cors::Any)
		.allow_origin(cors::Any);

	let routes = Router::new()
		.merge(routes())
		.layer(cors)
		.fallback_service(static_routes());

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("http://{}", addr);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, routes).await.unwrap();

	Ok(())
}
