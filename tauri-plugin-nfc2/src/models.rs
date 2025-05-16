use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UuidResponse {
    pub value: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NfcRequest {
    pub error_channel: Channel,
    pub data_channel: Channel,
    pub uuid: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HceRequest {
    pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NfcErrorResponse {
    pub code: String,
    pub data: String,
}
