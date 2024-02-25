use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use serde_json::json;

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(tag = "type", content = "data")]
#[serde(tag = "type")]
pub enum PayloadData {
    Message(MessagePayload),
    Json(JsonPayload),
    Variable(VariablePayload),
    Spacer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    #[serde(default)]
    date: DateTime<Utc>,
    // #[serde(flatten)]
    payload_data: PayloadData,
}

// impl Payload {
//     pub fn new(payload_data: PayloadData) -> Self {
//         Self {
//             date: Utc::now(),
//             payload_data,
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub message: String,
}

// impl MessagePayload {
//     pub fn new(message: String) -> Self {
//         Self { message }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacerPayload {}

// impl SpacerPayload {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonPayload {
    pub data: serde_json::Value,
    pub pretty: bool,
}

// impl JsonPayload {
//     pub fn new<T>(data: T, pretty: bool) -> Self
//     where
//         T: Serialize,
//     {
//         Self {
//             data: json!(data),
//             pretty,
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariablePayload {
    pub data: serde_json::Value,
}

// impl VariablePayload {
//     pub fn new<T>(data: T) -> Self
//     where
//         T: Serialize,
//     {
//         Self { data: json!(data) }
//     }
// }
