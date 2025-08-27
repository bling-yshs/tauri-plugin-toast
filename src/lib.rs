use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(mobile)]
use mobile::Toast;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the toast APIs.
pub trait ToastExt<R: Runtime> {
  fn toast(&self) -> &Toast<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ToastExt<R> for T {
  fn toast(&self) -> &Toast<R> {
    self.state::<Toast<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("toast")
    .invoke_handler(tauri::generate_handler![commands::show_toast])
    .setup(|app, api| {
      #[cfg(mobile)]
      let toast = mobile::init(app, api)?;
      app.manage(toast);
      Ok(())
    })
    .build()
}
