import { isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  getInstalledVersions,
  getPhpStatus,
  initializeWorkspace,
  installRuntime,
  removeRuntime,
  startPhp,
  stopPhp,
} from "../infrastructure/runtimesRepository";
import { cleanVersion } from "../types";
import {
  RuntimeService,
  RuntimeTiming,
  type RuntimeService as RuntimeServiceType,
} from "../runtimeConstants";
import {
  activateCatalogVersion,
  loadCatalogs,
  mockCatalog,
  type RuntimeCatalog,
} from "./runtimeCatalog";

export type RuntimeStore = {
  readonly php: RuntimeCatalog;
  readonly apache: RuntimeCatalog;
  readonly node: RuntimeCatalog;
  readonly isPhpRunning: boolean;
  readonly isCatalogLoading: boolean;
  readonly catalogError: string;
  readonly isInstalling: boolean;
  readonly installProgress: number;
  readonly installingService: string;
  readonly installingVersion: string;
  readonly installError: string;
  load: () => Promise<void>;
  refresh: () => Promise<void>;
  selectPhpVersion: (version: string) => Promise<void>;
  togglePhp: () => Promise<void>;
  selectNodeVersion: (version: string) => Promise<void>;
  install: (service: RuntimeServiceType, rawVersion: string) => Promise<void>;
  remove: (service: RuntimeServiceType, version: string) => Promise<void>;
  start: () => void;
  dispose: () => void;
};

const native = isTauri();

export function createRuntimesStore(): RuntimeStore {
  let php = $state<RuntimeCatalog>({ available: [], installed: [], active: null });
  let apache = $state<RuntimeCatalog>({ available: [], installed: [], active: null });
  let node = $state<RuntimeCatalog>({ available: [], installed: [], active: null });
  let isPhpRunning = $state(false);
  let isCatalogLoading = $state(true);
  let catalogError = $state("");
  let isInstalling = $state(false);
  let installProgress = $state(0);
  let installingService = $state("");
  let installingVersion = $state("");
  let installError = $state("");
  let unlisten: (() => void) | undefined;

  function catalog(service: RuntimeServiceType): RuntimeCatalog {
    return service === "PHP" ? php : service === "Apache" ? apache : node;
  }

  function setCatalog(service: RuntimeServiceType, value: RuntimeCatalog) {
    if (service === "PHP") php = value;
    else if (service === "Apache") apache = value;
    else node = value;
  }

  async function refresh() {
    if (!native) return;
    isCatalogLoading = true;
    catalogError = "";
    try {
      const loaded = await loadCatalogs();
      for (const service of [RuntimeService.PHP, RuntimeService.Apache, RuntimeService.Node])
        setCatalog(service, loaded[service]);
    } catch (error) {
      catalogError = error instanceof Error ? error.message : String(error);
    } finally {
      isCatalogLoading = false;
    }
  }

  async function load() {
    if (!native) {
      php = { available: mockCatalog[RuntimeService.PHP], installed: ["8.3.17"], active: "8.3.17" };
      apache = {
        available: mockCatalog[RuntimeService.Apache],
        installed: ["2.4.62"],
        active: "2.4.62",
      };
      node = {
        available: mockCatalog[RuntimeService.Node],
        installed: ["22.14.0"],
        active: "22.14.0",
      };
      isCatalogLoading = false;
      return;
    }
    try {
      await initializeWorkspace();
      await refresh();
      isPhpRunning = await getPhpStatus();
    } catch (error) {
      catalogError = error instanceof Error ? error.message : String(error);
      isCatalogLoading = false;
    }
  }

  async function selectPhpVersion(version: string) {
    php.active = cleanVersion(version);
    if (!native) return;
    try {
      await activateCatalogVersion(RuntimeService.PHP, php.active);
      if (isPhpRunning) await startPhp(php.active);
    } catch (error) {
      catalogError = error instanceof Error ? error.message : String(error);
    }
  }

  async function togglePhp() {
    if (!php.active) return;
    if (!native) {
      isPhpRunning = !isPhpRunning;
      return;
    }
    try {
      if (isPhpRunning) {
        await stopPhp();
        isPhpRunning = false;
      } else {
        await startPhp(php.active);
        isPhpRunning = true;
      }
    } catch (error) {
      catalogError = error instanceof Error ? error.message : String(error);
    }
  }

  async function selectNodeVersion(version: string) {
    node.active = cleanVersion(version);
    if (!native) return;
    try {
      await activateCatalogVersion(RuntimeService.Node, node.active);
    } catch (error) {
      catalogError = error instanceof Error ? error.message : String(error);
    }
  }

  async function install(service: RuntimeService, rawVersion: string) {
    const version = cleanVersion(rawVersion);
    isInstalling = true;
    installingService = service;
    installingVersion = version;
    installError = "";
    try {
      if (native) await installRuntime(service, version);
      else
        await new Promise((resolve) =>
          window.setTimeout(resolve, RuntimeTiming.mockInstallDelayMs),
        );
      const current = catalog(service);
      const installed = native
        ? await getInstalledVersions(service)
        : [...new Set([...current.installed, version])];
      setCatalog(service, { ...current, installed, active: current.active ?? version });
    } catch (error) {
      installError = error instanceof Error ? error.message : String(error);
    } finally {
      isInstalling = false;
      installingService = "";
      installingVersion = "";
      installProgress = 0;
    }
  }

  async function remove(service: RuntimeService, version: string) {
    try {
      if (service === RuntimeService.PHP && native && php.active === version && isPhpRunning) {
        await stopPhp();
        isPhpRunning = false;
      }
      if (native) await removeRuntime(service, version);
      const current = catalog(service);
      const installed = native
        ? await getInstalledVersions(service)
        : current.installed.filter((item) => cleanVersion(item) !== version);
      setCatalog(service, {
        ...current,
        installed,
        active: current.active === version ? (installed[0] ?? null) : current.active,
      });
    } catch (error) {
      catalogError = error instanceof Error ? error.message : String(error);
    }
  }

  function start() {
    if (!native || unlisten) return;
    listen<{ service: string; progress: number }>("runtime-download-progress", (event) => {
      if (event.payload.service === installingService) installProgress = event.payload.progress;
    }).then((cleanup) => (unlisten = cleanup));
  }
  function dispose() {
    unlisten?.();
    unlisten = undefined;
  }

  return {
    get php() {
      return php;
    },
    get apache() {
      return apache;
    },
    get node() {
      return node;
    },
    get isPhpRunning() {
      return isPhpRunning;
    },
    get isCatalogLoading() {
      return isCatalogLoading;
    },
    get catalogError() {
      return catalogError;
    },
    get isInstalling() {
      return isInstalling;
    },
    get installProgress() {
      return installProgress;
    },
    get installingService() {
      return installingService;
    },
    get installingVersion() {
      return installingVersion;
    },
    get installError() {
      return installError;
    },
    load,
    refresh,
    selectPhpVersion,
    togglePhp,
    selectNodeVersion,
    install,
    remove,
    start,
    dispose,
  };
}
