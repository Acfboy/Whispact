pub mod central;
pub mod peripheral;
use std::sync::Arc;

use crate::models::Error;
use central::BLECentral;
use peripheral::BLEPeripheral;
use std::cmp::Ordering::*;
use tauri::{async_runtime, AppHandle, Emitter, Wry};
use tauri_plugin_blep::mobile::{Blep, Message};
use tokio::sync::mpsc;
use uuid::Uuid;

/// BLE 通信的主从端都会实现的 trait
pub trait BLEComm {
    fn send(&self, message: Message) -> Result<(), Error>;

    /// 阻塞直到连接完成，返回接收器。
    /// 用于在触碰后等待连接。
    fn connect(&mut self) -> Result<mpsc::UnboundedReceiver<Message>, Error>;

    fn is_connected(&self) -> bool;
}

pub struct DeviceBridge {
    communicater: Option<Box<dyn BLEComm + Send + Sync>>,
    pub uuid: Uuid,
    message_rx: Option<mpsc::UnboundedReceiver<Message>>,
    next_msg: Option<Message>,
}

impl DeviceBridge {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        log::info!("uuid generated: {uuid}");
        Self {
            communicater: None,
            uuid: Uuid::new_v4(),
            message_rx: None,
            next_msg: None,
        }
    }

    /// 连接上另一条设备，根据 uuid 决定自己应该是主端还是从端，然后设置事件监听转发到前端
    /// - 这里规定大的作为主端，小的作为从端。
    pub fn connect(
        &mut self,
        uuid: Uuid,
        blep: Arc<Blep<Wry>>,
        handle: AppHandle,
    ) -> Result<(), Error> {
        let mut commu: Box<dyn BLEComm + Send + Sync> =
            match self.uuid.as_u128().cmp(&uuid.as_u128()) {
                Greater => {
                    log::info!("Act as BLECentral");
                    let commu = BLECentral::new(uuid);
                    Box::new(commu)
                }
                Less => {
                    log::info!("Act as BLEPeripheral");
                    let mut commu = BLEPeripheral::new();
                    commu.setup(blep, self.uuid);
                    Box::new(commu)
                }
                Equal => {
                    return Err(Error::Lucky(
                        "How lucky you are! There is only 1e-36 possibility to get the same uuid!"
                            .to_string(),
                    ));
                }
            };

        self.message_rx = Some(commu.connect()?);
        self.communicater = Some(commu);
        self.set_emmiter(handle)?;
        Ok(())
    }

    /// 设置事件发生器，向前端发送收到信号事件
    pub fn set_emmiter(&mut self, handle: AppHandle) -> Result<(), Error> {
        let mut rx = if self.message_rx.is_none() {
            return Err(Error::ReceiveBeforeConnect);
        } else {
            self.message_rx.take().unwrap()
        };
        async_runtime::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    Message::Disposable(s) => handle.emit("recv-disposable-msg", s),
                    Message::BackToBack(s) => handle.emit("recv-back-to-back-msg", s),
                    Message::Seal(s) => handle.emit("recv-seal-msg", s),
                    Message::PlanSync(p) => handle.emit("recv-plan-sync", p),
                }
                .expect("failed to send msg to frontend");
            }
        });
        log::info!("Message event emmiter set.");
        Ok(())
    }

    pub fn set_msg(&mut self, msg: Message) -> Result<(), Error> {
        if self.next_msg.is_some() {
            return Err(Error::LastMessageNotSend);
        }
        log::info!("Next message set: {msg:?}");
        self.next_msg = Some(msg);
        Ok(())
    }

    pub fn send(&mut self) -> Result<(), Error> {
        if self.next_msg.is_none() {
            return Ok(());
        }
        if self.communicater.is_none() {
            return Err(Error::SendBeforeConnect);
        }
        self.communicater
            .as_mut()
            .unwrap()
            .send(self.next_msg.take().unwrap())?;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        match &self.communicater {
            None => false,
            Some(c) => c.is_connected(),
        }
    }
}
