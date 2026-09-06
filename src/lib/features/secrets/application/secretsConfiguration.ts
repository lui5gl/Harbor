import { hasDuplicateSecretKey, validateSecretKey } from "../domain/validation";
import type { Project, SecretsConfiguration } from "../types";

export function validateSecretsConfiguration(projects: readonly Project[]): string | null {
  for (const project of projects) {
    for (const environment of project.environments) {
      for (const secret of environment.secrets) {
        const key = secret.key.trim();
        if (!key) continue;
        if (validateSecretKey(key)) {
          return `Variable "${key}" in ${environment.name} may only use letters, numbers, and underscores`;
        }
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
  };
}
