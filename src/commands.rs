use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ToastExt;

#[command]
pub(crate) async fn show_toast<R: Runtime>(
    app: AppHandle<R>,
    payload: ToastRequest,
) -> Result<ToastResponse> {
    app.toast().show_toast(payload)
}
