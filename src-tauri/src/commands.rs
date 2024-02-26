use log;
use sqlx::Execute;
use tauri::State;
use tera::Context;

use super::templating;
use crate::db::Db;
use crate::models::Log;

#[tauri::command]
pub async fn logs(db: State<'_, Db>) -> Result<String, ()> {
    let mut context = Context::new();
    let logs: Result<Vec<Log>, _> =
        sqlx::query_as::<_, Log>(r"SELECT id, date, data FROM logs ORDER BY date DESC")
            .fetch_all(&**db)
            .await;

    log::debug!("{:?}", logs);
    context.insert("logs", &logs.unwrap_or_default());
    Ok(templating::TEMPLATES
        .render("logentry.html", &context)
        .unwrap())
}

#[tauri::command]
pub async fn refresh_logs<'a>(db: State<'_, Db>, date: &'a str) -> Result<String, ()> {
    let mut context = Context::new();
    let logs: Result<Vec<Log>, _> = sqlx::query_as::<_, Log>(
        format!(
            r"SELECT * FROM logs WHERE DATETIME(date) > DATETIME('{}') ORDER BY date DESC",
            date
        )
        .as_str(),
    )
    .fetch_all(&**db)
    .await;

    let logs = logs.unwrap_or_default();
    log::debug!("Retrieved {} new logs", logs.len());

    context.insert("logs", &logs);
    Ok(templating::TEMPLATES
        .render("newlogs.html", &context)
        .unwrap())
}

#[tauri::command]
pub async fn delete_log<'a>(db: State<'_, Db>, id: &'a str) -> Result<String, ()> {
    let statement = sqlx::query(r"DELETE FROM logs WHERE id LIKE ?;")
        .bind(id)
        .execute(&**db)
        .await;

    Ok(templating::TEMPLATES
        .render("notice.html", &Context::new())
        .unwrap())
}
