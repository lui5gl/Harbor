import { hasDuplicateSecretKey, validateSecretKey } from "../domain/validation";
import type { Project, SecretsConfiguration } from "../types";

export function validateSecretsConfiguration(projects: readonly Project[]): string | null {
  const projectNames = new Set<string>();
  for (const project of projects) {
    const name = project.name.trim();
    if (!name || new TextEncoder().encode(name).length > 80)
      return "Project names must contain 1-80 UTF-8 bytes";
    if (projectNames.has(name.toLowerCase())) return "Project names must be unique";
    projectNames.add(name.toLowerCase());
    const environmentNames = new Set<string>();
    for (const environment of project.environments) {
      const environmentName = environment.name.trim();
      if (!environmentName || new TextEncoder().encode(environmentName).length > 80)
        return "Environment names must contain 1-80 UTF-8 bytes";
      if (environmentNames.has(environmentName.toLowerCase()))
        return `Environment names in ${name} must be unique`;
      environmentNames.add(environmentName.toLowerCase());
      for (const secret of environment.secrets) {
        const key = secret.key.trim();
        if (!key) continue;
        const keyError = validateSecretKey(key);
        if (keyError) {
          return `Variable "${key}" in ${environment.name}: ${keyError}`;
        }
        if (secret.value.includes("\0")) return `Variable "${key}" cannot contain a null character`;
        if (hasDuplicateSecretKey(environment.secrets, key, secret.id)) {
          return `Variable "${key}" is duplicated in ${environment.name}`;
        }
      }
    }
  }
  return null;
}

export function createSecretsConfiguration(
  projects: readonly Project[],
  activeEnvironmentId: number | null,
  revision = 0,
): SecretsConfiguration {
  return {
    projects: projects.map((project) => ({
      ...project,
      environments: project.environments.map((environment) => ({
        ...environment,
        secrets: environment.secrets.filter((secret) => secret.key.trim()),
      })),
    })),
    activeEnvironmentId,
    revision,
  };
}
