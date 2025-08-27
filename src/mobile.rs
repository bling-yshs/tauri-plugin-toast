use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

// initializes the Kotlin plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Toast<R>> {
  let handle = api.register_android_plugin("com.plugin.toast", "ExamplePlugin")?;
  Ok(Toast(handle))
}

/// Access to the toast APIs.
pub struct Toast<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Toast<R> {
  pub fn show_toast(&self, payload: ToastRequest) -> crate::Result<ToastResponse> {
    self
      .0
      .run_mobile_plugin("showToast", payload)
      .map_err(Into::into)
  }
}
