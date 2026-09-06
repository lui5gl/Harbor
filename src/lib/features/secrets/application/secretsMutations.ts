import type { Environment, Project, Secret } from "../types";

type SecretField = "key" | "value";
type SecretInput = Pick<Secret, "key" | "value">;

export type SecretCounters = {
  nextProjectId: number;
  nextEnvironmentId: number;
  nextSecretId: number;
};

export function getSecretCounters(projects: readonly Project[]): SecretCounters {
  const projectIds = projects.map((project) => project.id);
  const environmentIds = projects.flatMap((project) =>
    project.environments.map((environment) => environment.id),
  );
  const secretIds = projects.flatMap((project) =>
    project.environments.flatMap((environment) => environment.secrets.map((secret) => secret.id)),
  );

  return {
    nextProjectId: Math.max(0, ...projectIds) + 1,
    nextEnvironmentId: Math.max(0, ...environmentIds) + 1,
    nextSecretId: Math.max(0, ...secretIds) + 1,
  };
}

export function updateProject(
  projects: readonly Project[],
  projectId: number,
  update: (project: Project) => Project,
): Project[] {
  return projects.map((project) => (project.id === projectId ? update(project) : project));
}

export function updateEnvironment(
  projects: readonly Project[],
  projectId: number | null,
  environmentId: number | null,
  update: (environment: Environment) => Environment,
): Project[] {
  if (projectId === null || environmentId === null) return [...projects];
  return updateProject(projects, projectId, (project) => ({
    ...project,
    environments: project.environments.map((environment) =>
      environment.id === environmentId ? update(environment) : environment,
    ),
  }));
}

export function appendProject(
  projects: readonly Project[],
  projectId: number,
  environmentId: number,
  name: string,
  environmentName: string,
): Project[] {
  return [
    ...projects,
    {
      id: projectId,
      name,
      environments: [
        { id: environmentId, name: environmentName, isProduction: false, secrets: [] },
      ],
    },
  ];
}

export function appendEnvironment(
  projects: readonly Project[],
  projectId: number,
  environmentId: number,
  name: string,
  isProduction: boolean,
): Project[] {
  return updateProject(projects, projectId, (project) => ({
    ...project,
    environments: [...project.environments, { id: environmentId, name, isProduction, secrets: [] }],
  }));
}

export function appendSecret(
  projects: readonly Project[],
  projectId: number | null,
  environmentId: number | null,
  secretId: number,
  input: SecretInput,
): Project[] {
  return updateEnvironment(projects, projectId, environmentId, (environment) => ({
    ...environment,
    secrets: [...environment.secrets, { id: secretId, ...input }],
  }));
}

export function updateSecret(
  projects: readonly Project[],
  projectId: number | null,
  environmentId: number | null,
  secretId: number,
  field: SecretField,
  value: string,
): Project[] {
  return updateEnvironment(projects, projectId, environmentId, (environment) => ({
    ...environment,
    secrets: environment.secrets.map((secret) =>
      secret.id === secretId ? { ...secret, [field]: value } : secret,
    ),
  }));
}

export function removeSecret(
  projects: readonly Project[],
  projectId: number | null,
  environmentId: number | null,
  secretId: number,
): Project[] {
  return updateEnvironment(projects, projectId, environmentId, (environment) => ({
    ...environment,
    secrets: environment.secrets.filter((secret) => secret.id !== secretId),
  }));
}
