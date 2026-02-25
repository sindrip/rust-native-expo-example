package expo.modules.appmobile

import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition
import uniffi.app_mobile.BackgroundTaskCallback
import uniffi.app_mobile.add
import uniffi.app_mobile.delayedAdd
import uniffi.app_mobile.divide
import uniffi.app_mobile.multiply
import uniffi.app_mobile.startBackgroundTask
import uniffi.app_mobile.stopBackgroundTask
import uniffi.app_mobile.subtract

class AppMobileModule : Module() {
  override fun definition() = ModuleDefinition {
    Name("AppMobile")

    Events("onTick")

    Function("add") { left: Long, right: Long ->
      add(left.toULong(), right.toULong()).toLong()
    }

    Function("subtract") { left: Long, right: Long ->
      subtract(left.toULong(), right.toULong()).toLong()
    }

    Function("multiply") { left: Long, right: Long ->
      multiply(left.toULong(), right.toULong()).toLong()
    }

    Function("divide") { left: Long, right: Long ->
      divide(left.toULong(), right.toULong())?.toLong()
    }

    AsyncFunction("delayedAdd") { left: Long, right: Long, delayMs: Long ->
      delayedAdd(left.toULong(), right.toULong(), delayMs.toULong()).toLong()
    }

    Function("startBackgroundTask") {
      startBackgroundTask(object : BackgroundTaskCallback {
        override fun onTick(count: ULong) {
          sendEvent("onTick", mapOf("count" to count.toLong()))
        }
      })
    }

    Function("stopBackgroundTask") {
      stopBackgroundTask()
    }
  }
}
