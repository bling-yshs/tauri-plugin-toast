# Tauri Plugin Toast

This plugin provides a simple way to show toast notifications in your Tauri application. This is a mobile-only plugin that supports Android platform.

## Usage

```javascript
import { showToast } from '@bling-yshs/tauri-plugin-toast'

// Show a short toast message
await showToast('Hello, world!')

// Show a long toast message
await showToast('This is a long message that will be displayed for a longer duration', 'long')
```

## API

### `showToast(message: string, duration: 'short' | 'long' = 'short'): Promise<boolean>`

Shows a toast notification with the specified message and duration.

- `message`: The message to display in the toast
- `duration`: The duration for which the toast should be displayed ('short' or 'long')

Returns a promise that resolves to `true` if the toast was successfully displayed, `false` otherwise.