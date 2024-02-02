use crate::auth;
use crate::auth::AUTH_TOKEN;
use crate::database::Database;
use crate::errors::Error;
use crate::errors::Result;
use crate::filter::CityFilter;
use crate::model::Coordinates;
use crate::model::MapData;
use crate::osm_api;
use crate::pipe::Pipe;
use axum::extract::Path;
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use sqlx::FromRow;
use sqlx::Sqlite;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MapRequest {
	query: String,
	city: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Credentials {
	username: String,
	password: String,
}

#[derive(FromRow)]
struct User {
	id: i64,
}

#[derive(FromRow, Debug, Clone)]
struct CoordinateIds {
	coordinateId: i64,
}

pub async fn main_response_mapper(res: Response) -> Response {
	res
}

pub async fn request_handler(Json(request): Json<MapRequest>) -> Result<Json<MapData>> {
	let json_coordinates = osm_api::query_coordinates(&request.query).await?;
	let coordinates_json: MapData = serde_json::from_str(&json_coordinates)?;

	let json_boundaries = osm_api::query_city_boundaries(&request.city).await?;
	let city_boundaries_json: MapData = serde_json::from_str(&json_boundaries)?;

	let db = Database::connect().await?;

	let cmd = "CREATE TABLE IF NOT EXISTS coordinates (
		id INTEGER PRIMARY KEY,
		lat REAL NOT NULL,
		lon REAL NOT NULL
	);";
	sqlx::query(&cmd).execute(&db.db).await?;

	let cmd = format!(
		"CREATE TABLE IF NOT EXISTS {} (
				id INTEGER PRIMARY KEY,
				lat REAL NOT NULL,
				lon REAL NOT NULL
			);",
		&request.city
	);

	sqlx::query(&cmd).execute(&db.db).await?;

	let cmd = "INSERT OR IGNORE INTO coordinates (id, lat, lon) VALUES (?1, ?2, ?3)";
	for element in &coordinates_json.coordinates {
		if element.lat != 0.0 && element.lat != 0.0 {
			sqlx::query(&cmd)
				.bind(&element.id)
				.bind(&element.lat)
				.bind(&element.lon)
				.execute(&db.db)
				.await?;
		}
	}

	let cmd = format!("INSERT OR IGNORE INTO {} (id, lat, lon) VALUES (?1, ?2, ?3)", request.city);
	for element in &city_boundaries_json.coordinates {
		if element.lat != 0.0 && element.lat != 0.0 {
			sqlx::query(&cmd)
				.bind(&element.id)
				.bind(&element.lat)
				.bind(&element.lon)
				.execute(&db.db)
				.await?;
		}
	}

	let cmd = "SELECT * FROM coordinates";
	let coordinates = MapData::from(sqlx::query_as(&cmd).fetch_all(&db.db).await?);

	let cmd = format!("SELECT * FROM {}", &request.city);
	let city_boundaries = MapData::from(sqlx::query_as(&cmd).fetch_all(&db.db).await?);

	let mut pipe: Pipe<MapData> = Pipe::new();
	pipe.add_filter(Box::new(CityFilter::new(city_boundaries)));
	let result = pipe.run_filters(coordinates);

	db.db.close().await;

	Ok(Json(result))
}

pub async fn signup_handler(Json(credentials): Json<Credentials>) -> Result<Json<Value>> {
	let db = Database::connect().await?;

	let cmd = "CREATE TABLE IF NOT EXISTS users (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		username TEXT NOT NULL,
		password TEXT NOT NULL
	);";
	sqlx::query(&cmd).execute(&db.db).await?;

	let cmd = "SELECT * FROM users WHERE username = ?1";
	let user = sqlx::query(&cmd)
		.bind(&credentials.username)
		.fetch_optional(&db.db)
		.await?;

	if user.is_some() {
		return Err(Error::Signup);
	}

	let cmd = "INSERT OR IGNORE INTO users (username, password) VALUES (?1, ?2)";
	sqlx::query(&cmd)
		.bind(&credentials.username)
		.bind(&credentials.password)
		.execute(&db.db)
		.await?;

	let body = json!({
		"result": {
			"success": true
		}
	});

	db.db.close().await;

	Ok(Json(body))
}

pub async fn login_handler(cookies: Cookies, Json(credentials): Json<Credentials>) -> Result<Json<Value>> {
	let db = Database::connect().await?;

	let cmd = "SELECT * FROM users WHERE username = ?1 AND password = ?2";
	sqlx::query(&cmd)
		.bind(&credentials.username)
		.bind(&credentials.password)
		.fetch_optional(&db.db)
		.await?
		.ok_or(Error::Login)?;

	let cmd = "SELECT * FROM users WHERE username = ?1 AND password = ?2";
	let user = sqlx::query_as::<Sqlite, User>(&cmd)
		.bind(&credentials.username)
		.bind(&credentials.password)
		.fetch_optional(&db.db)
		.await?
		.ok_or(Error::Login)?;

	// TODO: generate real auth token
	// TODO: validate token

	let mut cookie = Cookie::new(AUTH_TOKEN, format!("{}-{}.{}.{}", "user", user.id, "exp", "sign"));
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookies.add(cookie);

	let body = json!({
		"result": {
			"success": true
		}
	});

	db.db.close().await;

	Ok(Json(body))
}

pub async fn save_handler(cookies: Cookies, Json(ids): Json<Vec<String>>) -> Result<impl IntoResponse> {
	let db = Database::connect().await?;

	let cmd = "CREATE TABLE IF NOT EXISTS userCoordinates (
	  id INTEGER PRIMARY KEY AUTOINCREMENT,
	  userId INTEGER,
	  coordinateId INTEGER
	);";
	sqlx::query(&cmd).execute(&db.db).await?;

	let (id, _, _) = auth::parse_token(cookies).await?;

	let cmd = "SELECT * FROM users WHERE id = ?1";
	let user_id = sqlx::query_as::<Sqlite, User>(&cmd)
		.bind(&id)
		.fetch_optional(&db.db)
		.await?
		.ok_or(Error::Login)?
		.id;

	let cmd = "INSERT INTO userCoordinates (userId, coordinateId) SELECT ?1, ?2 WHERE NOT EXISTS (SELECT 1 FROM userCoordinates WHERE userId = ?1 AND coordinateId = ?2);";
	for id in ids.iter() {
		let number: std::result::Result<i64, _> = id.parse();
		sqlx::query(&cmd)
			.bind(&user_id)
			.bind(&number?)
			.execute(&db.db)
			.await?;
	}

	db.db.close().await;

	Ok((StatusCode::OK, "OK"))
}

pub async fn get_handler(cookies: Cookies) -> Result<Json<MapData>> {
	let db = Database::connect().await?;

	let (user_id, _, _) = auth::parse_token(cookies).await?;

	let cmd = "SELECT * FROM userCoordinates WHERE userId = ?1";
	let coordinate_ids = sqlx::query_as::<Sqlite, CoordinateIds>(&cmd)
		.bind(&user_id)
		.fetch_all(&db.db)
		.await?;

	let mut coordinates = Vec::new();
	let cmd = "SELECT * FROM coordinates WHERE id = ?1";
	for coordinate_id in coordinate_ids {
		let coordinate = sqlx::query_as::<Sqlite, Coordinates>(&cmd)
			.bind(&coordinate_id.coordinateId)
			.fetch_optional(&db.db)
			.await?
			.ok_or(Error::Unknown)?;
		coordinates.push(coordinate);
	}

	let map_data = MapData { coordinates };

	db.db.close().await;

	Ok(Json(map_data))
}

pub async fn delete_handler(cookies: Cookies, Path(id): Path<i64>) -> Result<impl IntoResponse> {
	let db = Database::connect().await?;
	let (user_id, _, _) = auth::parse_token(cookies).await?;
	let cmd = "DELETE FROM userCoordinates WHERE userId = ?1 AND coordinateId = ?2";
	sqlx::query(&cmd)
		.bind(&user_id)
		.bind(&id)
		.execute(&db.db)
		.await?;

	db.db.close().await;

	Ok((StatusCode::OK, "OK"))
}
