import { invoke } from '@tauri-apps/api/core'

export async function showToast(
  message: string,
  duration: 'short' | 'long' = 'short'
): Promise<boolean> {
  return await invoke<{success?: boolean}>('plugin:toast|show_toast', {
    payload: {
      message,
      duration,
    },
  }).then((r) => r.success || false);
}
