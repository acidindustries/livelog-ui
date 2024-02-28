use crate::{db::Db, payloads::Payload};
use tauri::Manager;

pub async fn refresh<T: tauri::Runtime>(payload: Payload, manager: &impl Manager<T>) {
    let db = manager.state::<Db>();
    let payload_value: serde_json::Value = serde_json::value::to_value(&payload).unwrap();
    let result = sqlx::query(r"INSERT INTO logs (data) VALUES (?)")
        .bind(&payload_value["payload_data"])
        .execute(&**db)
        .await;

    match result {
        Err(e) => {
            println!("Error inserting payload {:?}: {}", payload_value, e);
        }
        Ok(res) => {
            println!("Inserted. Number of rows affected: {}", res.rows_affected());
            manager
                .emit("newlog", format!("payload: {:?}", payload))
                .unwrap();
        }
    }
}
