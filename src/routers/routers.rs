use crate::errors::errors::Result;
use crate::models::map_data::Coordinates;
use crate::models::map_data::MapData;
use crate::services::database::Database;
use crate::services::filter::CityFilter;
use crate::services::osm_api::OsmApi;
use crate::services::pipe::Pipe;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Request {
	pub query: String,
	pub city: String,
}

pub async fn handle_request(request: Json<Request>) -> Result<Json<Vec<Coordinates>>> {
	let json_coordinates = OsmApi::query_coordinates(&request.query).await?;
	let coordinates: MapData = serde_json::from_str(&json_coordinates)?;

	let json_boundaries = OsmApi::query_city_boundaries(&request.city).await?;
	let city_boundaries: MapData = serde_json::from_str(&json_boundaries)?;

	let db = Database::new().await?;
	db.insert_table("coordinates").await?;
	db.insert_table(&request.city).await?;
	db.insert_data(&coordinates, "coordinates").await?;
	db.insert_data(&city_boundaries, &request.city).await?;

	let coordinates_data = db.select_data("coordinates").await?;
	let city_boundaries_data = db.select_data(&request.city).await?;

	let mut pipe: Pipe<MapData> = Pipe::new();
	pipe.add_filter(Box::new(CityFilter::new(city_boundaries_data)));
	let result = pipe.run_filters(coordinates_data);

	println!("{:#?}", &result.coordinates);
	println!("Coordinates for city: {}", request.city);

	let response = Json(result.coordinates);

	Ok(response)
}
