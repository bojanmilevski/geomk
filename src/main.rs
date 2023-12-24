mod auth;
mod database;
mod errors;
mod filter;
mod handlers;
mod model;
mod osm_api;
mod pipe;
mod routes;

use axum::http::Method;
use axum::middleware;
use axum::Router;
use errors::Result;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors;

#[tokio::main]
async fn main() -> Result<()> {
	let cors = cors::CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_headers(cors::Any)
		.allow_origin(cors::Any);

	let routes_all = Router::new()
		.merge(routes::routes_map())
		.nest("/api", routes::routes_user_management())
		.nest("/api", routes::routes_requests())
		.layer(middleware::map_response(handlers::main_response_mapper))
		.layer(CookieManagerLayer::new())
		.layer(cors)
		.fallback_service(routes::routes_static());

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("http://{}", addr);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, routes_all).await.unwrap();

	Ok(())
}
