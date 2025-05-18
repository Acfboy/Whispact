use super::BLEComm;
use crate::models::Error;
use async_trait::async_trait;
use std::sync::Arc;
use tauri::{async_runtime, Wry};
use tauri_plugin_blep::mobile::{Blep, ConnectionStatus, Message};
use tokio::sync::mpsc;
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
        log::info!("Ble peripheral setup");
        self.is_advertize_start = true;
    }
}

#[async_trait]
impl BLEComm for BLEPeripheral {
    async fn send(&self, msg: Message) -> Result<(), Error> {
        let msg = msg.to_string();
        log::info!("Sending message: {msg:?}");
        match self.blep.clone() {
            Some(blep) => match blep.send(msg) {
                Err(s) => Err(Error::BlePeripheralSendFail(s.to_string())),
                Ok(ok) => {
                    if ok.success {
                        Ok(())
                    } else {
                        Err(Error::BlePeripheralSendFail("failed".to_string()))
                    }
                }
            },
            None => Ok(()),
        }
    }

    /// 阻塞直到连接成功。
    async fn connect(&mut self) -> Result<mpsc::UnboundedReceiver<Message>, Error> {
        log::info!("Wating for connect");
        if let Some(watcher) = &mut self.connect_watcher {
            let status = watcher.borrow().clone();
            if let ConnectionStatus::Disconnected = status {
                watcher.changed().await.unwrap();
                log::info!("Ble peipheral connected.");
            }
            if self.recv_msg_receiver.is_none() {
                Err(Error::ConnectBeforeSetup)
            } else {
                Ok(self.recv_msg_receiver.take().unwrap())
            }
        } else {
            Err(Error::ConnectBeforeSetup)
        }
    }

    fn is_connected(&self) -> bool {
        if self.connect_watcher.is_none() {
            false
        } else {
            matches!(
                *(self.connect_watcher.as_ref().unwrap().borrow()),
                ConnectionStatus::Connected
            )
        }
    }
}
