use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Blep<R>> {
    Ok(Blep(app.clone()))
}

/// Access to the mobilesensors APIs.
pub struct Blep<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Blep<R> {}
