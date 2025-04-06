use std::sync::Arc;

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
pub mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Blep;
#[cfg(mobile)]
use mobile::Blep;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the blep APIs.
pub trait BlepExt<R: Runtime> {
  fn blep(&self) -> Arc<Blep<R>>;
}

impl<R: Runtime, T: Manager<R>> crate::BlepExt<R> for T {
  fn blep(&self) -> Arc<Blep<R>> {
    (*self.state::<Arc<Blep<R>>>()).clone()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("blep")
    .invoke_handler(tauri::generate_handler![])
    .setup(|app, api| {
      #[cfg(mobile)]
      let blep = Arc::new(mobile::init(app, api)?);
      #[cfg(desktop)]
      let blep = desktop::init(app, api)?;
      app.manage(blep);
      Ok(())
    })
    .build()
}
