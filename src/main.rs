mod args;
mod database;
mod errors;
mod filter;
mod map_data;
mod osm_api;
mod pipe;

use args::Args;
use database::Database;
use filter::CityFilter;
use map_data::MapData;
use osm_api::OsmApi;
use pipe::Pipe;

use clap::Parser;

type Result<T> = std::result::Result<T, errors::Error>;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();

	let json_coordinates = OsmApi::query_coordinates(&args.query).await?;
	let coordinates: MapData = serde_json::from_str(&json_coordinates)?;

	let json_boundaries = OsmApi::query_city_boundaries(&args.city).await?;
	let city_boundaries: MapData = serde_json::from_str(&json_boundaries)?;

	let db = Database::new(&args.db_name, &args.city).await?;
	db.insert_data(&coordinates, "coordinates").await?;
	db.insert_data(&city_boundaries, &args.city).await?;

	let coordinates_data = db.select_data("coordinates").await?;
	let city_boundaries_data = db.select_data(&args.city).await?;

	let mut pipe: Pipe<MapData> = Pipe::new();
	// ... filters ...
	pipe.add_filter(Box::new(CityFilter::new(city_boundaries_data)));
	// ... filters ...
	let result = pipe.run_filters(coordinates_data);

	println!("{result:#?}");

	Ok(())
}
