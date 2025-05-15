use super::BLEComm;
use tauri_plugin_blep::mobile::Message;
use tauri::async_runtime;
use tauri_plugin_blec::{
    self, models::ScanFilter, models::WriteType, Handler, OnDisconnectHandler,
};
use tokio::sync::mpsc;
use uuid::Uuid;
use log;

/// ble 主端的通信
pub struct BLECentral {
    /// 从端 characteristic 的 uuid
    uuid: Uuid,

    /// tauri_plugin_blec 提供的 handler
    handler: &'static Handler,

    /// 转发从端 notify，即发来消息，的 receiver
    receiver: Option<mpsc::UnboundedReceiver<String>>,
}

impl BLECentral {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: uuid, 
            handler: tauri_plugin_blec::get_handler().unwrap(),
            receiver: None,
        }
    }
}

impl BLEComm for BLECentral {
    /// 扫描 3 秒，如果找到和包含指定 uuid 的设备就进行连接，返回成功，否则返回失败。
    fn connect(&mut self) -> Result<mpsc::UnboundedReceiver<Message>, String> {
        let (noti_sd, noti_rv) = mpsc::unbounded_channel();
        async_runtime::block_on(async move {
            let uuid = self.uuid.clone();

            let (sd, mut rv) = mpsc::channel(100);
            self.handler
                .discover(Some(sd), 3000, ScanFilter::None)
                .await
                .map_err(|e| "discover: ".to_owned() + &e.to_string())?;

            let handler = self.handler;
            while let Some(devices) = rv.recv().await {
                let target_device = devices
                    .iter()
                    .find(|&x| x.service_data.iter().any(|(id, _)| id.as_bytes() == uuid.as_bytes()));
                log::info!("decices: {devices:?}");
                if let Some(device) = target_device {
                    // TODO: 断连提示
                    handler
                        .connect(&device.address, OnDisconnectHandler::None)
                        .await
                        .map_err(|e| "connect".to_owned() + &e.to_string())?;
                    break;
                }
            }

            self.handler
                .subscribe(uuid, move |msg: Vec<u8>| {
                    let msg = serde_json::from_slice::<Message>(&msg);
                    noti_sd
                        .send(msg.expect("received not utf8 string"))
                        .expect("noti_sd send failed");
                })
                .await
                .map_err(|e| "subscribe: ".to_owned() + &e.to_string())?;
            if !handler.is_connected() {
                Err("target peripheral device not found".to_string())
            } else {
                Ok(noti_rv)
            }
        })
    }

    /// 向从端发送消息
    fn send(&self, message: Message) -> Result<(), String> {
        async_runtime::block_on(async move {
            let uuid = self.uuid.clone();
            self.handler
                .send_data(uuid, message.to_string().as_bytes(), WriteType::WithoutResponse)
                .await
                .map_err(|e| e.to_string())
        })
    }

}
