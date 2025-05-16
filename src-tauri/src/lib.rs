use std::sync::Mutex;
mod error;
use tauri::{async_runtime, command, plugin::PermissionState, AppHandle, Emitter, Manager};
use tauri_plugin_blep::{self, BlepExt};
mod ble;
use ble::DeviceBridge;
use error::Error;
use tauri_plugin_blep::mobile::Message;
use tauri_plugin_nfc2::{self, Nfc2Ext};
use tokio::sync::{mpsc::unbounded_channel, watch};
use uuid::Uuid;

/// 初始化读卡器，读到卡时进行连接和发送等待发送的事件。
///
/// # Note
/// - 被读的设备读到读卡的设备的 uuid 也是通过 reader 提供的 channel 来返回的。
/// - 注册向前端发送接收到的事件是在 connect 里，所以在这里也注册了。
fn start_reader(app: AppHandle, uuid: Uuid) -> Result<(), Error> {
    let (sd, mut rv) = watch::channel(Uuid::new_v4());
    let (err_sd, mut err_rv) = unbounded_channel();
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        let nfc = app_handle.nfc2();
        nfc.init_nfc_reader(sd, err_sd, uuid).unwrap_or_else(|e| {
            app_handle
                .emit("err", Error::InitNfc(e.to_string()))
                .unwrap();
        });
    });

    let app_handle = app.clone();
    async_runtime::spawn(async move {
        while rv.changed().await.is_ok() {
            let uuid = *rv.borrow();
            let state = app_handle.state::<Mutex<DeviceBridge>>();
            let mut guard = state.lock().unwrap();
            if (*guard).is_connected() {
                (*guard)
                    .connect(uuid, app_handle.blep(), app_handle.clone())
                    .unwrap_or_else(|e| {
                        app_handle.emit("err", e).unwrap();
                    });
            }
            (*guard).send().unwrap_or_else(|e| {
                app_handle.emit("err", e).unwrap();
            });
        }
    });

    let app_handle = app.clone();
    async_runtime::spawn(async move {
        while let Some(e) = err_rv.recv().await {
            app_handle
                .emit("err", format!("{e:?}"))
                .expect("failed to emit nfc readed uuid");
        }
    });
    Ok(())
}

#[command]
fn request_blep_bluetooth_permissions(app: AppHandle) -> Result<PermissionState, Error> {
    app.blep()
        .request_bluetooth_permission()
        .map_err(|e| Error::RequestBlueTooth(e.to_string()))
}

#[command]
fn set_disposable_msg(app: AppHandle, msg: String) -> Result<(), Error> {
    let state = app.state::<Mutex<DeviceBridge>>();
    let mut guard = state.lock().unwrap();
    (*guard).set_msg(Message::Disposable(msg))?;
    Ok(())
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
            request_blep_bluetooth_permissions,
            set_disposable_msg,
        ])
        .setup(|app| {
            let bridge = DeviceBridge::new();
            start_reader(app.handle().clone(), bridge.uuid).unwrap_or_else(|e| {
                app.emit("err", e).unwrap();
            });
            app.manage(Mutex::new(bridge));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
