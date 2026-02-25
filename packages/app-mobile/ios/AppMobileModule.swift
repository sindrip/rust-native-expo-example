import ExpoModulesCore

public class AppMobileModule: Module {
  public func definition() -> ModuleDefinition {
    Name("AppMobile")

    Events("onTick")

    Function("add") { (left: Int, right: Int) -> Int in
      Int(add(left: UInt64(left), right: UInt64(right)))
    }

    Function("subtract") { (left: Int, right: Int) -> Int in
      Int(subtract(left: UInt64(left), right: UInt64(right)))
    }

    Function("multiply") { (left: Int, right: Int) -> Int in
      Int(multiply(left: UInt64(left), right: UInt64(right)))
    }

    Function("divide") { (left: Int, right: Int) -> Int? in
      divide(left: UInt64(left), right: UInt64(right)).map { Int($0) }
    }

    AsyncFunction("delayedAdd") { (left: Int, right: Int, delayMs: Int) -> Int in
      Int(await delayedAdd(left: UInt64(left), right: UInt64(right), delayMs: UInt64(delayMs)))
    }

    Function("startBackgroundTask") {
      startBackgroundTask(callback: AppMobileBackgroundCallback(module: self))
    }

    Function("stopBackgroundTask") {
      stopBackgroundTask()
    }
  }
}

class AppMobileBackgroundCallback: BackgroundTaskCallback {
  private weak var module: AppMobileModule?

  init(module: AppMobileModule) {
    self.module = module
  }

  func onTick(count: UInt64) {
    module?.sendEvent("onTick", ["count": Int(count)])
  }
}
