use serde::de::DeserializeOwned;
use serde_json;
use tauri::{
  ipc::{Channel, InvokeResponseBody},
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_blep);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Blep<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.blep", "BlePeripheralPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_blep)?;
  Ok(Blep(handle))
}

/// Access to the blep APIs.
pub struct Blep<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Blep<R> {
  pub fn setup<F>(&self, callback: F) -> crate::Result<()>
  where
    F: Fn(RecvEvent) + Send + Sync + 'static
  {
    let channel = Channel::new(move |event| {
      let payload = match event {
        InvokeResponseBody::Json(payload) => serde_json::from_str::<RecvEvent>(&payload).unwrap_or_else(|err| RecvEvent::error(format!("Could not deserialize {err}"))),
        _ => RecvEvent::error("Unexpected event payload".to_string()),
      };
      callback(payload);
      Ok(())
    });
    self.0
      .run_mobile_plugin("setup", WatchRecvPayload { channel })
      .map_err(Into::into)
  }

  pub fn send(&self, message: String) -> crate::Result<SendResponse> {
    self.0
      .run_mobile_plugin("send", SendRequest { message })
      .map_err(Into::into)
  }
}
