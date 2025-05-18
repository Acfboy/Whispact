use tokio::sync::Mutex;
use tauri::{command, plugin::PermissionState, AppHandle, Manager};
use tauri_plugin_blep::{
    mobile::{Message, Plans},
    BlepExt,
};
use tauri_plugin_store::StoreExt;

use crate::{
    ble::DeviceBridge,
    models::{BackToBackDrafs, DisposableDrafts, Error, FinishedPlanList, SealedInstances},
};

#[command]
pub async fn set_disposable_msg(app: AppHandle, msg: String) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().await;
    (*guard).set_msg(Message::Disposable(msg))?;
    Ok(())
}

#[command]
pub async fn set_back_to_back_msg(app: AppHandle, msg: String) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().await;
    (*guard).set_msg(Message::BackToBack(msg))?;
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
    let value = store.get("disposable-drafts").unwrap_or_default();
    let value = serde_json::from_value(value).map_err(|e| Error::Load(e.to_string()))?;
    Ok(value)
}

#[command]
pub fn store_back_to_back_drafts(app: AppHandle, data: BackToBackDrafs) -> Result<(), Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    store.set("back-to-back-drafts", serde_json::to_value(&data).unwrap());
    Ok(())
}

#[command]
pub fn load_back_to_back_drafts(app: AppHandle) -> Result<BackToBackDrafs, Error> {
    let store = app.store("store.json").map_err(Into::<Error>::into)?;
    let value = store.get("back-to-back-drafts").unwrap_or_default();
    let value = serde_json::from_value(value).map_err(|e| Error::Load(e.to_string()))?;
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
    let value = store.get("sealed-instances").unwrap_or_default();
    let value = serde_json::from_value(value).map_err(|e| Error::Load(e.to_string()))?;
    Ok(value)
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
    let value = store.get("finished-plan-list").unwrap_or_default();
    let value = serde_json::from_value(value).map_err(|e| Error::Load(e.to_string()))?;
    Ok(value)
}
