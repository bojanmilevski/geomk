use crate::map_data::Coordinates;
use crate::map_data::MapData;
use crate::Result;

use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use sqlx::SqlitePool;

pub struct Database {
	db: SqlitePool,
}

impl Database {
	fn make_command(table_name: &str) -> String {
		format!(
			"CREATE TABLE IF NOT EXISTS {table_name} (
				id INTEGER PRIMARY KEY,
				lat REAL NOT NULL,
				lon REAL NOT NULL
			);"
		)
	}

	pub async fn new(db_name: &str, city: &str) -> Result<Self> {
		let url = format!("sqlite://{db_name}.db");

		if !Sqlite::database_exists(&url).await? {
			Sqlite::create_database(&url).await?;
		}

		let db = SqlitePool::connect(&url).await?;

		sqlx::query(&Self::make_command("coordinates"))
			.execute(&db)
			.await?;

		sqlx::query(&Self::make_command(&city)).execute(&db).await?;

		Ok(Self { db })
	}

	pub async fn insert_data(&self, data: &MapData, to: &str) -> Result<()> {
		let query_command = format!("INSERT OR IGNORE INTO {to} (id, lat, lon) VALUES (?1, ?2, ?3)");

		for element in &data.coordinates {
			if element.lat != 0.0 && element.lat != 0.0 {
				sqlx::query(&query_command)
					.bind(&element.id)
					.bind(&element.lat)
					.bind(&element.lon)
					.execute(&self.db)
					.await?;
			}
		}

		Ok(())
	}

	pub async fn select_data(&self, from: &str) -> Result<MapData> {
		let query_command = format!("SELECT id, lat, lon FROM {from}");
		let coordinates: Vec<Coordinates> = sqlx::query_as(&query_command).fetch_all(&self.db).await?;

		Ok(MapData { coordinates })
	}
}
