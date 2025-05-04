use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(mobile)]
mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(mobile)]
use mobile::Nfc2;

pub trait Nfc2Ext<R: Runtime> {
    fn nfc2(&self) -> &Nfc2<R>;
}

impl<R: Runtime, T: Manager<R>> crate::Nfc2Ext<R> for T {
    fn nfc2(&self) -> &Nfc2<R> {
        self.state::<Nfc2<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nfc2")
        .invoke_handler(tauri::generate_handler![])
        .setup(|app, api| {
            #[cfg(mobile)]
            let nfc2 = mobile::init(app, api)?;
            app.manage(nfc2);
            Ok(())
        })
        .build()
}
