#[cfg(test)]
mod tests {
	use serde_json::json;

	#[tokio::test]
	async fn test_example() -> Result<(), Box<dyn std::error::Error>> {
		let client = httpc_test::new_client("http://localhost:8080")?;
		let req_login = client.do_post(
			"/api/login",
			json!({
				"username": "bojan",
				"password": "bojan"
			}),
		);

		req_login.await?.print().await?;

		let req_data2 = client.do_post(
			"/api/request",
			json!({
				"city": "Skopje",
				"query": "drinking_water"
			}),
		);

		req_data2.await?.print().await?;

		Ok(())
	}
}
