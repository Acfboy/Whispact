use serde::de::DeserializeOwned;
use serde_json;
use tauri::plugin::PermissionState;
use tauri::{
    ipc::{Channel, InvokeResponseBody},
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};
use tokio::sync::mpsc;
use tokio::sync::watch;
use uuid::Uuid;
use crate::error::Error;

pub use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Blep<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.blep", "BlePeripheralPlugin")?;
    Ok(Blep(handle))
}

pub struct Blep<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Blep<R> {
    /// 设置插件  
    /// 传入 message_sender 用于转发收到的信息，connect_notifier 用于转发连接的变化。
    pub fn setup(
        &self,
        message_sender: mpsc::UnboundedSender<Message>,
        connect_notifier: watch::Sender<ConnectionStatus>,
        uuid: Uuid,
    ) -> crate::Result<()> {
        // 创建传输消息的 IPC channel，解析收到的消息后用 message_sender 转发。
        let channel = Channel::new(move |event| {
            let payload = match event {
                InvokeResponseBody::Json(payload) => serde_json::from_str::<RecvData>(&payload)
                    .map_err(|e| Error::InvalidMessage(e.to_string()))
                    .map_err(Into::<tauri::Error>::into)?,
                _ => panic!("Wrong return value from plugin-blep"),
            };
            let payload = serde_json::from_str(&payload.msg)
                .map_err(|e| Error::InvalidMessage(e.to_string()))
                .map_err(Into::<tauri::Error>::into)?;
            let sender = message_sender.clone();
            sender
                .send(payload)
                .expect("send received ble peripheral message failed");
            Ok(())
        });

        // 传输连接变化信息
        let connect_notifier = Channel::new(move |event| {
            let payload = match event {
                InvokeResponseBody::Json(s) => serde_json::from_str(&s)
                    .expect("counld not deserizlize ble peripheral connect status response"),
                _ => ConnectionStatus::Disconnected,
            };
            let sender = connect_notifier.clone();
            sender
                .send(payload)
                .expect("send ble peripheral connection change failed");
            Ok(())
        });

        let uuid = String::from(
            uuid.as_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer()),
        );
        self.0
            .run_mobile_plugin(
                "setup",
                WatchRecvPayload {
                    channel,
                    connect_notifier,
                    uuid,
                },
            )
            .map_err(Into::into)
    }

    pub fn send(&self, message: String) -> crate::Result<SendResponse> {
        self.0
            .run_mobile_plugin("send", SendRequest { message })
            .map_err(Into::into)
    }

    /// 获得 ble 相关权限
    pub fn request_bluetooth_permission(&self) -> crate::Result<PermissionState> {
        self.0
            .run_mobile_plugin::<PermissionResponse>(
                "requestPermissions",
                RequestPermission { bluetooth: true },
            )
            .map(|r| r.bluetooth)
            .map_err(Into::into)
    }
}
