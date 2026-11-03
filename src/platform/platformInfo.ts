import { create } from "zustand";
import { commands, isTauri } from "@/platform/ipc";

interface PlatformInfo {
  /** `macos` | `windows` | `linux` | `web`. */
  os: string;
  version: string | null;
  /** Native traffic lights overlay the title bar (Tauri, macOS, Overlay style). */
  overlayTitleBar: boolean;
  /** Frameless window: the title bar draws the main menu and caption buttons (Tauri, Windows and Linux). */
  framelessTitleBar: boolean;
  /** Android: the system draws the window and the screen is small, so decorative chrome is dropped. */
  mobile: boolean;
}

/** OS from the user agent (before the backend answers, or in a browser). "darwin" must not read as Windows. */
export function guessOs(): "macos" | "windows" | "linux" | "web" {
  if (typeof navigator === "undefined") return "web";
  const ua = navigator.userAgent;
  if (/Mac|darwin/i.test(ua)) return "macos";
  if (/Windows|Win32|Win64/i.test(ua)) return "windows";
  if (/Linux/i.test(ua)) return "linux";
  return "web";
}

/**
 * Android's WebView reports "Linux", but the system draws the window: there are no caption buttons or
 * resize edges to replace, so the desktop frameless title bar must not be used.
 */
export function isAndroid(): boolean {
  return typeof navigator !== "undefined" && /Android/i.test(navigator.userAgent);
}

export const usePlatform = create<PlatformInfo>(() => {
  const os = guessOs();
  return {
    os,
    version: null,
    overlayTitleBar: isTauri() && os === "macos",
    framelessTitleBar: isTauri() && !isAndroid() && (os === "windows" || os === "linux"),
    mobile: isAndroid(),
  };
});

export async function loadPlatformInfo() {
  if (!isTauri()) return;
  try {
    const info = await commands.appInfo();
    usePlatform.setState({
      os: info.platform,
      version: info.version,
      overlayTitleBar: info.platform === "macos",
      framelessTitleBar: !isAndroid() && (info.platform === "windows" || info.platform === "linux"),
    });
  } catch {
    /* keep the user-agent guess */
  }
}
