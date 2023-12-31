use crate::errors::Error;
use crate::errors::Result;

const OVERPASS_URL: &str = "https://overpass-api.de/api/interpreter";

async fn send_query(query_params: &str) -> Result<String> {
	let response = reqwest::Client::new()
		.get(OVERPASS_URL)
		.query(&[("data", &query_params)])
		.send()
		.await?;

	if !response.status().is_success() {
		return Err(Error::Response);
	}

	let raw_data = response.text().await?;

	Ok(raw_data)
}

pub async fn query_coordinates(query_item: &str) -> Result<String> {
	let query_params = format!(
		r#"
			[out:json];
			area["ISO3166-1"="MK"]->.a;
			(
				node(area.a)["amenity"="{query_item}"];
			);
			out center;
			"#
	);

	Ok(send_query(&query_params).await?)
}

pub async fn query_city_boundaries(city: &str) -> Result<String> {
	let query_params = format!(
		r#"
			[out:json];
			area["name:en"="{city}"];
			(
				relation["boundary"="administrative"]["name:en"="{city}"];
			);
			(._;>;);
			out body;
			"#
	);

	Ok(send_query(&query_params).await?)
}
