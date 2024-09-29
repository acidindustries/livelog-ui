use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum PayloadData {
    Message(MessagePayload),
    Json(JsonPayload),
    Variable(VariablePayload),
    Spacer,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceData {
    pub file: String,
    pub line: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    date: DateTime<Utc>,
    payload_data: PayloadData,
    source: Option<SourceData>,
    color: Option<String>,
    level: Option<String>,
    extras: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacerPayload {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonPayload {
    pub data: serde_json::Value,
    pub pretty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariablePayload {
    pub data: serde_json::Value,
}
