use serde::de::DeserializeOwned;
use tauri::{
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
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
}
