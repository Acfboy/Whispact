use tauri::{command, plugin::PermissionState, AppHandle, Manager};
use tauri_plugin_blep::{
    mobile::{Message, Plans},
    BlepExt,
};
use tauri_plugin_store::StoreExt;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    ble::DeviceBridge,
    models::{
        DisposableDrafts, Error, FinishedPlanList, Mail, MailCoverList, MailInner, PlanDrafts,
        SealedInstances,
    },
};

#[command]
pub async fn set_disposable_msg(app: AppHandle, msg: String) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().await;
    (*guard).set_msg(Message::Disposable(msg))?;
    Ok(())
}

#[command]
pub async fn set_seal_msg(app: AppHandle, msg: String) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().await;
    (*guard).set_msg(Message::Seal(msg))?;
    Ok(())
}

#[command]
pub async fn set_plan_sync_msg(app: AppHandle, plan: Plans) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().await;
    (*guard).set_msg(Message::PlanSync(plan))?;
    Ok(())
}

#[command]
pub async fn set_mail_msg(app: AppHandle, mail: Mail) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().await;
    let mail_str = serde_json::to_string(&mail).unwrap();
    (*guard).set_msg(Message::Mail(mail_str))?;
    Ok(())
}

#[command]
pub async fn clear_msg(app: AppHandle) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().await;
    (*guard).clear_msg()?;
    Ok(())
}

#[command]
pub fn request_blep_bluetooth_permissions(app: AppHandle) -> Result<PermissionState, Error> {
    app.blep()
        .request_bluetooth_permission()
        .map_err(|e| Error::RequestBlueTooth(e.to_string()))
}

#[command]
pub fn store_disposable_drafts(app: AppHandle, data: DisposableDrafts) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set("disposable-drafts", serde_json::to_value(&data).unwrap());
    Ok(())
}

#[command]
pub fn load_disposable_drafts(app: AppHandle) -> Result<DisposableDrafts, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get("disposable-drafts");
    let value = if let Some(v) = value {
        serde_json::from_value(v).map_err(|e| Error::Load(e.to_string()))?
    } else {
        DisposableDrafts::default()
    };
    Ok(value)
}

#[command]
pub fn store_sealed_instances(app: AppHandle, data: SealedInstances) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set("sealed-instances", serde_json::to_value(&data).unwrap());
    Ok(())
}

#[command]
pub fn load_sealed_instances(app: AppHandle) -> Result<SealedInstances, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get("sealed-instances");
    let value = if let Some(v) = value {
        serde_json::from_value(v).map_err(|e| Error::Load(e.to_string()))?
    } else {
        SealedInstances::default()
    };
    Ok(value)
}

#[command]
pub fn load_plan_drafts(app: AppHandle) -> Result<PlanDrafts, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get("plan-drafts");
    let value = if let Some(v) = value {
        serde_json::from_value(v).map_err(|e| Error::Load(e.to_string()))?
    } else {
        PlanDrafts::default()
    };
    Ok(value)
}

#[command]
pub fn store_plan_drafts(app: AppHandle, data: PlanDrafts) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set("plan-drafts", serde_json::to_value(&data).unwrap());
    Ok(())
}

#[command]
pub fn store_finished_plan_list(app: AppHandle, data: FinishedPlanList) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set("finished-plan-list", serde_json::to_value(&data).unwrap());
    Ok(())
}

#[command]
pub fn load_finished_plan_list(app: AppHandle) -> Result<FinishedPlanList, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get("finished-plan-list");
    let value = if let Some(v) = value {
        serde_json::from_value(v).map_err(|e| Error::Load(e.to_string()))?
    } else {
        FinishedPlanList::default()
    };
    Ok(value)
}

#[command]
pub fn store_mail_inner(app: AppHandle, uuid: Uuid, data: MailInner) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set(
        format!("mail-{}", uuid),
        serde_json::to_value(&data).unwrap(),
    );
    Ok(())
}

#[command]
pub fn load_mail_inner(app: AppHandle, uuid: Uuid) -> Result<MailInner, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get(format!("mail-{}", uuid));
    let value = if let Some(v) = value {
        serde_json::from_value(v).map_err(|e| Error::Load(e.to_string()))?
    } else {
        MailInner::default()
    };
    Ok(value)
}

#[command]
pub fn store_mail_covers(app: AppHandle, data: MailCoverList) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set("mail-cover-list", serde_json::to_value(&data).unwrap());
    Ok(())
}

#[command]
pub fn load_mail_covers(app: AppHandle) -> Result<MailCoverList, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get("mail-cover-list");
    let value = if let Some(v) = value {
        serde_json::from_value(v).map_err(|e| Error::Load(e.to_string()))?
    } else {
        MailCoverList::default()
    };
    Ok(value)
}

#[command]
pub fn store_mail_drafts_covers(app: AppHandle, data: MailCoverList) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set(
        "mail-drafts-cover-list",
        serde_json::to_value(&data).unwrap(),
    );
    Ok(())
}

#[command]
pub fn load_mail_drafts_covers(app: AppHandle) -> Result<MailCoverList, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get("mail-drafts-cover-list");
    let value = if let Some(v) = value {
        serde_json::from_value(v).map_err(|e| Error::Load(e.to_string()))?
    } else {
        MailCoverList::default()
    };
    Ok(value)
}
