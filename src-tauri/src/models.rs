use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri_plugin_blep::mobile::{Message, Plan};
use uuid::Uuid;

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
    Unsupport(String),
}

#[derive(Deserialize, Serialize)]
pub struct MessageDraft {
    title: String,
    body: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct DisposableDrafts {
    pub drafts: Vec<MessageDraft>,
}

#[derive(Deserialize, Serialize)]
pub struct Instance {
    pub instance: String,
    pub time: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct SealedInstances {
    pub instances: Vec<Instance>,
}

#[derive(Deserialize, Serialize)]
pub struct FinishedPlan {
    pub plan: Plan,
    pub time: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct PlanDrafts {
    pub drafts: HashMap<Uuid, Plan>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct FinishedPlanList {
    pub list: Vec<FinishedPlan>,
}

impl From<tauri_plugin_store::Error> for Error {
    fn from(value: tauri_plugin_store::Error) -> Self {
        Error::Store(value.to_string())
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum MessageType {
    Disposable,
    Seal,
    PlanSync,
    Mail,
    Empty,
}

impl MessageType {
    pub fn from(val: &Message) -> Self {
        match val {
            Message::Disposable(_) => Self::Disposable,
            Message::Empty => Self::Empty,
            Message::PlanSync(_) => Self::PlanSync,
            Message::Seal(_) => Self::Seal,
            Message::Mail(_) => Self::Mail,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Mail {
    cover: String,
    inner: MailInner,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MailInner {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, Default)]
struct MailCover {
    sealed: bool,
    cover: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MailCoverList {
    mails: HashMap<Uuid, MailCover>,
}
