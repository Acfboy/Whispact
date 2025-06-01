use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;
use tauri::plugin::PermissionState;
use uuid::Uuid;

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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecvData {
    pub msg: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchRecvPayload {
    pub channel: Channel,
    pub connect_notifier: Channel,
    pub uuid: String,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

/// 手机间通信的信号
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Message {
    /// 一次性消息
    Disposable(String),
    /// 打卡“保存这一刻”
    Seal(String),
    /// 同步共同计划
    PlanSync(Plans),
    /// 没有消息需要传递。作为信号量通知 notification 监听已经建立，可以通信。
    ///
    /// 即每次通信由主端先发送信息，发送信息表明监听已经建立，从端可以发送。
    Empty,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Plans {
    selected_plan: Option<Uuid>,
    plans: HashMap<Uuid, Plan>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plan {
    title: String,
    body: String,
}

impl Message {
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
