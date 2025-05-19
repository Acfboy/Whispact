use tauri_plugin_fs::FsExt;
use tokio::sync::Mutex;
mod models;
use tauri::{async_runtime, AppHandle, Emitter, Manager};
use tauri_plugin_blep::{self, BlepExt};
mod ble;
use ble::DeviceBridge;
use models::Error;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_nfc2::{self, Nfc2Ext};
use tokio::sync::{mpsc::unbounded_channel, watch};
use utils::*;
use uuid::Uuid;
mod utils;

/// 初始化读卡器，读到卡时进行连接和发送等待发送的事件。
///
/// # Note
/// - 被读的设备读到读卡的设备的 uuid 也是通过 reader 提供的 channel 来返回的。
/// - 注册向前端发送接收到的事件是在 connect 里，所以在这里也注册了。
fn start_reader(app: AppHandle, uuid: Uuid) -> Result<(), Error> {
    let placeholder = Uuid::new_v4();
    log::info!("{}", placeholder.simple());
    let (sd, mut rv) = watch::channel(placeholder);
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

            log::info!("Read uuid: {}", uuid.to_string());

            let state = app_handle.state::<Mutex<DeviceBridge>>();
            let mut guard = state.lock().await;
            if !(*guard).is_connected() {
                log::info!("Status: Disconnected; Try to connect...");

                (*guard)
                    .connect(uuid, app_handle.blep(), app_handle.clone())
                    .await
                    .unwrap_or_else(|e| {
                        app_handle.emit("err", e).unwrap();
                    });
            }
            (*guard).send().await.unwrap_or_else(|e| {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::LogDir {
                        file_name: Some("log".to_string()),
                    }),
                    Target::new(TargetKind::Stdout),
                ])
                .level(log::LevelFilter::Info)
                .max_file_size(10000)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_blep::init())
        .plugin(tauri_plugin_nfc2::init())
        .invoke_handler(tauri::generate_handler![
            request_blep_bluetooth_permissions,
            store_back_to_back_drafts,
            load_back_to_back_drafts,
            store_sealed_instances,
            load_sealed_instances,
            store_finished_plan_list,
            load_finished_plan_list,
            store_disposable_drafts,
            load_disposable_drafts,
            set_disposable_msg,
            set_back_to_back_msg,
            set_seal_msg,
            set_plan_sync_msg,
        ])
        .setup(|app| {
            let scope = app.fs_scope();
            let data_dir = app.path().data_dir().unwrap();
            scope.allow_directory(data_dir, true).unwrap();

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
