import { invoke, isTauri } from "@tauri-apps/api/core";

export function openWorkspaceDirectory(path: string): Promise<void> {
  if (!isTauri()) return Promise.resolve();
  return invoke("open_directory", { path });
}
