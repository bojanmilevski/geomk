mod controllers {
	pub mod controllers;
}

mod errors {
	pub mod errors;
}

mod models {
	pub mod map_data;
}

mod routers {
	pub mod routers;
}

mod services {
	pub mod database;
	pub mod filter;
	pub mod osm_api;
	pub mod pipe;
}

use axum::http::Method;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use controllers::controllers::index_controller;
use controllers::controllers::map_controller;
use controllers::controllers::map_script_controller;
use controllers::controllers::request_script_controller;
use controllers::controllers::stylesheet_controller;
use errors::errors::Result;
use routers::routers::handle_request;
use std::net::SocketAddr;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_headers(Any)
		.allow_origin(Any);
	let app = Router::new()
		.route("/", get(index_controller))
		.route("/map.html", get(map_controller))
		.route("/request", post(handle_request))
		.route("/map.js", get(map_script_controller))
		.route("/request.js", get(request_script_controller))
		.route("/style.css", get(stylesheet_controller))
		.layer(cors);
	let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
	println!("http://{}", addr);
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app).await.unwrap();

	Ok(())
}
