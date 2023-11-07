use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::{
	map_data::{Coordinates, MapData},
	Result,
};

pub struct Database {
	pub db: SqlitePool,
	pub table_name: String,
}

impl Database {
	pub async fn new(db_name: &str, table_name: &str) -> Result<Self> {
		let url = format!("sqlite://{db_name}.db");

		if !Sqlite::database_exists(&url).await? {
			Sqlite::create_database(&url).await?;
		}

		let db = SqlitePool::connect(&url).await?;

		let create_table_query = format!(
			"CREATE TABLE IF NOT EXISTS {table_name} (
                id INTEGER PRIMARY KEY,
                lat REAL NOT NULL,
                lon REAL NOT NULL
            );",
		);

		sqlx::query(&create_table_query).execute(&db).await?;

		let table_name = table_name.to_string();

		Ok(Self { db, table_name })
	}

	pub async fn insert_data(&self, data: &MapData) -> Result<&Self> {
		let insert_data_query =
			format!("INSERT OR IGNORE INTO {} (id, lat, lon) VALUES (?1, ?2, ?3)", &self.table_name);

		for element in &data.elements {
			sqlx::query(&insert_data_query)
				.bind(&element.id)
				.bind(&element.lat)
				.bind(&element.lon)
				.execute(&self.db)
				.await?;
		}

		Ok(&self)
	}

	pub async fn select_data(&self) -> Result<Vec<Coordinates>> {
		let select_data_query = format!("SELECT id, lat, lon FROM {}", &self.table_name);
		let coordinates: Vec<Coordinates> = sqlx::query_as(&select_data_query)
			.fetch_all(&self.db)
			.await?;

		Ok(coordinates)
	}
}
