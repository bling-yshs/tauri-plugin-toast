use tauri::{
  plugin::{Builder, TauriPlugin},
  Runtime,
};

pub use models::*;

#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("toast")
    .invoke_handler(tauri::generate_handler![commands::show_toast])
    .setup(|_app, _api| {
      #[cfg(mobile)]
      {
        use tauri::Manager;
        let toast = mobile::init(_app, _api)?;
        _app.manage(toast);
      }
      Ok(())
    })
    .build()
}
