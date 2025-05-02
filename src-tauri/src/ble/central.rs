use super::BLEComm;
use super::Message;
use tauri::async_runtime;
use tauri_plugin_blec::{
    self, models::ScanFilter, models::WriteType, Handler, OnDisconnectHandler,
};
use tokio::sync::mpsc;
use uuid::Builder;

/// ble 主端的通信
pub struct BLECentral {
    /// 从端 characteristic 的 uuid
    uuid: String,

    /// tauri_plugin_blec 提供的 handler
    handler: &'static Handler,

    /// 转发从端 notify，即发来消息，的 receiver
    receiver: Option<mpsc::UnboundedReceiver<String>>,
}

impl BLECentral {
    pub fn new() -> Self {
        Self {
            uuid: String::new(),
            handler: tauri_plugin_blec::get_handler().unwrap(),
            receiver: None,
        }
    }

    /// 更新 uuid
    pub fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }
}

impl BLEComm for BLECentral {
    /// 扫描 3 秒，如果找到和包含指定 uuid 的设备就进行连接，返回成功，否则返回失败。
    fn connect(&mut self) -> Result<(), String> {
        if self.uuid.is_empty() {
            return Err("haven't set uuid".to_string());
        }
        let (noti_sd, noti_rv) = mpsc::unbounded_channel();
        self.receiver = Some(noti_rv);
        async_runtime::block_on(async move {
            let uuid = Builder::from_bytes(
                self.uuid
                    .as_bytes()
                    .try_into()
                    .map_err(|e: std::array::TryFromSliceError| e.to_string())?,
            )
            .into_uuid();
            self.handler
                .subscribe(uuid, move |msg| {
                    noti_sd
                        .send(String::from_utf8(msg).expect("received not utf8 string"))
                        .expect("noti_sd send failed");
                })
                .await
                .map_err(|e| e.to_string())?;

            let (sd, mut rv) = mpsc::channel(100);
            self.handler
                .discover(Some(sd), 3000, ScanFilter::None)
                .await
                .map_err(|e| e.to_string())?;

            let handler = self.handler;
            let target_uuid = self.uuid.clone();
            while let Some(devices) = rv.recv().await {
                let target_device = devices
                    .iter()
                    .find(|&x| x.services.iter().any(|id| id.to_string() == target_uuid));
                if let Some(device) = target_device {
                    // TODO: 断连提示
                    handler
                        .connect(&device.address, OnDisconnectHandler::None)
                        .await
                        .map_err(|e| e.to_string())?;
                    return Ok(());
                }
            }
            if !handler.is_connected() {
                Err("target peripheral device not found".to_string())
            } else {
                Ok(())
            }
        })
    }

    /// 向 uuid 发送消息
    fn send(&self, message: String) -> Result<(), String> {
        let uuid = self.uuid.clone();
        async_runtime::block_on(async move {
            let uuid = Builder::from_bytes(
                uuid.as_bytes()
                    .try_into()
                    .map_err(|e| format!("invalid uuid {}", e))?,
            )
            .into_uuid();
            self.handler
                .send_data(uuid, message.as_bytes(), WriteType::WithoutResponse)
                .await
                .map_err(|e| e.to_string())
        })
    }

    /// 取走接收器
    fn take_recv<'a>(&mut self) -> mpsc::UnboundedReceiver<impl super::Message + 'a> {
        self.receiver.take().unwrap()
    }
}

/// 主端直接接收 String
impl Message for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}
