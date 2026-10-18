import type { Environment, Project } from "../types";
import { formatEnvironment } from "./environmentFormat";

export function exportEnvironmentText(environment: Environment): string {
  return formatEnvironment(environment.secrets);
}

export async function copyEnvironment(environment: Environment): Promise<void> {
  await navigator.clipboard.writeText(exportEnvironmentText(environment));
}

export function downloadEnvironment(project: Project, environment: Environment): void {
  const filePart = (value: string) => value.toLowerCase().replace(/[^a-z0-9_-]/g, "-") || "local";
  const url = URL.createObjectURL(
    new Blob([exportEnvironmentText(environment)], { type: "text/plain;charset=utf-8" }),
  );
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `.env.${filePart(project.name)}.${filePart(environment.name)}`;
  anchor.click();
  URL.revokeObjectURL(url);
}
