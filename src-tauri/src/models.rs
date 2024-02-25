use crate::payloads::PayloadData;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
use uuid::fmt::Hyphenated;

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    #[sqlx(try_from = "Hyphenated")]
    id: Uuid,
    date: chrono::DateTime<chrono::Utc>,
    data: sqlx::types::Json<PayloadData>,
}
