use crate::auth;
use crate::errors::Result;
use crate::handlers;
use axum::middleware;
use axum::response::Html;
use axum::response::Response;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::Router;

async fn map_index_route() -> Result<Html<String>> {
	Ok(Html(tokio::fs::read_to_string("./static/map.html").await?))
}

async fn map_script_route() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("./static/map.js").await?)?)
}

async fn index_route() -> Result<Html<String>> {
	Ok(Html(tokio::fs::read_to_string("./static/index.html").await?))
}

async fn login_route() -> Result<Html<String>> {
	Ok(Html(tokio::fs::read_to_string("./static/login.html").await?))
}

async fn signup_route() -> Result<Html<String>> {
	Ok(Html(tokio::fs::read_to_string("./static/signup.html").await?))
}

async fn style_route() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/css")
		.body(tokio::fs::read_to_string("./static/style.css").await?)?)
}

async fn redirect_route() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("./static/redirect.js").await?)?)
}

async fn user_manager_route() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("./static/userManagement.js").await?)?)
}

async fn profile_route() -> Result<Html<String>> {
	Ok(Html(tokio::fs::read_to_string("./static/profile.html").await?))
}

async fn profile_script_route() -> Result<Response<String>> {
	Ok(Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("./static/profile.js").await?)?)
}

pub fn routes_static() -> Router {
	Router::new()
		.route("/", get(index_route))
		.route("/login.html", get(login_route))
		.route("/signup.html", get(signup_route))
		.route("/style.css", get(style_route))
		.route("/redirect.js", get(redirect_route))
		.route("/userManagement.js", get(user_manager_route))
}

pub fn routes_map() -> Router {
	Router::new()
		.route("/map.html", get(map_index_route))
		.route("/map.js", get(map_script_route))
		.route("/profile.html", get(profile_route))
		.route("/profile.js", get(profile_script_route))
		.route_layer(middleware::from_fn(auth::mw_require_auth))
}

pub fn routes_user_management() -> Router {
	Router::new()
		.route("/signup", post(handlers::signup_handler))
		.route("/login", post(handlers::login_handler))
}

pub fn routes_requests() -> Router {
	Router::new()
		.route("/request", post(handlers::handle_request))
		.route("/save", post(handlers::save_handler))
		.route("/get", get(handlers::get_handler))
		.route("/delete/:id", delete(handlers::delete_handler))
		.route_layer(middleware::from_fn(auth::mw_require_auth))
}
