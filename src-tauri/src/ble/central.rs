use super::BLEComm;
use crate::models::Error;
use async_trait::async_trait;
use tauri_plugin_blec::{
    self, models::ScanFilter, models::WriteType, Handler, OnDisconnectHandler,
};
use tauri_plugin_blep::mobile::Message;
use tokio::sync::mpsc;
use uuid::Uuid;

/// ble 主端的通信
pub struct BLECentral {
    /// 从端 characteristic 的 uuid
    uuid: Uuid,

    /// tauri_plugin_blec 提供的 handler
    handler: &'static Handler,
}

impl BLECentral {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid,
            handler: tauri_plugin_blec::get_handler().unwrap(),
        }
    }
}

#[async_trait]
impl BLEComm for BLECentral {
    /// 扫描 3 秒，如果找到和包含指定 uuid 的设备就进行连接，返回成功，否则返回失败。
    async fn connect(&mut self) -> Result<mpsc::UnboundedReceiver<Message>, Error> {
        let (noti_sd, noti_rv) = mpsc::unbounded_channel();

        let (sd, mut rv) = mpsc::channel(100);
        log::info!("Ble central scanning...");
        self.handler
            .discover(Some(sd), 3000, ScanFilter::None)
            .await
            .map_err(|e| Error::BleCentralDiscover(e.to_string()))?;

        let handler = self.handler;
        while let Some(devices) = rv.recv().await {
            log::info!("Discovered service: {devices:?}");
            let target_device = devices.iter().find(|&x| {
                x.service_data
                    .iter()
                    .any(|(id, _)| id.as_bytes() == self.uuid.as_bytes())
            });
            if let Some(device) = target_device {
                handler
                    .connect(&device.address, OnDisconnectHandler::None)
                    .await
                    .map_err(|e| Error::BleCentralConnect(e.to_string()))?;
                log::info!("Ble central connected.");
                break;
            }
        }

        self.handler
            .subscribe(self.uuid, move |msg: Vec<u8>| {
                let msg = serde_json::from_slice::<Message>(&msg);
                noti_sd
                    .send(msg.expect("received not utf8 string"))
                    .expect("noti_sd send failed");
            })
            .await
            .map_err(|e| Error::BleCentralSubscribe(e.to_string()))?;
        if !handler.is_connected() {
            Err(Error::BleCentralDeviceNotFound)
        } else {
            Ok(noti_rv)
        }
    }

    /// 向从端发送消息
    async fn send(&self, message: Message) -> Result<(), Error> {
        log::info!("Ble central sending message: {message:?}");
        self.handler
            .send_data(
                self.uuid,
                message.to_string().as_bytes(),
                WriteType::WithoutResponse,
            )
            .await
            .map_err(|e| Error::BleCenteralSendDataFailed(e.to_string()))
    }

    fn is_connected(&self) -> bool {
        self.handler.is_connected()
    }
}
