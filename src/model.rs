use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
	#[serde(rename = "elements")]
	pub coordinates: Vec<Coordinates>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromRow)]
pub struct Coordinates {
	pub id: i64,
	#[serde(default = "def")]
	pub lat: f64,
	#[serde(default = "def")]
	pub lon: f64,
}

fn def() -> f64 {
	0.0
}

impl MapData {
	pub fn from(coordinates: Vec<Coordinates>) -> Self {
		Self { coordinates }
	}
}

impl Coordinates {
	pub fn is_in_city(&self, city: &MapData) -> bool {
		let mut inside = false;
		let n = city.coordinates.len();

		for i in 0..n {
			let j = (i + 1) % n;

			if (city.coordinates[i].lon > self.lon) != (city.coordinates[j].lon > self.lon)
				&& (self.lat
					< (city.coordinates[j].lat - city.coordinates[i].lat) * (self.lon - city.coordinates[i].lon)
						/ (city.coordinates[j].lon - city.coordinates[i].lon)
						+ city.coordinates[i].lat)
			{
				inside = !inside;
			}
		}

		inside
	}
}
