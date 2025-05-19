use crate::error::Error;
use serde::de::DeserializeOwned;
use tauri::{
    ipc::{Channel, InvokeResponseBody},
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};
use tokio::sync::{mpsc, watch};
use uuid::Uuid;

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Nfc2<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.nfc2", "Nfc2Plugin")?;
    Ok(Nfc2(handle))
}

pub struct Nfc2<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Nfc2<R> {
    /// 初始化 nfc 读卡器，并设置 uuid。
    /// - `uuid_sender`: 一个 watch 的 sender，内部值是最后一次 nfc 读到的对方 uuid。
    /// - `error_sender`: 用于发送错误信息，错误信息可能有 NFC_NOT_SUPPORTED, NFC_DISABLED, SECURITY_ERROR, TAG_ERROR, IO_ERROR。
    pub fn init_nfc_reader(
        &self,
        uuid_sender: watch::Sender<Uuid>,
        error_sender: mpsc::UnboundedSender<NfcErrorResponse>,
        uuid: Uuid,
    ) -> crate::Result<()> {
        let data_channel = Channel::new(move |event| {
            let payload = if let InvokeResponseBody::Json(payload) = event {
                serde_json::from_str(&payload).expect("could not deserialize ble nfc uuid response")
            } else {
                UuidResponse::default()
            };
            let payload = Uuid::parse_str(&payload.value)
                .map_err(|_| Error::InvalidCard)
                .map_err(Into::<tauri::Error>::into)?;
            uuid_sender
                .send(payload)
                .expect("send received nfc message failed");
            Ok(())
        });

        let error_channel = Channel::new(move |event| {
            let payload = if let InvokeResponseBody::Json(payload) = event {
                serde_json::from_str(&payload).expect("could not parse nfc error message")
            } else {
                NfcErrorResponse::default()
            };
            error_sender
                .send(payload)
                .expect("failed to send received nfc error");
            Ok(())
        });

        let uuid = String::from(uuid.simple().encode_upper(&mut Uuid::encode_buffer()));
        self.0
            .run_mobile_plugin(
                "init",
                NfcRequest {
                    data_channel,
                    error_channel,
                    uuid,
                },
            )
            .map_err(Into::into)
    }
}
