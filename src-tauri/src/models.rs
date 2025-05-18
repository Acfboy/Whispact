use serde::{Deserialize, Serialize};
use tauri_plugin_blep::mobile::Plan;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    BleCentralDiscover(String),
    BleCentralConnect(String),
    BleCentralSubscribe(String),
    BleCentralDeviceNotFound,
    BleCenteralSendDataFailed(String),
    LastMessageNotSend,
    SendBeforeConnect,
    ReceiveBeforeConnect,
    ConnectBeforeSetup,
    BlePeripheralSendFail(String),
    RequestBlueTooth(String),
    InitNfc(String),
    Lucky(String),
    Store(String),
    Load(String),
}

#[derive(Deserialize, Serialize)]
pub struct MessageDraft {
    title: String,
    body: String,
}

#[derive(Deserialize, Serialize)]
pub struct DisposableDrafts {
    pub drafts: Vec<MessageDraft>,
}

#[derive(Deserialize, Serialize)]
pub struct BackToBackDrafs {
    pub drafts: Vec<MessageDraft>,
}

#[derive(Deserialize, Serialize)]
pub struct Instance {
    pub instance: String,
    pub time: String,
}

#[derive(Deserialize, Serialize)]
pub struct SealedInstances {
    pub instances: Vec<Instance>,
}

#[derive(Deserialize, Serialize)]
pub struct FinishedPlan {
    pub plan: Plan,
    pub time: String,
}

#[derive(Deserialize, Serialize)]
pub struct FinishedPlanList {
    pub list: FinishedPlan,
}

impl Into<Error> for tauri_plugin_store::Error {
    fn into(self) -> Error {
        Error::Store(self.to_string())
    }
}
