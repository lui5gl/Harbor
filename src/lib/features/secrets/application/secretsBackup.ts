import type { SecretsConfiguration } from "../types";
import { validateSecretsConfiguration } from "./secretsConfiguration";

const ITERATIONS = 600_000;
const MAX_BYTES = 4_194_304;

function encode(bytes: Uint8Array): string {
  return btoa(Array.from(bytes, (byte) => String.fromCharCode(byte)).join(""));
}

function decode(value: unknown, length?: number): Uint8Array<ArrayBuffer> {
  if (
    typeof value !== "string" ||
    value.length > MAX_BYTES * 2 ||
    !/^[A-Za-z0-9+/]*={0,2}$/.test(value)
  )
    throw new Error("Invalid encrypted backup");
  const bytes = Uint8Array.from(atob(value), (character) => character.charCodeAt(0));
  if (length !== undefined && bytes.length !== length) throw new Error("Invalid encrypted backup");
  return bytes;
}

async function deriveKey(password: string, salt: Uint8Array<ArrayBuffer>) {
  const material = await crypto.subtle.importKey(
    "raw",
    new TextEncoder().encode(password),
    "PBKDF2",
    false,
    ["deriveKey"],
  );
  return crypto.subtle.deriveKey(
    { name: "PBKDF2", hash: "SHA-256", salt, iterations: ITERATIONS },
    material,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt", "decrypt"],
  );
}

export function validateBackupConfiguration(value: unknown): SecretsConfiguration {
  if (
    !value ||
    typeof value !== "object" ||
    !("projects" in value) ||
    !Array.isArray(value.projects)
  )
    throw new Error("Invalid backup structure");
  const projectIds = new Set<number>();
  const environmentIds = new Set<number>();
  const identifier = (id: unknown): id is number =>
    typeof id === "number" && Number.isSafeInteger(id) && id > 0;
  const projects = value.projects.map((project) => {
    if (
      !project ||
      !identifier(project.id) ||
      projectIds.has(project.id) ||
      typeof project.name !== "string" ||
      !Array.isArray(project.environments)
    )
      throw new Error("Invalid backup project");
    projectIds.add(project.id);
    return {
      id: project.id,
      name: project.name,
      environments: project.environments.map((environment: unknown) => {
        if (
          !environment ||
          typeof environment !== "object" ||
          !("id" in environment) ||
          !identifier(environment.id) ||
          environmentIds.has(environment.id) ||
          !("name" in environment) ||
          typeof environment.name !== "string" ||
          !("secrets" in environment) ||
          !Array.isArray(environment.secrets)
        )
          throw new Error("Invalid backup environment");
        environmentIds.add(environment.id);
        const secretIds = new Set<number>();
        return {
          id: environment.id,
          name: environment.name,
          secrets: environment.secrets.map((secret) => {
            if (
              !secret ||
              !identifier(secret.id) ||
              secretIds.has(secret.id) ||
              typeof secret.key !== "string" ||
              !secret.key.trim() ||
              typeof secret.value !== "string"
            )
              throw new Error("Invalid backup variable");
            secretIds.add(secret.id);
            return { id: secret.id, key: secret.key, value: secret.value };
          }),
        };
      }),
    };
  });
  const error = validateSecretsConfiguration(projects);
  if (error) throw new Error(error);
  return { projects, activeEnvironmentId: null, revision: 0 };
}

export async function encryptBackup(
  configuration: SecretsConfiguration,
  password: string,
): Promise<string> {
  if (password.length < 12 || password.length > 1024)
    throw new Error("Use a backup passphrase between 12 and 1024 characters");
  const payload = new TextEncoder().encode(
    JSON.stringify(validateBackupConfiguration(configuration)),
  );
  if (payload.length > MAX_BYTES) throw new Error("Backup exceeds the 4 MiB limit");
  const salt = crypto.getRandomValues(new Uint8Array(16));
  const iv = crypto.getRandomValues(new Uint8Array(12));
  const key = await deriveKey(password, salt);
  const ciphertext = await crypto.subtle.encrypt(
    { name: "AES-GCM", iv, additionalData: new TextEncoder().encode("Harbor secrets backup v1") },
    key,
    payload,
  );
  return JSON.stringify({
    format: "harbor-secrets",
    version: 1,
    kdf: "PBKDF2-SHA256",
    iterations: ITERATIONS,
    cipher: "AES-256-GCM",
    salt: encode(salt),
    iv: encode(iv),
    ciphertext: encode(new Uint8Array(ciphertext)),
  });
}

export async function decryptBackup(text: string, password: string): Promise<SecretsConfiguration> {
  if (text.length > MAX_BYTES * 2 || password.length > 1024)
    throw new Error("Backup exceeds the supported limits");
  const envelope = JSON.parse(text);
  if (
    !envelope ||
    envelope.format !== "harbor-secrets" ||
    envelope.version !== 1 ||
    envelope.kdf !== "PBKDF2-SHA256" ||
    envelope.iterations !== ITERATIONS ||
    envelope.cipher !== "AES-256-GCM"
  )
    throw new Error("Unsupported encrypted backup format");
  const salt = decode(envelope.salt, 16);
  const iv = decode(envelope.iv, 12);
  const ciphertext = decode(envelope.ciphertext);
  const key = await deriveKey(password, salt);
  let plaintext: ArrayBuffer;
  try {
    plaintext = await crypto.subtle.decrypt(
      { name: "AES-GCM", iv, additionalData: new TextEncoder().encode("Harbor secrets backup v1") },
      key,
      ciphertext,
    );
  } catch {
    throw new Error("Incorrect passphrase or damaged backup. Nothing has been restored.");
  }
  return validateBackupConfiguration(JSON.parse(new TextDecoder().decode(plaintext)));
}
