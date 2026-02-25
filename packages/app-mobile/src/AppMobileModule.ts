import { NativeModule, requireNativeModule } from "expo";

import type { AppMobileModuleEvents } from "./AppMobile.types";

declare class AppMobileModule extends NativeModule<AppMobileModuleEvents> {
  add(left: number, right: number): number;
  subtract(left: number, right: number): number;
  multiply(left: number, right: number): number;
  divide(left: number, right: number): number | null;
  delayedAdd(left: number, right: number, delayMs: number): Promise<number>;
  startBackgroundTask(): void;
  stopBackgroundTask(): void;
}

export default requireNativeModule<AppMobileModule>("AppMobile");
