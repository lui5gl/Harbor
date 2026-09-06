import { invoke, isTauri } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

export function openRuntimeUrl(url: string): void {
  if (isTauri()) {
    void openUrl(url);
    return;
  }
  window.open(url, "_blank");
}

export function openRuntimePath(path: string): void {
  if (isTauri()) {
    void invoke("open_directory", { path });
  }
}
