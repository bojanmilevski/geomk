use crate::errors::Error;
use crate::Result;

const OVERPASS_URL: &str = "https://overpass-api.de/api/interpreter";

pub async fn query(query_item: &str) -> Result<String> {
	let query_data = format!(
		r#"
        [out:json];
        area["ISO3166-1"="MK"]->.a;
        (
            node(area.a)["amenity"="{query_item}"];
        );
        out center;
    "#
	);

	let response = reqwest::Client::new()
		.get(OVERPASS_URL)
		.query(&[("data", &query_data)])
		.send()
		.await?;

	if !response.status().is_success() {
		return Err(Error::Response);
	}

	let raw_data = response.text().await?;

	Ok(raw_data)
}
