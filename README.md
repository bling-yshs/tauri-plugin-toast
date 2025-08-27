# Tauri Plugin Toast

This plugin provides a simple way to show toast notifications in your Tauri application. This is a mobile-only plugin that supports Android platform.

## Preview

<img src="https://raw.githubusercontent.com/bling-yshs/ys-image-host/main/img/image-20250827154024202.png" alt="toast" style="zoom:33%;" />

## Install

1. Install `@bling-yshs/tauri-plugin-toast` in the frontend

   ```bash
   npm i @bling-yshs/tauri-plugin-toast@latest
   ```

2. Install `tauri-plugin-toast` in `src-tauri`

   ```bash
   cargo add tauri-plugin-toast
   ```

3. Add the permission to `src-tauri/capabilities/default.json`

   ```json
   {
     "permissions": [
       "toast:default"
     ]
   }
   ```

4. Add the init code in the Rust entry point

   ```rust
   pub fn run() {
       tauri::Builder::default()
           .plugin(tauri_plugin_toast::init())
           .run(tauri::generate_context!())
           .expect("error while running tauri application");
   }
   ```

## Usage

```javascript
import { showToast } from '@bling-yshs/tauri-plugin-toast'

// Show a short toast message
await showToast('Hello, world!')

// Show a long toast message
await showToast('This is a long message that will be displayed for a longer duration', 'long')
```
