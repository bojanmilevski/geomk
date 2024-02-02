use crate::errors::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use sqlx::SqlitePool;

const DB_NAME: &str = "database";

pub struct Database {
	pub db: SqlitePool,
}

impl Database {
	pub async fn connect() -> Result<Self> {
		let db_url = format!("sqlite://{}.db", DB_NAME);

		if !Sqlite::database_exists(&db_url).await? {
			Sqlite::create_database(&db_url).await?;
		}

		let db = SqlitePool::connect(&db_url).await?;

		Ok(Self { db })
	}
}
