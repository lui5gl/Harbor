import { isTauri } from "@tauri-apps/api/core";
import {
  configurePhpCliAlias,
  getInstalledVersions,
  getPhpStatus,
  setActiveVersion,
  startPhp,
  stopPhp,
} from "$lib/features/services/infrastructure/runtimesRepository";
import { RuntimeService, RuntimeTiming } from "$lib/features/services/runtimeConstants";

const isNativeApp = isTauri();

type ServiceState = {
  installedVersions: string[];
  selectedVersion: string;
};

export type QuickTrayServices = {
  readonly php: ServiceState;
  readonly node: ServiceState;
  readonly apache: ServiceState;
  readonly isPhpRunning: boolean;
  readonly isBusy: boolean;
  readonly error: string;
  load: () => Promise<void>;
  refreshPhpStatus: () => Promise<void>;
  selectPhpVersion: (version: string) => Promise<void>;
  togglePhp: () => Promise<void>;
  selectNodeVersion: (version: string) => Promise<void>;
  start: () => void;
  dispose: () => void;
};

export function createQuickTrayServices(onStatus: (message: string) => void): QuickTrayServices {
  let php = $state<ServiceState>({ installedVersions: [], selectedVersion: "" });
  let node = $state<ServiceState>({ installedVersions: [], selectedVersion: "" });
  let apache = $state<ServiceState>({ installedVersions: [], selectedVersion: "" });
  let isPhpRunning = $state(false);
  let isBusy = $state(false);
  let error = $state("");
  let intervalId: number | undefined;

  function selectVersion(current: ServiceState, installedVersions: string[]): ServiceState {
    const selectedVersion = installedVersions.includes(current.selectedVersion)
      ? current.selectedVersion
      : (installedVersions[0] ?? "");
    return { installedVersions, selectedVersion };
  }

  async function load() {
    if (!isNativeApp) {
      php = { installedVersions: ["8.3.17", "8.2.27"], selectedVersion: "8.3.17" };
      node = { installedVersions: ["22.14.0", "20.18.3"], selectedVersion: "22.14.0" };
      apache = { installedVersions: ["2.4.62"], selectedVersion: "2.4.62" };
      isPhpRunning = false;
      return;
    }

    try {
      const [phpInstalled, nodeInstalled, apacheInstalled, running] = await Promise.all([
        getInstalledVersions(RuntimeService.PHP),
        getInstalledVersions(RuntimeService.Node),
        getInstalledVersions(RuntimeService.Apache),
        getPhpStatus(),
      ]);
      php = selectVersion(php, phpInstalled);
      node = selectVersion(node, nodeInstalled);
      apache = selectVersion(apache, apacheInstalled);
      isPhpRunning = running;
      error = "";
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    }
  }

  async function refreshPhpStatus() {
    if (!isNativeApp) return;
    try {
      isPhpRunning = await getPhpStatus();
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    }
  }

  async function selectPhpVersion(version: string) {
    php = { ...php, selectedVersion: version };
    if (!isNativeApp || !version) return;
    isBusy = true;
    try {
      await configurePhpCliAlias(version);
      if (isPhpRunning) await startPhp(version);
      onStatus(`PHP CLI configurado a v${version}`);
      error = "";
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    } finally {
      isBusy = false;
    }
  }

  async function togglePhp() {
    if (!isNativeApp) {
      isPhpRunning = !isPhpRunning;
      return;
    }
    if (!php.selectedVersion) return;
    isBusy = true;
    try {
      if (isPhpRunning) {
        await stopPhp();
        isPhpRunning = false;
        onStatus("PHP FastCGI detenido");
      } else {
        await startPhp(php.selectedVersion);
        isPhpRunning = true;
        onStatus(`PHP FastCGI iniciado (v${php.selectedVersion})`);
      }
      error = "";
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    } finally {
      isBusy = false;
    }
  }

  async function selectNodeVersion(version: string) {
    node = { ...node, selectedVersion: version };
    if (!isNativeApp || !version) return;
    isBusy = true;
    try {
      await setActiveVersion(RuntimeService.Node, version);
      onStatus(`Node.js activo: v${version}`);
      error = "";
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    } finally {
      isBusy = false;
    }
  }

  function start() {
    if (intervalId !== undefined) return;
    intervalId = window.setInterval(
      () => void refreshPhpStatus(),
      RuntimeTiming.quickTrayStatusRefreshMs,
    );
  }

  function dispose() {
    if (intervalId === undefined) return;
    window.clearInterval(intervalId);
    intervalId = undefined;
  }

  return {
    get php() {
      return php;
    },
    get node() {
      return node;
    },
    get apache() {
      return apache;
    },
    get isPhpRunning() {
      return isPhpRunning;
    },
    get isBusy() {
      return isBusy;
    },
    get error() {
      return error;
    },
    load,
    refreshPhpStatus,
    selectPhpVersion,
    togglePhp,
    selectNodeVersion,
    start,
    dispose,
  };
}
