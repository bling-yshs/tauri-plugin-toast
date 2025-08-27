use tauri::{AppHandle, command, Manager, Runtime};

use crate::models::*;
use crate::Result;

#[cfg(mobile)]
use crate::mobile::Toast;

#[command]
pub(crate) async fn show_toast<R: Runtime>(
    _app: AppHandle<R>,
    _payload: ToastRequest,
) -> Result<ToastResponse> {
    #[cfg(mobile)]
    {
        let toast = _app.state::<Toast<R>>().inner();
        return toast.show_toast(_payload);
    }
    #[cfg(not(mobile))]
    {
        // 在非移动平台上返回默认响应
        Ok(ToastResponse { success: false })
    }
}
