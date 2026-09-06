import { invoke } from "@tauri-apps/api/core";

export type RuntimeService = "PHP" | "Apache" | "Node.js";

const catalogCommands = {
  PHP: "get_php_versions",
  Apache: "get_apache_versions",
  "Node.js": "get_node_versions",
} as const;

const installCommands = {
  PHP: "install_php",
  Apache: "install_apache",
  "Node.js": "install_node",
} as const;

export function initializeWorkspace(): Promise<string> {
  return invoke<string>("initialize_harbor_workspace");
}

export function getActiveRuntimes(): Promise<{ php?: string; nodejs?: string; apache?: string }> {
  return invoke("get_active_runtimes");
}

export function getCatalog(service: RuntimeService): Promise<string[]> {
  return invoke<string[]>(catalogCommands[service]);
}

export function getInstalledVersions(service: RuntimeService): Promise<string[]> {
  return invoke<string[]>("get_installed_versions", { service });
}

export function setActiveVersion(service: RuntimeService, version: string): Promise<string> {
  const commands = {
    PHP: "set_active_php_version",
    Apache: "set_active_apache_version",
    "Node.js": "set_active_node_version",
  } as const;

  return invoke<string>(commands[service], { version });
}

export function configurePhpCliAlias(version: string): Promise<string> {
  return invoke<string>("configure_php_cli_alias", { version });
}

export function installRuntime(service: RuntimeService, version: string): Promise<string> {
  return invoke<string>(installCommands[service], { version });
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
