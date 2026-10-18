import type { Secret } from "../types";

export function validateSecretKey(key: string): string | null {
  const normalizedKey = key.trim();

  if (!normalizedKey) {
    return "The variable name is required";
  }

  if (normalizedKey.length > 256 || !/^[A-Za-z_][A-Za-z0-9_]*$/.test(normalizedKey)) {
    return "Use 1-256 letters, numbers, or underscores; start with a letter or underscore";
  }

  if (normalizedKey.toUpperCase().startsWith("HARBOR_")) {
    return "Variable names beginning with HARBOR_ are reserved";
  }

  return null;
}

export function hasDuplicateSecretKey(secrets: Secret[], key: string, ignoredId?: number): boolean {
  const normalizedKey = key.trim().toUpperCase();

  return secrets.some(
    (secret) => secret.id !== ignoredId && secret.key.trim().toUpperCase() === normalizedKey,
  );
}
