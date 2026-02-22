package expo.modules.appmobile

import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition
import uniffi.app_mobile.BackgroundTaskCallback
import uniffi.app_mobile.add
import uniffi.app_mobile.subtract
import uniffi.app_mobile.multiply
import uniffi.app_mobile.divide
import uniffi.app_mobile.delayedAdd
import uniffi.app_mobile.startBackgroundTask
import uniffi.app_mobile.stopBackgroundTask

class ExpoAppMobileModule : Module() {
  // Each module class must implement the definition function. The definition consists of components
  // that describes the module's functionality and behavior.
  // See https://docs.expo.dev/modules/module-api for more details about available components.
  override fun definition() = ModuleDefinition {
    // Sets the name of the module that JavaScript code will use to refer to the module. Takes a string as an argument.
    // Can be inferred from module's class name, but it's recommended to set it explicitly for clarity.
    // The module will be accessible from `requireNativeModule('ExpoAppMobile')` in JavaScript.
    Name("ExpoAppMobile")

    Events("onTick")

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

    // Defines a JavaScript synchronous function that runs the native code on the JavaScript thread.
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

    // Defines a JavaScript function that always returns a Promise and whose native code
    // is by default dispatched on the different thread than the JavaScript runtime runs on.
    AsyncFunction("delayedAdd") { left: Long, right: Long, delayMs: Long ->
      delayedAdd(left.toULong(), right.toULong(), delayMs.toULong()).toLong()
    }
  }
}
