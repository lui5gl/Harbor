import { invoke, isTauri } from "@tauri-apps/api/core";
import { persistLocale, type Locale } from "$lib/i18n";

const SETTINGS_STORAGE_KEY = "harbor-settings";
const WORKSPACE_PATH = "C:\\Harbor";
const SETTINGS_FEEDBACK_DURATION_MS = 1_800;

export type UserSettings = {
  locale: Locale;
  launchAtStartup: boolean;
  minimizeToTray: boolean;
  closeToTray: boolean;
  openLastSection: boolean;
  proxyEnabled: boolean;
  proxyHost: string;
  proxyPort: string;
  proxyUsername: string;
  proxyPassword: string;
};

export type ProxySettings = {
  enabled: boolean;
  host: string;
  port: number;
  username: string | null;
  password: string | null;
};

export const defaultSettings: UserSettings = {
  locale: "es",
  launchAtStartup: false,
  minimizeToTray: true,
  closeToTray: true,
  openLastSection: true,
  proxyEnabled: false,
  proxyHost: "",
  proxyPort: "",
  proxyUsername: "",
  proxyPassword: "",
};

export function createSettingsStore() {
  const nativeApp = isTauri();
  let settings = $state<UserSettings>({ ...defaultSettings });
  let proxy = $state<ProxySettings>({
    enabled: false,
    host: "",
    port: 8080,
    username: null,
    password: null,
  });
  let saved = $state(false);
  let error = $state("");

  function loadLocalSettings() {
    const stored = localStorage.getItem(SETTINGS_STORAGE_KEY);
    if (!stored) return;
    try {
      const parsed: Partial<UserSettings> = JSON.parse(stored);
      settings = { ...defaultSettings, ...parsed };
      persistLocale(settings.locale);
    } catch {
      localStorage.removeItem(SETTINGS_STORAGE_KEY);
    }
  }

  async function loadProxy() {
    if (!nativeApp) return;
    try {
      proxy = await invoke<ProxySettings>("get_proxy_settings");
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    }
  }

  function load() {
    loadLocalSettings();
    void loadProxy();
  }

  function setLocale(locale: Locale) {
    settings.locale = locale;
    persistLocale(locale);
  }

  async function save(nextProxy: ProxySettings) {
    error = "";
    proxy = nextProxy;
    if (nativeApp) await invoke("save_proxy_settings", { settings: nextProxy });
    localStorage.setItem(SETTINGS_STORAGE_KEY, JSON.stringify(settings));
    saved = true;
    window.setTimeout(() => (saved = false), SETTINGS_FEEDBACK_DURATION_MS);
  }

  function reset() {
    settings = { ...defaultSettings };
    persistLocale(settings.locale);
  }

  return {
    get settings() {
      return settings;
    },
    get proxy() {
      return proxy;
    },
    get saved() {
      return saved;
    },
    get error() {
      return error;
    },
    get workspacePath() {
      return WORKSPACE_PATH;
    },
    load,
    setLocale,
    save,
    reset,
    setError(value: string) {
      error = value;
    },
  };
}
