use crate::{db::Db, payloads::Payload};
use tauri::Manager;

pub async fn refresh<T: tauri::Runtime>(payload: Payload, manager: &impl Manager<T>) {
    let db = manager.state::<Db>();
    let payload_value: serde_json::Value = serde_json::value::to_value(&payload).unwrap();
    println!("{:?}", payload_value);

    let id = sqlx::query(r#"INSERT INTO logs (data, source, level, color) VALUES (?, ?, ?, ?)"#)
        .bind(&payload_value["payload_data"])
        .bind(&payload_value.get("source").unwrap())
        .bind(&payload_value["level"].as_str())
        .bind(&payload_value["color"].as_str())
        .execute(&**db)
        .await;

    match id {
        Err(e) => {
            log::debug!("Error inserting payload {:?}: {}", payload_value, e);
        }
        Ok(id) => {
            println!("Test");
            log::debug!("Inserted. Number of rows affected: {:?}", id.last_insert_rowid());
            manager
                .emit_all("newlog", id.last_insert_rowid())
                .unwrap();
        }
    }
}
