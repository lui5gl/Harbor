import { invoke, isTauri } from "@tauri-apps/api/core";

export type DetectedTerminal = { id: string; label: string };

const webFallbackTerminals: DetectedTerminal[] = [
  { id: "powershell", label: "Windows PowerShell" },
];

export async function listTerminals(): Promise<DetectedTerminal[]> {
  if (!isTauri()) return webFallbackTerminals;
  return invoke<DetectedTerminal[]>("list_terminals");
}

export function openWorkspaceDirectory(path: string): Promise<void> {
  if (!isTauri()) return Promise.resolve();
  return invoke("open_directory", { path });
}
