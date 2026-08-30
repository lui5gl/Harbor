export type Secret = {
  id: number;
  key: string;
  value: string;
};

export type Profile = {
  id: number;
  name: string;
  isProduction: boolean;
  secrets: Secret[];
};

export type SecretsConfiguration = {
  profiles: Profile[];
  activeProfileId: number | null;
};

export type ProfileSort = "manual" | "production" | "name";
