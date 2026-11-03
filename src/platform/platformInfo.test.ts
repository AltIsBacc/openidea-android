import { afterEach, describe, expect, it, vi } from "vitest";

const ANDROID_UA =
  "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130 Mobile Safari/537.36";
const LINUX_UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130 Safari/537.36";

function setUserAgent(ua: string) {
  Object.defineProperty(window.navigator, "userAgent", { configurable: true, get: () => ua });
}

async function loadStore() {
  vi.resetModules();
  return (await import("./platformInfo")).usePlatform;
}

describe("platform info", () => {
  afterEach(() => {
    delete (window as unknown as { __TAURI_INTERNALS__?: unknown }).__TAURI_INTERNALS__;
  });

  it("uses the frameless title bar on desktop Linux", async () => {
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {};
    setUserAgent(LINUX_UA);
    expect((await loadStore()).getState().framelessTitleBar).toBe(true);
  });

  it("draws no window chrome on Android, whose user agent also says Linux", async () => {
    (window as unknown as { __TAURI_INTERNALS__: unknown }).__TAURI_INTERNALS__ = {};
    setUserAgent(ANDROID_UA);
    expect((await loadStore()).getState().framelessTitleBar).toBe(false);
  });

  it("flags Android as mobile and desktop as not", async () => {
    setUserAgent(ANDROID_UA);
    expect((await loadStore()).getState().mobile).toBe(true);
    setUserAgent(LINUX_UA);
    expect((await loadStore()).getState().mobile).toBe(false);
  });
});
