use crate::payloads::Payload;
use rocket::serde::json::Json;
use rocket::{post, State};
use std::sync::mpsc::Sender;

#[post("/ingest", format = "application/json", data = "<payload>")]
pub async fn ingest(tx_payload: &State<Sender<Payload>>, payload: String) -> Json<String> {
    let p: Result<Payload, _> = serde_json::from_str(&payload[..]);
    match p {
        Ok(payload) => match tx_payload.send(payload) {
            Ok(_) => (),
            Err(e) => {
                log::debug!("Error sending event: {}", e);
                return Json(e.to_string());
            }
        },
        Err(e) => {
            log::debug!("Error: {}", e);
            return Json(e.to_string());
        }
    };

    return Json("".to_string());
}
