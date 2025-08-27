const COMMANDS: &[&str] = &["show_toast"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .build();
}
