use crate::database::Database;
use crate::errors;
use crate::errors::Result;
use crate::filter::CityFilter;
use crate::map_data::Coordinates;
use crate::map_data::MapData;
use crate::osm_api::OsmApi;
use crate::pipe::Pipe;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use sqlx::SqlitePool;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Request {
	pub query: String,
	pub city: String,
}

pub async fn handle_request(request: Json<Request>) -> Result<Json<Vec<Coordinates>>> {
	let json_coordinates = OsmApi::query_coordinates(&request.query).await?;
	let coordinates: MapData = serde_json::from_str(&json_coordinates)?;

	let json_boundaries = OsmApi::query_city_boundaries(&request.city).await?;
	let city_boundaries: MapData = serde_json::from_str(&json_boundaries)?;

	let db = Database::new().await?;
	db.insert_table("coordinates").await?;
	db.insert_data(&coordinates, "coordinates").await?;
	db.insert_table(&request.city).await?;
	db.insert_data(&city_boundaries, &request.city).await?;

	let coordinates_data = db.select_data("coordinates").await?;
	let city_boundaries_data = db.select_data(&request.city).await?;

	let mut pipe: Pipe<MapData> = Pipe::new();
	pipe.add_filter(Box::new(CityFilter::new(city_boundaries_data)));
	let result = pipe.run_filters(coordinates_data);

	Ok(Json(result.coordinates))
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CredentialsPayload {
	pub username: String,
	pub password: String,
}

pub async fn signup_handler(credentials: Json<CredentialsPayload>) -> Result<Json<Value>> {
	let url = format!("sqlite://database.db");

	if !Sqlite::database_exists(&url).await? {
		Sqlite::create_database(&url).await?;
	}

	let db = SqlitePool::connect(&url).await?;

	sqlx::query(
		"CREATE TABLE IF NOT EXISTS users (
        username TEXT NOT NULL,
        password TEXT NOT NULL
    );",
	)
	.execute(&db)
	.await?;

	let user_exists = sqlx::query("SELECT * FROM users WHERE username = ?1")
		.bind(&credentials.username)
		.fetch_optional(&db)
		.await?;

	if user_exists.is_some() {
		return Err(errors::Error::Signup);
	}

	sqlx::query("INSERT OR IGNORE INTO users (username, password) VALUES (?1, ?2)")
		.bind(&credentials.username)
		.bind(&credentials.password)
		.execute(&db)
		.await?;

	Ok(Json(json!({
		"result": {
			"success": true
		}
	})))
}

pub async fn login_handler(credentials: Json<CredentialsPayload>) -> Result<Json<Value>> {
	let url = format!("sqlite://database.db");

	if !Sqlite::database_exists(&url).await? {
		Sqlite::create_database(&url).await?;
	}

	let db = SqlitePool::connect(&url).await?;

	let user_exists = sqlx::query("SELECT * FROM users WHERE username = ?1 AND password = ?2")
		.bind(&credentials.username)
		.bind(&credentials.password)
		.fetch_optional(&db)
		.await?;

	if user_exists.is_none() {
		return Err(errors::Error::Login);
	}

	Ok(Json(json!({
		"result": {
			"success": true
		}
	})))
}
