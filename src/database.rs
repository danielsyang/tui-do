use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

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

    return SqlitePool::connect(DB_URL).await.unwrap();
}

pub trait TaskCrud {
    fn insert_task(&self) -> String;
}

impl TaskCrud for MyApp {
    fn insert_task(&self) -> String {
        todo!("insert task into database")
    }
}
