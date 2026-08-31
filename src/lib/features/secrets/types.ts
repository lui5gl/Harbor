export type Secret = {
  id: number;
  key: string;
  value: string;
};

export type Environment = {
  id: number;
  name: string;
  isProduction: boolean;
  secrets: Secret[];
};

export type Project = {
  id: number;
  name: string;
  environments: Environment[];
};

export type SecretsConfiguration = {
  projects: Project[];
  activeEnvironmentId: number | null;
};
