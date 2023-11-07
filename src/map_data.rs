use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct MapData {
	pub elements: Vec<Coordinates>,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Coordinates {
	pub id: i64,
	pub lat: f32,
	pub lon: f32,
}
