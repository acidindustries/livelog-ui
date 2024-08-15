use log;
use sqlx::{QueryBuilder, Sqlite};
use tauri::{Manager, State, Window};
use tera::Context;

use super::templating;
use crate::db::Db;
use crate::filters::Filter;
use crate::models::Log;

#[tauri::command]
pub async fn logs(db: State<'_, Db>) -> Result<String, ()> {
    let mut context = Context::new();
    let logs: Result<Vec<Log>, _> = sqlx::query_as::<_, Log>(
        r"SELECT id, date, data, source, color, level, extras FROM logs ORDER BY date DESC",
    )
    .fetch_all(&**db)
    .await;
    match logs {
        Ok(logs) => {
            log::debug!("{:?}", logs);
            context.insert("logs", &logs);
            Ok(templating::TEMPLATES
                .render("logentry.html", &context)
                .unwrap())
        }
        Err(e) => Ok(format!("Error {}", e)),
    }
}

#[tauri::command]
pub async fn apply_filters(
    db: State<'_, Db>,
    color: Option<String>,
    level: Option<String>,
    search: Option<String>,
) -> Result<String, ()> {
    println!("Apply filters Colors: {:?}", color);
    println!("Input filter: {:?}", search);
    let mut context = Context::new();
    let mut query_builder: QueryBuilder<'_, Sqlite> =
        QueryBuilder::new(r#"SELECT id, date, data, source, color, level, extras FROM logs"#);

    let colors = Some(color).unwrap().filter(|color| !color.is_empty());

    let t = Filter::new(colors, level, search);

    let logs: Result<Vec<Log>, _> = t.get_query(&mut query_builder).fetch_all(&**db).await;
    match logs {
        Ok(logs) => {
            println!("{:?}", logs);
            log::debug!("{:?}", logs);
            context.insert("logs", &logs);
            Ok(templating::TEMPLATES
                .render("logentry.html", &context)
                .unwrap())
        }
        Err(e) => Ok(format!("Error {}", e)),
    }
}

#[tauri::command]
pub async fn refresh_logs(db: State<'_, Db>, id: i64) -> Result<String, ()> {
    let mut context = Context::new();
    let log: Result<Log, _> = sqlx::query_as::<_, Log>(r"SELECT * FROM logs WHERE rowid = ?;")
        .bind(id)
        .fetch_one(&**db)
        .await;

    match log {
        Ok(log) => {
            context.insert("log", &log);
            return Ok(templating::TEMPLATES
                .render("newlogs.html", &context)
                .unwrap());
        }
        Err(_) => return Ok("".to_string()),
    }
}

#[tauri::command]
pub async fn delete_log<'a>(window: Window, db: State<'_, Db>, id: &'a str) -> Result<String, ()> {
    let statement = sqlx::query(r"DELETE FROM logs WHERE id LIKE ?;")
        .bind(id)
        .execute(&**db)
        .await;

    match statement {
        Ok(_) => {
            let _ = window.emit_all("refresh", ());
            return Ok("".to_string());
        }
        Err(_) => {
            // return Ok(templating::TEMPLATES
            // .render("notice.html", &Context::new())
            // .unwrap());
            todo!();
        }
    }
}

#[tauri::command]
pub async fn clear_all<'a>(window: Window, db: State<'_, Db>) -> Result<String, ()> {
    let statement = sqlx::query("DELETE FROM logs;").execute(&**db).await;

    match statement {
        Ok(_) => {
            window.emit_all("clearlogs", ()).unwrap();
        }
        Err(_) => (),
    };
    Ok(templating::TEMPLATES
        .render("logentry.html", &Context::new())
        .unwrap())
}
