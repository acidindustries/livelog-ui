use crate::payloads::Payload;
use rocket::{get, post, State};
use std::sync::mpsc::Sender;

#[get("/test")]
pub fn test() -> &'static str {
    "Test"
}

#[post("/ingest", format = "application/json", data = "<payload>")]
pub async fn ingest(tx_payload: &State<Sender<Payload>>, payload: String) {
    let p: Payload = serde_json::from_str(&payload[..]).unwrap();
    println!("{:?}", p);
    match tx_payload.send(p) {
        Ok(_) => (),
        Err(e) => log::debug!("Error sending event: {}", e),
    }
}
