use async_trait::async_trait;
use sqlx::{migrate::MigrateDatabase, Connection, Pool, Sqlite, SqliteConnection, SqlitePool};
use uuid::Uuid;

use crate::app::MyApp;

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn connection() -> Pool<Sqlite> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {} ", DB_URL);

        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(err) => panic!("Error: {}", err),
        }
    };

    let mut conn = SqliteConnection::connect(DB_URL).await.unwrap();

    sqlx::query(
        "
        CREATE TABLE if not EXISTS Tasks (	
            ID TEXT PRIMARY KEY,
            DESCRIPTION TEXT NOT NULL,
            FINISHED BOOLEAN NOT NULL DEFAULT FALSE,
            CREATED_AT DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&mut conn)
    .await
    .unwrap();

    conn.close().await.unwrap();

    return SqlitePool::connect(DB_URL).await.unwrap();
}

#[async_trait]
pub trait TaskCrud {
    async fn insert_task(&self, description: &String) -> String;
}

#[async_trait]
impl TaskCrud for MyApp {
    async fn insert_task(&self, description: &String) -> String {
        let id = Uuid::new_v4();
        sqlx::query("INSERT into Tasks (ID, DESCRIPTION) values ($1, $2)")
            .bind(id.to_string())
            .bind(description)
            .execute(&self.db_connection)
            .await
            .unwrap();

        return id.to_string();
    }
}
