use super::BLEComm;
use tokio::sync::mpsc;
use std::sync::Arc;
use tauri::{async_runtime, Wry};
use tauri::async_runtime::block_on;
use tauri_plugin_blep::mobile::{Blep, ConnectionStatus, Message};
use tokio::sync::watch;
use uuid::Uuid;

/// 封装 BLE 中外设通信。
pub struct BLEPeripheral {
    /// 用于接收 BLE 收到的消息
    /// 在 BLE 外设接收到消息的回调中，会有一个 Sender 通过通道将消息发送给这个 Receiver。
    /// 广播开始的时候放入 Receiver，之前为 None。
    recv_msg_receiver: Option<mpsc::UnboundedReceiver<Message>>,

    /// 接收连接变化消息
    connect_watcher: Option<watch::Receiver<ConnectionStatus>>,

    /// 保存 ble 外设插件类的引用。
    blep: Option<Arc<Blep<Wry>>>,

    /// 是否已经启动广播
    is_advertize_start: bool,
}

impl BLEPeripheral {
    pub fn new() -> Self {
        Self {
            recv_msg_receiver: None,
            connect_watcher: None,
            blep: None,
            is_advertize_start: false,
        }
    }

    /// 启动广播。
    /// 如果已经启动，不做任何事。
    pub fn setup(&mut self, blep: Arc<Blep<Wry>>, uuid: Uuid) {
        self.blep = Some(blep.clone());

        let (sd, rv) = mpsc::unbounded_channel();
        self.recv_msg_receiver = Some(rv);
        let (noti_sd, noti_rv) = watch::channel(ConnectionStatus::Disconnected);
        self.connect_watcher = Some(noti_rv);

        async_runtime::spawn(async move {
            blep.setup(sd, noti_sd, uuid).expect("failed to setup blep");
        });
        self.is_advertize_start = true;
    }
}

impl BLEComm for BLEPeripheral {
    fn send(&self, msg: Message) -> Result<(), String> {
        let msg = msg.to_string();
        match self.blep.clone() {
            Some(blep) => match block_on(async move { blep.send(msg) }) {
                Err(s) => Err(s.to_string()),
                Ok(ok) => {
                    if ok.success {
                        Ok(())
                    } else {
                        Err("failed".to_string())
                    }
                }
            },
            None => Ok(()),
        }
    }

    /// 阻塞直到连接成功。如果没有初始化，则 panic
    fn connect(&mut self) -> Result<mpsc::UnboundedReceiver<Message>, String> {
        if let Some(watcher) = &mut self.connect_watcher {
            if let ConnectionStatus::Disconnected = *watcher.borrow() {
                let mut watcher_1 = watcher.clone();
                block_on(async move {
                    watcher_1.changed().await.unwrap();
                });
            }
            Ok(self.recv_msg_receiver.take().unwrap())
        } else {
            panic!("connect before setup");
        }
    }
}
