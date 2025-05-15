pub mod central;
pub mod peripheral;
use std::sync::Arc;

use central::BLECentral;
use peripheral::BLEPeripheral;
use tauri::{async_runtime, AppHandle, Emitter, Wry};
use tauri_plugin_blep::mobile::{Blep, Message};
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::error::Error;

// type IMessage = impl Message;
/// BLE 通信的主从端都会实现的 trait
pub trait BLEComm {
    /// 发送一条信息
    fn send(&self, message: Message) -> Result<(), Error>;

    /// 阻塞直到连接完成，返回接收器。
    /// 用于在触碰后等待连接。
    fn connect(&mut self) -> Result<mpsc::UnboundedReceiver<Message>, Error>;
}

pub struct DeviceBridge {
    communicater: Option<Box<dyn BLEComm + Send + Sync>>,
    pub uuid: Uuid,
    message_rx: Option<mpsc::UnboundedReceiver<Message>>,
    next_msg: Option<Message>,
}

impl DeviceBridge {
    pub fn new() -> Self {
        Self {
            communicater: None,
            uuid: Uuid::new_v4(),
            message_rx: None,
            next_msg: None,
        }
    }

    /// 连接上另一条设备，根据 uuid 决定自己应该是主端还是从端
    /// - 这里规定大的作为主端，小的作为从端。
    pub fn connect(&mut self, uuid: Uuid, blep: Arc<Blep<Wry>>) -> Result<(), Error> {
        self.communicater = if self.uuid.as_u128() > uuid.as_u128() {
            let commu = BLECentral::new(uuid);
            Some(Box::new(commu))
        } else if self.uuid.as_u128() < uuid.as_u128() {
            let mut commu = BLEPeripheral::new();
            commu.setup(blep, self.uuid);
            Some(Box::new(commu))
        } else {
            panic!("How lucky you are! There is only 1e-36 possibility to get the same uuid!");
        };

        let rx = Some(self.communicater.as_mut().unwrap().connect()?);
        self.message_rx = rx;
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
                    Message::PlanCheck(u) => handle.emit("recv-plan-check", u),
                    Message::PlanSync(p) => handle.emit("recv-plan-sync", p),
                    Message::DiffPlan => handle.emit("err-diff-plan", ()),
                    Message::DiffSeal => handle.emit("err-diff-seal", ()),
                }.expect("failed to send msg to frontend");
            }
        });
        Ok(())
    }

    pub fn set_msg(&mut self, msg: Message) -> Result<(), Error>{
        if self.next_msg.is_some() {
            return Err(Error::LastMessageNotSend);
        }
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
        self.communicater.as_mut().unwrap().send(self.next_msg.take().unwrap())?;
        Ok(())
    }
}
