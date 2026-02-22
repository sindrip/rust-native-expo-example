import ExpoModulesCore

private final class TickCallback: BackgroundTaskCallback, @unchecked Sendable {
  private let emit: (UInt64) -> Void

  init(emit: @escaping (UInt64) -> Void) {
    self.emit = emit
  }

  func onTick(count: UInt64) {
    emit(count)
  }
}

public class ExpoAppMobileModule: Module {
  // Each module class must implement the definition function. The definition consists of components
  // that describes the module's functionality and behavior.
  // See https://docs.expo.dev/modules/module-api for more details about available components.
  public func definition() -> ModuleDefinition {
    // Sets the name of the module that JavaScript code will use to refer to the module. Takes a string as an argument.
    // Can be inferred from module's class name, but it's recommended to set it explicitly for clarity.
    // The module will be accessible from `requireNativeModule('ExpoAppMobile')` in JavaScript.
    Name("ExpoAppMobile")

    Events("onTick")

    // Defines a JavaScript synchronous function that runs the native code on the JavaScript thread.
    Function("add") { (left: UInt64, right: UInt64) -> UInt64 in
      add(left: left, right: right)
    }

    Function("subtract") { (left: UInt64, right: UInt64) -> UInt64 in
      subtract(left: left, right: right)
    }

    Function("multiply") { (left: UInt64, right: UInt64) -> UInt64 in
      multiply(left: left, right: right)
    }

    Function("divide") { (left: UInt64, right: UInt64) -> UInt64? in
      divide(left: left, right: right)
    }

    Function("startBackgroundTask") { [weak self] in
      let callback = TickCallback { count in
        self?.sendEvent("onTick", ["count": count])
      }
      startBackgroundTask(callback: callback)
    }

    Function("stopBackgroundTask") {
      stopBackgroundTask()
    }

    // Defines a JavaScript function that always returns a Promise and whose native code
    // is by default dispatched on the different thread than the JavaScript runtime runs on.
    AsyncFunction("delayedAdd") { (left: UInt64, right: UInt64, delayMs: UInt64) async -> UInt64 in
      print("[delayedAdd] called: \(Date())")
      let result = await delayedAdd(left: left, right: right, delayMs: delayMs)
      print("[delayedAdd] returned: \(Date())")
      return result
    }
  }
}
