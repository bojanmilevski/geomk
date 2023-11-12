mod args;
mod database;
mod errors;
mod filter;
mod map_data;
mod osm_api;
mod pipe;

use args::Args;
use database::Database;
use map_data::MapData;
use pipe::Pipe;

use clap::Parser;

type Result<T> = std::result::Result<T, errors::Error>;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();

	let coordinates = osm_api::query_coordinates(&args.query).await?;
	let map_data: MapData = serde_json::from_str(&coordinates)?;

	let boundaries = osm_api::query_city_boundaries(&args.city).await?;
	let city_boundaries: MapData = serde_json::from_str(&boundaries)?;

	let db = Database::new(&args.db_name, &args.city).await?;
	db.insert_data(&map_data, "coordinates").await?;
	db.insert_data(&city_boundaries, &args.city).await?;

	let table_data = db.select_data(&args.city).await?;
	for c in table_data {
		println!("{:#?}", c);
	}

	/*
	let pipe: Pipe<MapData> = Pipe::new();
	pipe.add_filter(Box::new(JsonFilter));
	let result = pipe.run_filters(table_data);
	*/

	Ok(())
}
