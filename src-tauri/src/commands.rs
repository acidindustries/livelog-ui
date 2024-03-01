use log;
use sqlx::{Execute, QueryBuilder, Sqlite};
use tauri::{Manager, State, Window};
use tera::Context;

use super::templating;
use crate::db::Db;
use crate::models::Log;

#[tauri::command]
pub async fn logs(db: State<'_, Db>) -> Result<String, ()> {
    let mut context = Context::new();
    let logs: Result<Vec<Log>, _> =
        sqlx::query_as::<_, Log>(r"SELECT id, date, data, source, color, level, extras FROM logs ORDER BY rowid DESC")
            .fetch_all(&**db)
            .await;
    match logs {
        Ok(logs) => {
            log::debug!("{:?}", logs);
            context.insert("logs", &logs);
            Ok(templating::TEMPLATES
                .render("logentry.html", &context)
                .unwrap())
        },
        Err(e) => Ok(format!("Error {}", e)),
    }
}

#[tauri::command]
pub async fn apply_filters(db: State<'_, Db>, color: Option<String>, level: Option<String>, search: Option<String>) -> Result<String, ()> {
        println!("Apply filters Colors: {:?}", color);
        println!("Input filter: {:?}", search);
    let mut context = Context::new();
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(r#"SELECT id, date, data, source, color, level, extras FROM logs"#);

    let mut some_search = false;
    let mut some_color = false;
    if search.is_some() || color.is_some() || level.is_some() {
        query_builder.push(" WHERE");
        if search.is_some() {
            some_search = true;
        }

        if search.is_some() {
            some_color = true;
        }
    }

    if let Some(search) = search {
        if !search.is_empty() {
            query_builder
                .push(r" (json_extract(logs.data, '$.data', '$.message') LIKE ")
                .push_bind(format!("%{}%", search.clone()))
                .push(r" OR json_extract(logs.source, '$.file', '$.line') LIKE ")
                .push_bind(format!("%{}%", search))
                .push(")");
        }
        else {
            some_search = false;
        }
    }

    if let Some(colors) = color {
        let mut nullable = false;
        if some_search {
            query_builder.push(" AND");
        }
        let colors = colors
                        .split(",")
                        .filter(|color| !color.is_empty())
                        .collect::<Vec<&str>>();

        if colors.contains(&"null") {
            nullable = true;
        }

        let colors = colors
                        .iter()
                        .map(|color| format!("'{}'", color))
                        .collect::<Vec<String>>()
                        .join(",");

        query_builder.push(" (color IN (");
        query_builder.push(colors.clone());
        println!("{}", colors);
        query_builder.push(")");
        if nullable {
            if colors.len() > 0 {
                query_builder.push(" OR");
            }
            query_builder.push(" color IS NULL");
        }
        query_builder.push(")");
    }

    if let Some(levels) = level {
        if some_color {
            query_builder.push(" AND");
        }
        let levels = levels
                        .split(",")
                        .filter(|level| !level.is_empty())
                        .collect::<Vec<&str>>();

        let levels = levels
                        .iter()
                        .map(|level| format!("'{}'", level))
                        .collect::<Vec<String>>()
                        .join(",");

        query_builder.push(" (level IN (");
        query_builder.push(levels.clone());
        println!("{}", levels);
        query_builder.push("))");
    }
    query_builder.push(" ORDER BY rowid DESC");

    // let logs: Result<Vec<Log>, _> =
    //         sqlx::query_as::<_, Log>(r"SELECT id, date, data, source, color, level, extras FROM logs WHERE json_extract(logs.data, '$.data', '$.message') LIKE ? OR json_extract(logs.source, '$.file', '$.line') LIKE ? ORDER BY rowid DESC")
    //         .bind(format!("%{}%", search.unwrap()))
    //         .fetch_all(&**db)
    //         .await;
    // println!("Query: {}", query_builder.build().sql());
    let logs: Result<Vec<Log>, _> = query_builder.build_query_as::<Log>().fetch_all(&**db).await;
    match logs {
        Ok(logs) => {
            println!("{:?}", logs);
            log::debug!("{:?}", logs);
            context.insert("logs", &logs);
            Ok(templating::TEMPLATES
                .render("logentry.html", &context)
                .unwrap())
        },
        Err(e) => Ok(format!("Error {}", e)),
    }
}

#[tauri::command]
pub async fn refresh_logs(db: State<'_, Db>, id: i64) -> Result<String, ()> {
    let mut context = Context::new();
    let log: Result<Log, _> = sqlx::query_as::<_, Log>(
            r"SELECT * FROM logs WHERE rowid = ?;",
    )
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
        Err(_) => {
            return Ok("".to_string())
        },
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
    let statement = sqlx::query("DELETE FROM logs;")
    .execute(&**db)
    .await;

    match statement {
        Ok(_) => {
            window.emit_all("clearlogs", ()).unwrap();
        }
        Err(_) => ()
    };
    Ok(templating::TEMPLATES
        .render("logentry.html", &Context::new())
        .unwrap())
}
