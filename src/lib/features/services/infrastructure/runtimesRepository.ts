import { invoke } from "@tauri-apps/api/core";
import { RuntimeServiceCommand, type RuntimeService } from "../runtimeConstants";

export function initializeWorkspace(): Promise<string> {
  return invoke<string>("initialize_harbor_workspace");
}

export function getActiveRuntimes(): Promise<{ php?: string; nodejs?: string; apache?: string }> {
  return invoke("get_active_runtimes");
}

export function getCatalog(service: RuntimeService): Promise<string[]> {
  return invoke<string[]>(RuntimeServiceCommand.getCatalog[service]);
}

export function getInstalledVersions(service: RuntimeService): Promise<string[]> {
  return invoke<string[]>("get_installed_versions", { service });
}

export function setActiveVersion(service: RuntimeService, version: string): Promise<string> {
  return invoke<string>(RuntimeServiceCommand.activate[service], { version });
}

export function configurePhpCliAlias(version: string): Promise<string> {
  return invoke<string>("configure_php_cli_alias", { version });
}

export function installRuntime(service: RuntimeService, version: string): Promise<string> {
  return invoke<string>(RuntimeServiceCommand.install[service], { version });
}

export function removeRuntime(service: RuntimeService, version: string): Promise<void> {
  return invoke("remove_runtime", { service, version });
}

export function getPhpStatus(): Promise<boolean> {
  return invoke<boolean>("get_php_status");
}

export function startPhp(version: string): Promise<string> {
  return invoke<string>("start_php", { version });
}

export function stopPhp(): Promise<void> {
  return invoke("stop_php");
}
