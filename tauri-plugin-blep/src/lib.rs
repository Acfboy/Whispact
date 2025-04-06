use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

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
  fn blep(&self) -> &Blep<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BlepExt<R> for T {
  fn blep(&self) -> &Blep<R> {
    self.state::<Blep<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("blep")
    .invoke_handler(tauri::generate_handler![])
    .setup(|app, api| {
      #[cfg(mobile)]
      let blep = mobile::init(app, api)?;
      #[cfg(desktop)]
      let blep = desktop::init(app, api)?;
      app.manage(blep);
      Ok(())
    })
    .build()
}
