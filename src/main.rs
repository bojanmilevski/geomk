mod errors;
mod filter;
mod map_data;
mod osm_api;
mod pipe;
mod sqlite;

use errors::Result;
use filter::CityFilter;
use map_data::Coordinates;
use map_data::MapData;
use osm_api::OsmApi;
use pipe::Pipe;
use sqlite::Database;

use axum::http::Method;
use axum::http::Response;
use axum::response::Html;
use axum::routing::get;
use axum::routing::post;
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use std::net::SocketAddr;

const DB_NAME: &str = "database";
const QUERY: &str = "drinking_water";

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Request {
	// query: String,
	city: String,
}

async fn handle(request: Json<Request>) -> Result<Json<Vec<Coordinates>>> {
	let json_coordinates = OsmApi::query_coordinates(&QUERY).await?;
	let coordinates: MapData = serde_json::from_str(&json_coordinates)?;

	let json_boundaries = OsmApi::query_city_boundaries(&request.city).await?;
	let city_boundaries: MapData = serde_json::from_str(&json_boundaries)?;

	let db = Database::new(&DB_NAME, &request.city).await?;
	db.insert_data(&coordinates, "coordinates").await?;
	db.insert_data(&city_boundaries, &request.city).await?;

	let coordinates_data = db.select_data("coordinates").await?;
	let city_boundaries_data = db.select_data(&request.city).await?;

	let mut pipe: Pipe<MapData> = Pipe::new();
	pipe.add_filter(Box::new(CityFilter::new(city_boundaries_data)));
	let result = pipe.run_filters(coordinates_data);

	let response = Json(result.coordinates);

	Ok(response)
}

async fn get_root() -> Html<String> {
	Html(
		tokio::fs::read_to_string("src/static/index.html")
			.await
			.unwrap(),
	)
}

async fn get_style() -> Response<String> {
	Response::builder()
		.header("Content-Type", "text/css")
		.body(tokio::fs::read_to_string("src/static/style.css").await.unwrap())
		.unwrap()
}

async fn get_map() -> Response<String> {
	Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("src/static/map.js").await.unwrap())
		.unwrap()
}

async fn get_request() -> Response<String> {
	Response::builder()
		.header("Content-Type", "text/javascript")
		.body(tokio::fs::read_to_string("src/static/request.js").await.unwrap())
		.unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_headers(Any)
		.allow_origin(Any);
	let app = Router::new()
		.route("/", get(get_root))
		.route("/city", post(handle))
		.route("/map.js", get(get_map))
		.route("/request.js", get(get_request))
		.route("/style.css", get(get_style))
		.layer(cors);
	let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
	println!("http://{}", addr);
	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app).await.unwrap();

	Ok(())
}
