import {
  getActiveRuntimes,
  getCatalog,
  getInstalledVersions,
  setActiveVersion,
} from "../infrastructure/runtimesRepository";
import { RuntimeService, type RuntimeService as RuntimeServiceType } from "../runtimeConstants";
import { cleanVersion } from "../types";

export type RuntimeCatalog = {
  available: string[];
  installed: string[];
  active: string | null;
};

export const mockCatalog: Record<RuntimeServiceType, string[]> = {
  [RuntimeService.PHP]: [
    "8.4.4 (Active)",
    "8.3.17 (Active)",
    "8.2.27 (Security)",
    "8.1.31 (EOL)",
    "7.4.33 (EOL)",
  ],
  [RuntimeService.Apache]: ["2.4.62 (Active)", "2.4.61 (Active)", "2.4.58 (Active)"],
  [RuntimeService.Node]: [
    "22.14.0 (LTS - Jod)",
    "23.8.0 (Current)",
    "20.18.3 (LTS - Iron)",
    "18.20.7 (EOL)",
  ],
};

export function resolveCatalog(
  available: string[],
  installed: string[],
  saved?: string,
): RuntimeCatalog {
  const active = saved && installed.includes(saved) ? saved : (installed[0] ?? null);
  return { available, installed, active };
}

export async function loadCatalogs(): Promise<Record<RuntimeServiceType, RuntimeCatalog>> {
  const saved = await getActiveRuntimes();
  const services: RuntimeServiceType[] = [
    RuntimeService.PHP,
    RuntimeService.Apache,
    RuntimeService.Node,
  ];
  const results = await Promise.all(
    services.map(async (service) => {
      const [available, installed] = await Promise.all([
        getCatalog(service),
        getInstalledVersions(service),
      ]);
      const savedVersion =
        service === RuntimeService.PHP
          ? saved.php
          : service === RuntimeService.Apache
            ? saved.apache
            : saved.nodejs;
      return [service, resolveCatalog(available, installed, savedVersion)] as const;
    }),
  );
  return Object.fromEntries(results) as Record<RuntimeServiceType, RuntimeCatalog>;
}

export function activateCatalogVersion(
  service: RuntimeServiceType,
  version: string,
): Promise<string> {
  return setActiveVersion(service, cleanVersion(version));
}
