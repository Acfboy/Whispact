use std::sync::Mutex;

use tauri::{async_runtime, command, AppHandle, Emitter, Manager, Wry};
use tauri_plugin_blep::{self, BlepExt};
mod ble;
use ble::{central::BLECentral, peripheral::BLEPeripheral, BLEComm, Message};
use tauri_plugin_nfc2::{self, Nfc2Ext};
use tokio::sync::{mpsc::unbounded_channel, watch};
use uuid::{uuid, Uuid};

/// 开启 ble 从端广播，并监听收到的消息，收到消息时发送 ble-message-received 事件
#[command]
fn start_ble_peripheral(app: AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<BLEPeripheral<Wry>>>();
    let mut data = state.lock().unwrap();

    // for test
    let uuid = uuid!("0000ffe1-0000-1000-8000-00805f9b34fb");

    data.setup(app.blep(), uuid).map_err(|e| e.to_string())?;
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

/// 仅仅用来测试 nfc。会启动监听 nfc 的事件，发送一个 nfc-new-uuid 事件通知前端读取到的 uuid。
#[command]
fn start_reader(app: AppHandle) -> Result<(), String> {
    let (sd, mut rv) = watch::channel(Uuid::new_v4());
    let (err_sd, mut err_rv) = unbounded_channel();
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        let nfc = app_handle.nfc2();
        nfc.init_nfc_reader(sd, err_sd).map_err(|e| e.to_string()).unwrap();
    });
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        while rv.changed().await.is_ok() {
            let uuid = String::from(
                rv.borrow()
                    .as_hyphenated()
                    .encode_lower(&mut Uuid::encode_buffer()),
            );
            app_handle
                .emit("nfc-new-uuid", uuid)
                .expect("failed to emit nfc readed uuid");
        }
    });
    let app_handle = app.clone();
    async_runtime::spawn(async move {
        while let Some(e) = err_rv.recv().await {
            app_handle.emit("nfc-error", format!("{e:?}"))
                .expect("failed to emit nfc readed uuid");
        }
    });
    Ok(())
}

/// 仅用于测试，随机生成一个 uuid 并设置给卡模拟，然后返回给前端。
#[command]
fn set_hce_uuid(app: AppHandle) -> Result<String, String> {
    let nfc = app.nfc2();
    let uuid = Uuid::new_v4();
    let uuid_str = String::from(
        uuid.as_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    );
    nfc.set_hce(uuid).map_err(|e| e.to_string())?;
    Ok(uuid_str)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_blep::init())
        .plugin(tauri_plugin_nfc2::init())
        .invoke_handler(tauri::generate_handler![
            start_ble_peripheral,
            ble_peripheral_send,
            ble_peripheral_wait_connect,
            start_ble_central_with_uuid,
            ble_central_send,
            start_reader,
            set_hce_uuid,
        ])
        .manage(Mutex::new(BLEPeripheral::<Wry>::new()))
        .manage(Mutex::new(BLECentral::new()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
