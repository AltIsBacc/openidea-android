import { usePlatform } from "@/platform/platformInfo";

export const LOCAL_UNAVAILABLE = "Local files are not available on Android. Use Remote Development.";

export function isRemoteOnly(): boolean {
  return usePlatform.getState().mobile;
}

export function useRemoteOnly(): boolean {
  return usePlatform((s) => s.mobile);
}
