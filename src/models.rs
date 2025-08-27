use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToastRequest {
  pub message: String,
  pub duration: Option<String>, // "short" or "long"
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToastResponse {
  pub success: bool,
}
