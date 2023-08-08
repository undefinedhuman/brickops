#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use home::home_dir;
use sqlx::{FromRow, migrate::MigrateDatabase, Sqlite, SqlitePool};
use tauri::State;

struct Database(SqlitePool);

#[derive(Clone, FromRow, Debug, serde::Serialize)]
struct Set {
    id: i32,
    name: String,
    category: i32,
}

#[tauri::command]
async fn get_sets<'a>(page: i32, size: i32, db: State<'a, Database>) -> Result<Vec<Set>, String> {
    return sqlx::query_as::<_, Set>(
        &format!("SELECT * FROM bricklink_sets OFFSET {} LIMIT {};", page, size)
    ).fetch_all(&db.0).await.map_err(|e| e.to_string());
}

#[tokio::main]
async fn main() {
    let home_dir = home_dir().unwrap();
    let brickops_dir = format!("{}/.brickops", home_dir.display());
    let db_url = format!("sqlite://{}/items.db", brickops_dir);

    fs::create_dir_all(&brickops_dir)
        .unwrap_or_else(|e| {
            panic!("Failed to create directory {}: {}", brickops_dir, e)
        });

    Sqlite::create_database(&db_url).await.unwrap_or_else(|e| {
        panic!("Failed to create database {}: {}", db_url, e)
    });

    let db = SqlitePool::connect(&db_url).await.unwrap();

    sqlx::migrate!().run(&db).await.unwrap_or_else(|e| {
        panic!("Failed to run migrations: {}", e)
    });

    tauri::Builder::default()
        .manage(Database(db))
        .invoke_handler(tauri::generate_handler![get_sets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
