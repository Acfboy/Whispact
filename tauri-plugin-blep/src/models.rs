use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;
use tauri::plugin::PermissionState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub bluetooth: PermissionState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermission {
    pub bluetooth: bool,
}

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
    pub uuid: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Deserialize, Default)]
pub struct RecvMessage {
    pub msg: String,
}
