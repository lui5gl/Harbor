import { parse } from "./dotenvParser";
import { validateSecretKey } from "../domain/validation";

export type ParsedVariable = { key: string; value: string };

export function parseEnvironment(text: string) {
  if (text.length > 1_048_576) throw new Error("Environment imports are limited to 1 MiB");
  const parsed = parse(text);
  const variables = new Map<string, ParsedVariable>();
  const errors: string[] = [];
  let duplicates = 0;
  for (const [key, value] of Object.entries(parsed)) {
    const error = validateSecretKey(key);
    if (error || value.includes("\0")) {
      errors.push(`${key}: ${error ?? "Null characters are not supported"}`);
      continue;
    }
    const normalized = key.toUpperCase();
    if (variables.has(normalized)) duplicates++;
    variables.set(normalized, { key, value });
  }
  return { variables: [...variables.values()], errors, duplicates };
}

export function formatEnvironment(variables: readonly ParsedVariable[]): string {
  return variables
    .filter((variable) => variable.key.trim())
    .map(({ key, value }) => {
      const normalized = key.trim();
      const error = validateSecretKey(normalized);
      if (error || value.includes("\0"))
        throw new Error(error ?? "Null characters are not supported");
      for (const candidate of [value, `'${value}'`, `"${value}"`, `\`${value}\``]) {
        const assignment = `${normalized}=${candidate}`;
        const parsed = parse(assignment);
        if (Object.keys(parsed).length === 1 && parsed[normalized] === value) return assignment;
      }
      throw new Error(
        `Variable ${normalized} cannot be exported losslessly as .env. Use an encrypted backup instead.`,
      );
    })
    .join("\n");
}
