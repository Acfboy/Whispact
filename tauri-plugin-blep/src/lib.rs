use std::sync::Arc;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(mobile)]
pub mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(mobile)]
use mobile::Blep;

pub trait BlepExt<R: Runtime> {
    fn blep(&self) -> Arc<Blep<R>>;
}

impl<R: Runtime, T: Manager<R>> crate::BlepExt<R> for T {
    fn blep(&self) -> Arc<Blep<R>> {
        (*self.state::<Arc<Blep<R>>>()).clone()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("blep")
        .invoke_handler(tauri::generate_handler![])
        .setup(|app, api| {
            #[cfg(mobile)]
            let blep = Arc::new(mobile::init(app, api)?);
            app.manage(blep);
            Ok(())
        })
        .build()
}
