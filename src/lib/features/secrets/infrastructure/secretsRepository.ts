import { invoke } from "@tauri-apps/api/core";
import type { SecretsConfiguration } from "../types";

export function loadSecretProfiles(): Promise<SecretsConfiguration> {
  return invoke<SecretsConfiguration>("load_secret_profiles");
}

export function saveSecretProfiles(configuration: SecretsConfiguration): Promise<number> {
  return invoke("save_secret_profiles", { configuration });
}

export function activateSecretProfile(profileId: number): Promise<void> {
  return invoke("activate_secret_profile_for_powershell", { profileId });
}

export function deactivateSecretEnvironment(): Promise<void> {
  return invoke("deactivate_secret_environment");
}

// Opens the secret terminal with the specified profile and terminal executable
export function openSecretTerminal(profileId: number, terminalExecutable: string): Promise<void> {
  return invoke("open_secret_terminal", { profileId, terminalExecutable });
}
