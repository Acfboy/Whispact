pub mod central;
pub mod peripheral;
use std::sync::Arc;

use crate::models::Error;
use async_trait::async_trait;
use central::BLECentral;
use peripheral::BLEPeripheral;
use std::cmp::Ordering::*;
use tauri::{async_runtime, AppHandle, Emitter, Wry};
use tauri_plugin_blep::mobile::{Blep, Message};
use tokio::sync::{mpsc, Semaphore};
use uuid::Uuid;

/// BLE 通信的主从端都会实现的 trait
#[async_trait]
pub trait BLEComm {
    async fn send(&self, message: Message) -> Result<(), Error>;

    /// 直到连接完成返回接收器。
    /// 用于在触碰后等待连接。
    async fn connect(&mut self) -> Result<mpsc::UnboundedReceiver<Message>, Error>;

    fn is_connected(&self) -> bool;
}

pub struct DeviceBridge {
    communicater: Option<Box<dyn BLEComm + Send + Sync>>,
    pub uuid: Uuid,
    message_rx: Option<mpsc::UnboundedReceiver<Message>>,
    next_msg: Option<Message>,
    /// 之后可能会支持与不同的人通信，故需要记录上一次连接的 uuid，如果新读到的不一致，需要断开并重新连接。
    last_uuid: Option<Uuid>,
    /// 作为从端通信的时候需要信号量用于确定主端的监听已经设置。
    ///
    /// 具体地，收到主端的信号后接收增加 permit，表示已经可以发送信息。从端在发送信息的时候等待获得一个 permit。
    notify_semaphore: Option<Arc<Semaphore>>,
}

impl DeviceBridge {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        log::info!("uuid generated: {uuid}");
        Self {
            communicater: None,
            uuid,
            message_rx: None,
            next_msg: None,
            notify_semaphore: None,
            last_uuid: None,
        }
    }

    /// 连接上另一条设备，根据 uuid 决定自己应该是主端还是从端，然后设置事件监听转发到前端
    /// - 这里规定大的作为主端，小的作为从端。
    pub async fn connect(
        &mut self,
        uuid: Uuid,
        blep: Arc<Blep<Wry>>,
        handle: AppHandle,
    ) -> Result<(), Error> {
        if let Some(last) = &self.last_uuid {
            if uuid.as_bytes() != last.as_bytes() {
                return Err(Error::Unsupport("暂不支持多次不同触碰".to_string()));
            }
        } else {
            self.last_uuid = Some(uuid);
        }

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
                    self.notify_semaphore = Some(Arc::new(Semaphore::new(0)));
                    Box::new(commu)
                }
                Equal => {
                    return Err(Error::Lucky(
                        "How lucky you are! There is only 1e-36 possibility to get the same uuid!"
                            .to_string(),
                    ));
                }
            };

        self.message_rx = Some(commu.connect().await?);
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

        let semaphore = self.notify_semaphore.clone();

        async_runtime::spawn(async move {
            while let Some(msg) = rx.recv().await {
                log::info!("Received: {:?}", msg);

                if let Some(s) = semaphore.clone() {
                    s.add_permits(1);
                }

                match msg {
                    Message::Disposable(s) => handle.emit("recv-disposable-msg", s),
                    Message::BackToBack(s) => handle.emit("recv-back-to-back-msg", s),
                    Message::Seal(s) => handle.emit("recv-seal-msg", s),
                    Message::PlanSync(p) => handle.emit("recv-plan-sync", p),
                    Message::Empty => Ok(()),
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

    pub async fn send(&mut self) -> Result<(), Error> {
        if self.communicater.is_none() {
            return Err(Error::SendBeforeConnect);
        }

        if let Some(s) = self.notify_semaphore.clone() {
            // 如果是从端，需要等待许可，即收到对方消息，即监听已经设置，再发送消息。
            let res = s.acquire().await;
            if let Err(e) = res {
                return Err(Error::BlePeripheralSendFail(e.to_string()));
            }
        }

        if self.next_msg.is_none() {
            self.next_msg = Some(Message::Empty);
        }
        self.communicater
            .as_mut()
            .unwrap()
            .send(self.next_msg.take().unwrap())
            .await?;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        match &self.communicater {
            None => false,
            Some(c) => c.is_connected(),
        }
    }
}
