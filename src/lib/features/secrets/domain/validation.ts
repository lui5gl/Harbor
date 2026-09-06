import type { Secret } from "../types";

export function validateSecretKey(key: string): string | null {
  const normalizedKey = key.trim();

  if (!normalizedKey) {
    return "The variable name is required";
  }

  if (!/^[A-Za-z0-9_]+$/.test(normalizedKey)) {
    return "Only letters, numbers, and underscores are allowed";
  }

  return null;
}

export function hasDuplicateSecretKey(
  secrets: Secret[],
  key: string,
  ignoredId?: number,
): boolean {
  const normalizedKey = key.trim().toUpperCase();

  return secrets.some(
    (secret) =>
      secret.id !== ignoredId &&
      secret.key.trim().toUpperCase() === normalizedKey,
  );
}