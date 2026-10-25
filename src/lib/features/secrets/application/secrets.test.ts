import { describe, expect, it } from "vitest";
import { formatEnvironment, parseEnvironment } from "./environmentFormat";
import { generateSecret } from "./secretGenerator";
import { validateSecretKey } from "../domain/validation";
import { decryptBackup, encryptBackup, validateBackupConfiguration } from "./secretsBackup";
import { cloneEnvironment } from "./secretsMutations";

describe("environment files", () => {
  it("round-trips supported values without revealing or interpolating them", () => {
    const values = [
      "",
      "hello",
      " spaced ",
      "#hash",
      "one\ntwo",
      'a"b',
      "a'b",
      "a`b",
      "C:\\temp\\new",
      "$TOKEN",
      "literal\\n",
    ];
    const variables = values.map((value, index) => ({ key: `KEY_${index}`, value }));
    expect(parseEnvironment(formatEnvironment(variables)).variables).toEqual(variables);
  });

  it("rejects values that cannot be exported losslessly", () => {
    expect(() => formatEnvironment([{ key: "TOKEN", value: "'\"`\n\\n'\"`" }])).toThrow();
  });

  it("validates imported keys and resolves case-insensitive duplicates", () => {
    const parsed = parseEnvironment("export TOKEN=first\ntoken=last\n1BAD=no\nHARBOR_CUSTOM=no");
    expect(parsed.variables).toEqual([{ key: "token", value: "last" }]);
    expect(parsed.errors).toHaveLength(2);
    expect(parsed.duplicates).toBe(1);
  });
});

describe("secret generation and validation", () => {
  it("rejects reserved and nonportable keys", () => {
    for (const key of ["1TOKEN", "harbor_custom", "BAD-KEY", "A".repeat(257)])
      expect(validateSecretKey(key)).not.toBeNull();
    for (const key of ["TOKEN", "_TOKEN", "API_KEY_2"]) expect(validateSecretKey(key)).toBeNull();
  });

  it("generates the requested formats", () => {
    expect(generateSecret("hex256", 16)).toMatch(/^[a-f0-9]{64}$/);
    expect(generateSecret("hex128", 16)).toMatch(/^[a-f0-9]{32}$/);
    expect(generateSecret("token", 16)).toMatch(/^[A-Za-z0-9]{32}$/);
    expect(generateSecret("uuid", 16)).toMatch(/^[a-f0-9-]{36}$/);
    for (let index = 0; index < 20; index++) {
      const password = generateSecret("password", 24);
      expect(password).toHaveLength(24);
      expect(password).toMatch(/[a-z]/);
      expect(password).toMatch(/[A-Z]/);
      expect(password).toMatch(/[0-9]/);
      expect(password).toMatch(/[^A-Za-z0-9]/);
    }
    expect(() => generateSecret("password", 0)).toThrow();
    expect(() => generateSecret("password", Infinity)).toThrow();
  });
});

describe("encrypted backups", () => {
  const configuration = {
    projects: [
      {
        id: 1,
        name: "Project",
        environments: [
          {
            id: 2,
            name: "Dev",
            secrets: [{ id: 3, key: "TOKEN", value: "private-backup-value" }],
          },
        ],
      },
    ],
    activeEnvironmentId: null,
    revision: 0,
  };

  it("round-trips encrypted backups and rejects wrong passwords or modifications", async () => {
    const encrypted = await encryptBackup(configuration, "a long backup passphrase");
    expect(encrypted).not.toContain("private-backup-value");
    expect(await decryptBackup(encrypted, "a long backup passphrase")).toEqual(configuration);
    await expect(decryptBackup(encrypted, "incorrect password")).rejects.toThrow(
      "Incorrect passphrase",
    );
    const changed = JSON.parse(encrypted);
    changed.ciphertext = (changed.ciphertext[0] === "A" ? "B" : "A") + changed.ciphertext.slice(1);
    await expect(
      decryptBackup(JSON.stringify(changed), "a long backup passphrase"),
    ).rejects.toThrow();
  });

  it("rejects malformed structures, duplicate ids and short passphrases", async () => {
    expect(() => validateBackupConfiguration({ projects: [null] })).toThrow();
    expect(() =>
      validateBackupConfiguration({
        projects: [configuration.projects[0], configuration.projects[0]],
      }),
    ).toThrow();
    await expect(encryptBackup(configuration, "short")).rejects.toThrow();
    await expect(decryptBackup('{"version":999}', "passphrase")).rejects.toThrow();
  });
});

describe("environment cloning", () => {
  it("deep clones variables with unique sequential IDs", () => {
    const projects = [
      {
        id: 1,
        name: "Backend",
        environments: [
          {
            id: 10,
            name: "Production",
            secrets: [
              { id: 101, key: "DB_URL", value: "postgres://prod" },
              { id: 102, key: "API_KEY", value: "prod-secret-token" },
            ],
          },
        ],
      },
    ];

    const result = cloneEnvironment(projects, 1, 10, 20, "Staging", 200);
    const project = result.find((p) => p.id === 1)!;
    expect(project.environments).toHaveLength(2);

    const cloned = project.environments.find((e) => e.id === 20)!;
    expect(cloned.name).toBe("Staging");
    expect(cloned.secrets).toHaveLength(2);
    expect(cloned.secrets[0]).toEqual({ id: 200, key: "DB_URL", value: "postgres://prod" });
    expect(cloned.secrets[1]).toEqual({ id: 201, key: "API_KEY", value: "prod-secret-token" });

    // Original must remain untouched
    const original = project.environments.find((e) => e.id === 10)!;
    expect(original.secrets[0].id).toBe(101);
  });
});
