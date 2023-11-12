use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct MapData {
	pub elements: Vec<Coordinates>,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Coordinates {
	pub id: i64,

	#[serde(default = "default")]
	pub lat: f64,

	#[serde(default = "default")]
	pub lon: f64,
}

fn default() -> f64 {
	0.0
}
