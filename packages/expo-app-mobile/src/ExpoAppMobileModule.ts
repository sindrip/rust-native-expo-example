import { NativeModule, requireNativeModule } from "expo";

import { ExpoAppMobileModuleEvents } from "./ExpoAppMobile.types";

declare class ExpoAppMobileModule extends NativeModule<ExpoAppMobileModuleEvents> {
  add(left: number, right: number): number;
  subtract(left: number, right: number): number;
  multiply(left: number, right: number): number;
  divide(left: number, right: number): number | null;
  delayedAdd(left: number, right: number, delayMs: number): Promise<number>;
  startBackgroundTask(): void;
  stopBackgroundTask(): void;
}

// This call loads the native module object from the JSI.
export default requireNativeModule<ExpoAppMobileModule>("ExpoAppMobile");
