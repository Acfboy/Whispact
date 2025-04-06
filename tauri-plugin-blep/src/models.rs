use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendRequest {
  pub message: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendResponse {
  pub success: bool
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchRecvPayload {
  pub channel: Channel,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecvEvent {
  pub status: String,
  pub message: String
}

impl RecvEvent {
  pub fn error(s: String) -> Self {
    RecvEvent {
      status: "error".to_string(),
      message: s,
    }
  }
}