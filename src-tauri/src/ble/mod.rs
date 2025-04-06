mod central;
pub mod peripheral;
use serde::Serialize;
use tauri::async_runtime::Receiver;

/// BLE 通信的主从端都会实现的 trait
pub trait BLEComm {
    /// 发送一条信息
    fn send(&self, message: String) -> Result<(), String>;
    /// 取出一个接收器
    fn take_recv(&mut self) -> Receiver<RecvMessage>; 
}

#[derive(Serialize, Clone)]
pub enum RecvMessage {
    Msg(String),
    Error(String),
    Disconnected
}


impl From<tauri_plugin_blep::RecvEvent> for RecvMessage {
    fn from(e: tauri_plugin_blep::RecvEvent) -> Self {
        match e.status.as_str() {
            "connected" => Self::Msg(e.message),
            "disconnected" => Self::Disconnected,
            "error" => Self::Error(e.message),
            _ => unreachable!(),
        }
    }
}

