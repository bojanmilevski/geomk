use crate::map_data::MapData;

pub trait Filter<T> {
	fn execute(&self, input: T) -> T;
}

pub struct CityFilter {
	city: MapData,
}

impl CityFilter {
	pub fn new(city: MapData) -> Self {
		Self { city }
	}
}

impl Filter<MapData> for CityFilter {
	fn execute(&self, input: MapData) -> MapData {
		let mut map = MapData { coordinates: Vec::new() };

		for e in input.coordinates {
			if e.is_in_city(&self.city) {
				map.coordinates.push(e);
			}
		}

		map
	}
}
