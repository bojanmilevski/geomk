mod database;
mod errors;
mod filter;
mod map_data;
mod osm_api;
mod pipe;
mod routes;

use axum::body::Body;
use axum::http::Method;
use axum::http::Request;
use axum::middleware;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get_service;
use axum::routing::post;
use axum::Router;
use errors::Result;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_cookies::Cookies;
use tower_http::cors;
use tower_http::services::ServeDir;

const AUTH_TOKEN: &str = "test";

fn routes_static() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}

fn routes_api() -> Router {
	Router::new()
		.nest_service("/map", get_service(ServeDir::new("./api")))
		.route_layer(middleware::from_fn(mw_require_auth))
}

fn login_apis() -> Router {
	Router::new()
		.route("/signup", post(routes::signup_handler))
		.route("/login", post(routes::login_handler))
}

fn routes() -> Router {
	Router::new()
		.route("/request", post(routes::handle_request))
		.route_layer(middleware::from_fn(mw_require_auth))
}

async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
	auth_token.ok_or(errors::Error::AuthToken)?;
	Ok(next.run(req).await)
}

async fn main_response_mapper(res: Response) -> Response {
	res
}

#[tokio::main]
async fn main() -> Result<()> {
	let cors = cors::CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_headers(cors::Any)
		.allow_origin(cors::Any);

	let routes_all = Router::new()
		.merge(routes_api())
		.nest("/api", login_apis())
		.nest("/api", routes())
		.layer(middleware::map_response(main_response_mapper))
		.layer(CookieManagerLayer::new())
		.layer(cors)
		.fallback_service(routes_static());

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	println!("http://{}", addr);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, routes_all).await.unwrap();

	Ok(())
}
