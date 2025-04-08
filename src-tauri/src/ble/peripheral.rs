use std::sync::Arc;
use super::{BLEComm, Message};
use tauri::{
    async_runtime::{block_on, channel, Receiver},
    Runtime,
};
use tauri_plugin_blep::{mobile::Blep, ConnectionStatus, RecvMessage};

/// 封装 BLE 中外设通信。
pub struct BLEPeripheral<R: Runtime> {
    /// 用于接收 BLE 收到的消息
    /// 在 BLE 外设接收到消息的回调中，会有一个 Sender 通过通道将消息发送给这个 Receiver。
    /// 广播开始的时候放入 Receiver，之前为 None。
    recv_msg_receiver: Option<Receiver<RecvMessage>>,

    /// 接收连接变化消息
    connect_notifier: Option<Receiver<ConnectionStatus>>,

    /// 保存 ble 外设插件类的引用。
    blep: Option<Arc<Blep<R>>>,

    /// 是否已经启动广播
    is_advertize_start: bool,
}

const CHANNEL_SIZE: usize = 100;

impl<R: Runtime> BLEPeripheral<R> {
    pub fn new() -> Self {
        Self {
            recv_msg_receiver: None,
            connect_notifier: None,
            blep: None,
            is_advertize_start: false,
        }
    }

    /// 启动广播。
    /// 如果已经启动，不做任何事。
    pub fn setup(&mut self, blep: Arc<Blep<R>>) -> tauri_plugin_blep::Result<()> {
        if self.is_advertize_start {
            return Ok(());
        }

        self.blep = Some(blep.clone());

        let (sd, rv) = channel(CHANNEL_SIZE);
        self.recv_msg_receiver = Some(rv);

        let (noti_sd, noti_rv) = channel(CHANNEL_SIZE);
        self.connect_notifier = Some(noti_rv);

        blep.setup(sd, noti_sd)?;

        self.is_advertize_start = true;
        Ok(())
    }
}

impl<R: Runtime> BLEComm for BLEPeripheral<R> {
    fn send(&self, msg: String) -> Result<(), String> {
        match self.blep.clone() {
            Some(blep) => {
                match block_on(async move { blep.send(msg) }) {
                    Err(s) => Err(s.to_string()),
                    Ok(ok) => if ok.success { Ok(()) } else { Err("failed".to_string()) }
                }
            }
            None => Ok(()),
        }
    }

    /// 取走消息接收端。如果没有初始化过或者已经取走，则 panic
    fn take_recv<'a>(&mut self) -> Receiver<impl Message + 'a> {
        self.recv_msg_receiver.take().expect("peripheral recv_msg_receiver is None")
    }

    fn connect(&mut self) {
        unimplemented!()
    }
}
