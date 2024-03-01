use std::collections::HashMap;

use crate::payloads::{PayloadData, SourceData};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
use uuid::fmt::Hyphenated;

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    #[sqlx(try_from = "Hyphenated")]
    id: Uuid,
    date: chrono::DateTime<chrono::Utc>,
    data: sqlx::types::Json<PayloadData>,
    source: Option<sqlx::types::Json<SourceData>>,
    color: Option<String>,
    level: Option<String>,
    extras: Option<sqlx::types::Json<HashMap<String, String>>>,
}
