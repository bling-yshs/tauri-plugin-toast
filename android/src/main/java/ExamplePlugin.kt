package com.plugin.toast

import android.app.Activity
import android.widget.Toast
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class ToastArgs {
  var message: String? = null
  var duration: String? = null
}

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example()

    @Command
    fun showToast(invoke: Invoke) {
        try {
            val args = invoke.parseArgs(ToastArgs::class.java)
            val message = args.message ?: "Hello from Tauri!"
            val duration = when (args.duration) {
                "long" -> Toast.LENGTH_LONG
                else -> Toast.LENGTH_SHORT
            }

            // 确保在主线程中显示 Toast
            activity.runOnUiThread {
                Toast.makeText(activity, message, duration).show()
            }

            val ret = JSObject()
            ret.put("success", true)
            invoke.resolve(ret)
        } catch (e: Exception) {
            android.util.Log.e("ToastPlugin", "显示Toast失败", e)
            val ret = JSObject()
            ret.put("success", false)
            invoke.resolve(ret)
        }
    }
}
