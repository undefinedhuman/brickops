#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use home::home_dir;
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone, FromRow, Debug)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let db_url: String;
    match home_dir() {
        Some(path) => {
            db_url = format!("sqlite://{}/.brickvault/items.db", path.display());
        },
        None => panic!("Could not find home directory!"),
    }

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("[Brickvault] Database created successfully!"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db = SqlitePool::connect(&db_url).await.unwrap();

    let migration_results = sqlx::migrate!().run(&db).await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table'
         AND name NOT LIKE 'sqlite_%';",
    )
        .fetch_all(&db)
        .await
        .unwrap();
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
