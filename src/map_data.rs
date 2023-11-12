use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct MapData {
	#[serde(rename = "elements")]
	pub coordinates: Vec<Coordinates>,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Coordinates {
	pub id: i64,

	#[serde(rename = "lat", default)]
	pub lat: f64,

	#[serde(rename = "lon", default)]
	pub lon: f64,
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
