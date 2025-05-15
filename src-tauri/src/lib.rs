use std::sync::Mutex;

use tauri::{async_runtime, command, plugin::PermissionState, AppHandle, Emitter, Manager, Wry};
use tauri_plugin_blep::{self, BlepExt};
mod ble;
use ble:: DeviceBridge;
use tauri_plugin_nfc2::{self, Nfc2Ext};
use tokio::sync::{mpsc::unbounded_channel, watch};
use uuid::Uuid;

/// 仅仅用来测试 nfc。会启动监听 nfc 的事件，发送一个 nfc-new-uuid 事件通知前端读取到的 uuid。
#[command]
fn start_reader(app: AppHandle) -> Result<(), String> {
    let (sd, mut rv) = watch::channel(Uuid::new_v4());
    let (err_sd, mut err_rv) = unbounded_channel();
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        let nfc = app_handle.nfc2();
        nfc.init_nfc_reader(sd, err_sd)
            .map_err(|e| e.to_string())
            .unwrap();
    });
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        while rv.changed().await.is_ok() {
            let uuid =  rv.borrow().clone();
            let state = app_handle.state::<Mutex<DeviceBridge>>();
            let mut guard = state.lock().unwrap();
            (*guard).connect(uuid, app_handle.blep()).unwrap();
            (*guard).send().unwrap();
        }
    });
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        while let Some(e) = err_rv.recv().await {
            app_handle
                .emit("nfc-error", format!("{e:?}"))
                .expect("failed to emit nfc readed uuid");
        }
    });
    Ok(())
}

/// 将当前的 uuid 设置给卡模拟
#[command]
fn set_hce_uuid(app: AppHandle) -> Result<String, String> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let guard = state.lock().unwrap();
    let uuid = (*guard).uuid;
    drop(guard);
    let nfc = app.nfc2();
    let uuid_str = String::from(
        uuid.as_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    );
    nfc.set_hce(uuid).map_err(|e| e.to_string())?;
    Ok(uuid_str)
}

#[command]
fn request_blep_bluetooth_permissions(app: AppHandle) -> Result<PermissionState, String> {
    app.blep()
        .request_bluetooth_permission()
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_blep::init())
        .plugin(tauri_plugin_nfc2::init())
        .invoke_handler(tauri::generate_handler![
            start_reader,
            set_hce_uuid,
            request_blep_bluetooth_permissions
        ])
        .manage(Mutex::new(DeviceBridge::new()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
