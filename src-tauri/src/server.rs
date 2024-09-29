use std::sync::mpsc::Sender;

use crate::{endpoints, filters::Filter, payloads::Payload};
use rocket::{self, futures::lock::Mutex, routes};

pub async fn init(
    tx_channel: Sender<Payload>,
) -> Result<rocket::Rocket<rocket::Ignite>, rocket::Error> {
    rocket::build()
        .manage(tx_channel)
        .mount("/", routes![endpoints::ingest])
        .launch()
        .await
}
