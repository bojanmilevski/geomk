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
use std::net::SocketAddr;
use tower_http::cors;
use tower_http::services::ServeDir;

fn static_routes() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}

fn routes() -> Router {
	Router::new()
		.route("/request", post(routers::handle_request))
		.route("/signup", post(routers::signup_handler))
		.route("/login", post(routers::login_handler))
}

#[tokio::main]
async fn main() -> Result<()> {
	let cors = cors::CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_headers(cors::Any)
		.allow_origin(cors::Any);

	let routes = Router::new()
		.nest("/api", routes())
		.layer(cors)
		.fallback_service(static_routes());

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("http://{}", addr);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, routes).await.unwrap();

	Ok(())
}
