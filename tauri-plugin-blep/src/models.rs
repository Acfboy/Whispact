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
    /// 背对背消息
    BackToBack(String),
    /// 打卡“保存这一刻”
    Seal(String),
    /// 同步共同计划
    PlanSync(Vec<Plan>),
    /// 打卡完成共同计划，同步后只需要传一个 uuid
    PlanCheck(Uuid),
    /// 两人填写的保存这一刻不同
    DiffSeal,
    /// 两人想完成的计划不同
    DiffPlan,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Plan {
    id: Uuid,
    plan: String,
}

impl Message {
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
