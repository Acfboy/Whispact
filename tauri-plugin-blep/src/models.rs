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
    pub success: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchRecvPayload {
    pub channel: Channel,
    pub connect_notifier: Channel,
}

#[derive(Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Deserialize, Default)]
pub struct RecvMessage {
    pub msg: String,
}
