use dirs;
use dotenv;
use rocket_db_pools::Database;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

/// Create a Rocket fairing to bootstrap the DB.
#[derive(Database)]
#[database("logs")]
pub struct Db(pub sqlx::SqlitePool);

pub async fn init() -> SqlitePool {
    if !db_file_exists() {
        create_db_file();
    }

    let db_path = get_db_path();

    SqlitePoolOptions::new()
        .max_connections(100)
        .connect(&db_path)
        .await
        .expect(format!("Unable to connect to Sqlite database with url {}", db_path).as_str())
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    match dotenv::dotenv() {
        Ok(_) => {
            if let Ok(path) = dotenv::var("DATABASE_URL") {
                return path;
            }
        }
        Err(_) => (),
    }
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/livelog/logs.sqlite"
}
