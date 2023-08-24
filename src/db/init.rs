use sqlx::Connection;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}


async fn get_db() -> sqlx::Result<sqlx::SqliteConnection> {
    sqlx::SqliteConnection::connect("sqlite:todo.db").await
}

pub async fn connect() -> sqlx::SqliteConnection {
    //create table todo if not exists
    let mut conn = get_db().await.expect("Error connecting to db");
    let _ = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )
        "#,
    ).execute(&mut conn).await.unwrap();
    conn
}