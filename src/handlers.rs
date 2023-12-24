use crate::auth;
use crate::auth::AUTH_TOKEN;
use crate::errors::Error;
use crate::errors::Result;
use crate::filter::CityFilter;
use crate::model::MapData;
use crate::osm_api;
use crate::pipe::Pipe;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use sqlx::migrate::MigrateDatabase;
use sqlx::FromRow;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

const DB_NAME: &str = "database";

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

pub async fn main_response_mapper(res: Response) -> Response {
	res
}

pub async fn handle_request(Json(request): Json<MapRequest>) -> Result<Json<MapData>> {
	let json_coordinates = osm_api::query_coordinates(&request.query).await?;
	let coordinates_json: MapData = serde_json::from_str(&json_coordinates)?;

	let json_boundaries = osm_api::query_city_boundaries(&request.city).await?;
	let city_boundaries_json: MapData = serde_json::from_str(&json_boundaries)?;

	let db_url = format!("sqlite://{}.db", DB_NAME);

	if !Sqlite::database_exists(&db_url).await? {
		Sqlite::create_database(&db_url).await?;
	}

	let db = SqlitePool::connect(&db_url).await?;

	let cmd = format!(
		"CREATE TABLE IF NOT EXISTS coordinates (
				id INTEGER PRIMARY KEY,
				lat REAL NOT NULL,
				lon REAL NOT NULL
			);"
	);

	sqlx::query(&cmd).execute(&db).await?;

	let cmd = format!(
		"CREATE TABLE IF NOT EXISTS {} (
				id INTEGER PRIMARY KEY,
				lat REAL NOT NULL,
				lon REAL NOT NULL
			);",
		&request.city
	);

	sqlx::query(&cmd).execute(&db).await?;

	let cmd = format!("INSERT OR IGNORE INTO coordinates (id, lat, lon) VALUES (?1, ?2, ?3)");
	for element in &coordinates_json.coordinates {
		if element.lat != 0.0 && element.lat != 0.0 {
			sqlx::query(&cmd)
				.bind(&element.id)
				.bind(&element.lat)
				.bind(&element.lon)
				.execute(&db)
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
				.execute(&db)
				.await?;
		}
	}

	let cmd = format!("SELECT * FROM coordinates");
	let coordinates = MapData::from(sqlx::query_as(&cmd).fetch_all(&db).await?);

	let cmd = format!("SELECT * FROM {}", &request.city);
	let city_boundaries = MapData::from(sqlx::query_as(&cmd).fetch_all(&db).await?);

	db.close().await;

	let mut pipe: Pipe<MapData> = Pipe::new();
	pipe.add_filter(Box::new(CityFilter::new(city_boundaries)));
	let result = pipe.run_filters(coordinates);

	Ok(Json(result))
}

pub async fn signup_handler(Json(credentials): Json<Credentials>) -> Result<Json<Value>> {
	let db_url = format!("sqlite://{}.db", DB_NAME);

	if !Sqlite::database_exists(&db_url).await? {
		Sqlite::create_database(&db_url).await?;
	}

	let db = SqlitePool::connect(&db_url).await?;

	sqlx::query(
		"CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL,
        password TEXT NOT NULL
    );",
	)
	.execute(&db)
	.await?;

	let user = sqlx::query("SELECT * FROM users WHERE username = ?1")
		.bind(&credentials.username)
		.fetch_optional(&db)
		.await?;

	if user.is_some() {
		return Err(Error::Signup);
	}

	sqlx::query("INSERT OR IGNORE INTO users (username, password) VALUES (?1, ?2)")
		.bind(&credentials.username)
		.bind(&credentials.password)
		.execute(&db)
		.await?;

	db.close().await;

	let body = json!({
		"result": {
			"success": true
		}
	});

	Ok(Json(body))
}

pub async fn login_handler(cookies: Cookies, Json(credentials): Json<Credentials>) -> Result<Json<Value>> {
	let db_url = format!("sqlite://{}.db", DB_NAME);

	if !Sqlite::database_exists(&db_url).await? {
		Sqlite::create_database(&db_url).await?;
	}

	let db = SqlitePool::connect(&db_url).await?;

	sqlx::query("SELECT * FROM users WHERE username = ?1 AND password = ?2")
		.bind(&credentials.username)
		.bind(&credentials.password)
		.fetch_optional(&db)
		.await?
		.ok_or(Error::Login)?;

	let user = sqlx::query_as::<Sqlite, User>("SELECT * FROM users WHERE username = ?1 AND password = ?2")
		.bind(&credentials.username)
		.bind(&credentials.password)
		.fetch_optional(&db)
		.await?
		.ok_or(Error::Login)?;

	db.close().await;
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

	Ok(Json(body))
}

pub async fn save_handler(cookies: Cookies, Json(ids): Json<Vec<String>>) -> Result<Json<i64>> {
	let db_url = format!("sqlite://{}.db", DB_NAME);

	if !Sqlite::database_exists(&db_url).await? {
		Sqlite::create_database(&db_url).await?;
	}

	let db = SqlitePool::connect(&db_url).await?;

	sqlx::query(
		"CREATE TABLE IF NOT EXISTS userCoordinates (
        userId INTEGER,
        coordinateId INTEGER
    );",
	)
	.execute(&db)
	.await?;

	let (id, _, _) = auth::parse_token(cookies).await?;
	let user_id = sqlx::query_as::<Sqlite, User>("SELECT * FROM users WHERE id = ?1")
		.bind(&id)
		.fetch_optional(&db)
		.await?
		.ok_or(Error::Login)?
		.id;

	for id in ids.iter() {
		let number: std::result::Result<i64, _> = id.parse();
		sqlx::query("INSERT INTO userCoordinates (userId, coordinateId) VALUES (?1, ?2)")
			.bind(&user_id)
			.bind(&number?)
			.execute(&db)
			.await?;
	}

	db.close().await;

	Ok(Json(Default::default()))
}
