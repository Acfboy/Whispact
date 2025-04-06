use std::sync::Mutex;

use tauri::{async_runtime, command, AppHandle, Emitter, Manager, Wry};
use tauri_plugin_blec;
use tauri_plugin_blep::{self, BlepExt};
mod ble;
use ble::{peripheral::BLEPeripheral, BLEComm};

#[command]
fn start_ble_peripheral(app: AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<BLEPeripheral<Wry>>>();
    let mut data = state.lock().unwrap();
    data.setup(app.blep()).map_err(|e| e.to_string())?;
    let mut rv = data.take_recv();
    let handle = app.clone();
    async_runtime::spawn(async move {
        while let Some(m) = rv.recv().await {
            handle.emit("ble-message-received", m).expect("can't send when ble message received");
        }
    });
    Ok(())
}

#[command]
fn ble_peripheral_send(app: AppHandle, msg: String) -> Result<(), String> {
    let state = app.state::<Mutex<BLEPeripheral<Wry>>>();
    let data = state.lock().map_err(|e| e.to_string())?;
    data.send(msg)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_blep::init())
        .invoke_handler(tauri::generate_handler![
            start_ble_peripheral,
            ble_peripheral_send
        ])
        .manage(Mutex::new(BLEPeripheral::<Wry>::new()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
