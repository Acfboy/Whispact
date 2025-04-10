use std::sync::Mutex;

use tauri::{async_runtime, command, AppHandle, Emitter, Manager, Wry};
use tauri_plugin_blep::{self, BlepExt};
mod ble;
use ble::{central::BLECentral, peripheral::BLEPeripheral, BLEComm, Message};

/// 开启 ble 从端广播，并监听收到的消息，收到消息时发送 ble-message-received 事件
#[command]
fn start_ble_peripheral(app: AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<BLEPeripheral<Wry>>>();
    let mut data = state.lock().unwrap();
    data.setup(app.blep()).map_err(|e| e.to_string())?;
    let rv = data.take_recv();
    let handle = app.clone();
    async_runtime::spawn(async move {
        let mut rv = rv;
        while let Some(m) = rv.recv().await {
            handle
                .emit("ble-message-received", m.as_str())
                .expect("can't send when ble message received");
        }
    });
    Ok(())
}

/// 从端发送消息
#[command]
fn ble_peripheral_send(app: AppHandle, msg: String) -> Result<(), String> {
    let state = app.state::<Mutex<BLEPeripheral<Wry>>>();
    let data = state.lock().map_err(|e| e.to_string())?;
    data.send(msg)?;
    Ok(())
}

/// 从端阻塞等待连接。实际上由于前端调用是异步的，并不会阻塞
#[command]
fn ble_peripheral_wait_connect(app: AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<BLEPeripheral<Wry>>>();
    let mut data = state.lock().unwrap();
    data.connect()?;
    Ok(())
}

/// 传入收到的 uuid 开启三秒的主端扫描，尝试连接。
///
/// - 无法连接返回错误。
/// - 若连接成功，设置收到消息监听，收到消息时发送 ble-message-received 事件。
///
/// 同样由于前端调用是异步的，不会真的阻塞。
#[command]
fn start_ble_central_with_uuid(app: AppHandle, uuid: String) -> Result<(), String> {
    let state = app.state::<Mutex<BLECentral>>();
    let mut data = state.lock().unwrap();
    data.set_uuid(uuid);
    data.connect()?;
    let mut rv = data.take_recv();
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        while let Some(s) = rv.recv().await {
            app_handle
                .emit("ble-message-received", s.as_str())
                .expect("can't send when ble message received");
        }
    });
    Ok(())
}

/// 主端发送消息
#[command]
fn ble_central_send(app: AppHandle, msg: String) -> Result<(), String> {
    let state = app.state::<Mutex<BLECentral>>();
    let data = state.lock().unwrap();
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
            ble_peripheral_send,
            ble_peripheral_wait_connect,
            start_ble_central_with_uuid,
            ble_central_send
        ])
        .manage(Mutex::new(BLEPeripheral::<Wry>::new()))
        .manage(Mutex::new(BLECentral::new()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
