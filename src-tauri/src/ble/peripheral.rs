use super::{BLEComm, Message};
use std::sync::Arc;
use tauri::async_runtime::block_on;
use tauri::Runtime;
use tauri_plugin_blep::mobile::{Blep, ConnectionStatus, RecvMessage};
use tokio::sync::mpsc;
use tokio::sync::watch;
use uuid::Uuid;

/// 封装 BLE 中外设通信。
pub struct BLEPeripheral<R: Runtime> {
    /// 用于接收 BLE 收到的消息
    /// 在 BLE 外设接收到消息的回调中，会有一个 Sender 通过通道将消息发送给这个 Receiver。
    /// 广播开始的时候放入 Receiver，之前为 None。
    recv_msg_receiver: Option<mpsc::UnboundedReceiver<RecvMessage>>,

    /// 接收连接变化消息
    connect_watcher: Option<watch::Receiver<ConnectionStatus>>,

    /// 保存 ble 外设插件类的引用。
    blep: Option<Arc<Blep<R>>>,

    /// 是否已经启动广播
    is_advertize_start: bool,
}

impl<R: Runtime> BLEPeripheral<R> {
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
    pub fn setup(&mut self, blep: Arc<Blep<R>>, uuid: Uuid) -> tauri_plugin_blep::Result<()> {
        if self.is_advertize_start {
            return Ok(());
        }

        self.blep = Some(blep.clone());

        let (sd, rv) = mpsc::unbounded_channel();
        self.recv_msg_receiver = Some(rv);
        let (noti_sd, noti_rv) = watch::channel(ConnectionStatus::Disconnected);
        self.connect_watcher = Some(noti_rv);

        blep.setup(sd, noti_sd, uuid)?;

        self.is_advertize_start = true;
        Ok(())
    }
}

impl<R: Runtime> BLEComm for BLEPeripheral<R> {
    fn send(&self, msg: String) -> Result<(), String> {
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

    /// 取走消息接收端。如果没有初始化过或者已经取走，则 panic
    /// TODO: 更好的错误处理
    fn take_recv<'a>(&mut self) -> mpsc::UnboundedReceiver<impl Message + 'a> {
        self.recv_msg_receiver
            .take()
            .expect("peripheral recv_msg_receiver is None")
    }

    /// 阻塞直到连接成功。如果没有初始化，则 panic。
    fn connect(&mut self) -> Result<(), String> {
        if let Some(watcher) = &mut self.connect_watcher {
            if let ConnectionStatus::Disconnected = *watcher.borrow() {
                let mut watcher_1 = watcher.clone();
                block_on(async move {
                    watcher_1.changed().await.unwrap();
                });
            }
            Ok(())
        } else {
            panic!("connect before setup");
        }
    }
}
