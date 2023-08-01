use async_trait::async_trait;
use sqlx::{
    migrate::MigrateDatabase,
    types::chrono::{DateTime, NaiveDateTime, Utc},
    Connection, Pool, Row, Sqlite, SqliteConnection, SqlitePool,
};
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
            DUE_DATE DATETIME NOT NULL,
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

#[derive(sqlx::FromRow, Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub finished: bool,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait TaskCrud {
    async fn get_tasks(&self) -> Vec<Task>;
    async fn insert_task(&self, description: &String, due_date: &NaiveDateTime) -> String;
    async fn update_task(&self, item_id: &String, finished: &bool) -> String;
}

#[async_trait]
impl TaskCrud for MyApp {
    async fn get_tasks(&self) -> Vec<Task> {
        let rows = sqlx::query(
            "SELECT id, description, finished, created_at FROM Tasks ORDER BY CREATED_AT;",
        )
        .fetch_all(&self.db_connection)
        .await
        .unwrap();

        let tasks = rows
            .iter()
            .map(|row| {
                let id = row.get::<String, _>(0);
                let description = row.get::<String, _>(1);
                let finished = row.get::<bool, _>(2);
                let created_at = row.get::<DateTime<Utc>, _>(3);

                Task {
                    description,
                    id,
                    finished,
                    created_at,
                }
            })
            .collect::<Vec<_>>();

        return tasks;
    }

    async fn insert_task(&self, description: &String, due_date: &NaiveDateTime) -> String {
        let id = Uuid::new_v4();
        sqlx::query("INSERT into Tasks (ID, DESCRIPTION, DUE_DATE) values ($1, $2, $3)")
            .bind(id.to_string())
            .bind(description)
            .bind(due_date.to_string())
            .execute(&self.db_connection)
            .await
            .unwrap();

        return id.to_string();
    }

    async fn update_task(&self, item_id: &String, finished: &bool) -> String {
        sqlx::query("UPDATE Tasks SET FINISHED = $1 WHERE ID = $2;")
            .bind(finished)
            .bind(item_id)
            .execute(&self.db_connection)
            .await
            .unwrap();

        return "Done".to_string();
    }
}
