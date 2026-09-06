<script lang="ts">
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import {
    FolderOpen,
    MonitorCog,
    Network,
    RotateCcw,
    Save,
    Settings2,
  } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import { onMount } from "svelte";
  import { i18n, persistLocale, t, type Locale } from "$lib/i18n";

  const isNativeApp = isTauri();
  const defaultSettings = {
    locale: "es" as Locale,
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

  let settings = $state({ ...defaultSettings });
  let saved = $state(false);
  let workspacePath = $state("C:\\Harbor");
  let error = $state("");

  type ProxySettings = {
    enabled: boolean;
    host: string;
    port: number;
    username: string | null;
    password: string | null;
  };

  let proxy = $state<ProxySettings>({
    enabled: false,
    host: "",
    port: 8080,
    username: null,
    password: null,
  });

  onMount(() => {
    const stored = localStorage.getItem("harbor-settings");
    if (stored) {
      try {
        settings = { ...defaultSettings, ...JSON.parse(stored) };
        persistLocale(settings.locale);
      } catch {
        localStorage.removeItem("harbor-settings");
      }
    }
    if (isNativeApp) {
      void loadProxySettings();
    }
  });

  async function loadProxySettings() {
    try {
      proxy = await invoke<ProxySettings>("get_proxy_settings");
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    }
  }

  function saveSettings() {
    const port = Number(settings.proxyPort);
    if (settings.proxyEnabled) {
      if (!settings.proxyHost.trim()) {
        error = t("settings.errors.proxyHostRequired");
        return;
      }
      if (!Number.isInteger(port) || port < 1 || port > 65535) {
        error = t("settings.errors.proxyPortInvalid");
        return;
      }
    }

    error = "";
    proxy = {
      enabled: settings.proxyEnabled,
      host: settings.proxyHost.trim(),
      port,
      username: settings.proxyUsername.trim() || null,
      password: settings.proxyPassword || null,
    };
    if (isNativeApp) {
      void invoke("save_proxy_settings", { settings: proxy }).catch((caught) => {
        error = caught instanceof Error ? caught.message : String(caught);
      });
    }
    localStorage.setItem("harbor-settings", JSON.stringify(settings));
    saved = true;
    window.setTimeout(() => (saved = false), 1800);
  }

  function resetSettings() {
    settings = { ...defaultSettings };
    persistLocale(settings.locale);
    saveSettings();
  }

  function changeLocale(event: Event) {
    const nextLocale = (event.currentTarget as HTMLSelectElement).value as Locale;
    settings.locale = nextLocale;
    persistLocale(nextLocale);
  }

  async function openWorkspace() {
    error = "";
    try {
      if (isNativeApp) {
        await invoke("open_directory", { path: workspacePath });
      }
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    }
  }
</script>

<svelte:head>
  <title>Harbor | {t("settings.title")}</title>
  <meta
    name="description"
    content={t("settings.description")}
  />
</svelte:head>

<main class="settings-page" aria-labelledby="settings-title">
  <header class="page-header">
    <div>
      <p class="eyebrow">{t("settings.eyebrow")}</p>
      <h1 id="settings-title">{t("settings.title")}</h1>
      <p class="page-description">
        {t("settings.description")}
      </p>
    </div>
    <div class="header-actions">
      <Button.Root
        class="secondary-button"
        type="button"
        onclick={resetSettings}
      >
        <RotateCcw size={15} aria-hidden="true" />
        <span>{t("settings.reset")}</span>
      </Button.Root>
      <Button.Root class="primary-button" type="button" onclick={saveSettings}>
        <Save size={15} aria-hidden="true" />
        <span>{saved ? t("settings.saved") : t("settings.save")}</span>
      </Button.Root>
    </div>
  </header>

  {#if error}<p class="error" role="alert">{error}</p>{/if}

  <div class="settings-grid">
    <section class="settings-section" aria-labelledby="behavior-title">
      <div class="section-heading">
        <span class="section-icon"
          ><MonitorCog size={18} aria-hidden="true" /></span
        >
        <div>
          <h2 id="behavior-title">{t("settings.behavior")}</h2>
          <p>{t("settings.behaviorDescription")}</p>
        </div>
      </div>
      <div class="settings-list">
        <label class="setting-row">
          <span
            ><strong>{t("settings.launchAtStartup")}</strong><small
              >{t("settings.launchAtStartupDescription")}</small
            ></span
          >
          <input
            type="checkbox"
            bind:checked={settings.launchAtStartup}
            aria-label={t("settings.launchAtStartup")}
          />
        </label>
        <label class="setting-row">
          <span
            ><strong>{t("settings.minimizeToTray")}</strong><small
              >{t("settings.minimizeToTrayDescription")}</small
            ></span
          >
          <input
            type="checkbox"
            bind:checked={settings.minimizeToTray}
            aria-label={t("settings.minimizeToTray")}
          />
        </label>
        <label class="setting-row">
          <span
            ><strong>{t("settings.closeToTray")}</strong><small
              >{t("settings.closeToTrayDescription")}</small
            ></span
          >
          <input
            type="checkbox"
            bind:checked={settings.closeToTray}
            aria-label={t("settings.closeToTray")}
          />
        </label>
        <label class="setting-row">
          <span
            ><strong>{t("settings.restoreLastSection")}</strong><small
              >{t("settings.restoreLastSectionDescription")}</small
            ></span
          >
          <input
            type="checkbox"
            bind:checked={settings.openLastSection}
            aria-label={t("settings.restoreLastSection")}
          />
        </label>
      </div>
    </section>

    <section class="settings-section" aria-labelledby="language-title">
      <div class="section-heading">
        <span class="section-icon"><Settings2 size={18} aria-hidden="true" /></span>
        <div>
          <h2 id="language-title">{t("settings.language")}</h2>
          <p>{t("settings.languageDescription")}</p>
        </div>
      </div>
      <div class="workspace-control">
        <label for="language-select">{t("settings.language")}</label>
        <select id="language-select" value={i18n.locale} onchange={changeLocale}>
          <option value="es">{t("settings.spanish")}</option>
          <option value="en">{t("settings.english")}</option>
        </select>
      </div>
    </section>

    <section class="settings-section" aria-labelledby="workspace-title">
      <div class="section-heading">
        <span class="section-icon"
          ><Settings2 size={18} aria-hidden="true" /></span
        >
        <div>
          <h2 id="workspace-title">{t("settings.workspace")}</h2>
          <p>{t("settings.workspaceDescription")}</p>
        </div>
      </div>
      <div class="workspace-control">
        <label for="workspace-path">{t("settings.workspaceLabel")}</label>
        <div class="path-row">
          <input id="workspace-path" value={workspacePath} readonly />
          <Button.Root
            class="icon-button"
            type="button"
            onclick={openWorkspace}
            title={t("settings.openWorkspace")}
            aria-label={t("settings.openWorkspace")}
          >
            <FolderOpen size={16} aria-hidden="true" />
          </Button.Root>
        </div>
        <p class="hint">
          {t("settings.workspaceHint")}
        </p>
      </div>
    </section>

    <section class="settings-section proxy-section" aria-labelledby="proxy-title">
      <div class="section-heading">
        <span class="section-icon"><Network size={18} aria-hidden="true" /></span>
        <div>
          <h2 id="proxy-title">{t("settings.proxy")}</h2>
          <p>{t("settings.proxyDescription")}</p>
        </div>
        <label class="switch-label">
          <input type="checkbox" bind:checked={settings.proxyEnabled} aria-label={t("settings.proxy")} />
          <span>{settings.proxyEnabled ? t("settings.enabled") : t("settings.disabled")}</span>
        </label>
      </div>
      {#if settings.proxyEnabled}
        <div class="proxy-form">
          <div class="form-row">
            <label for="proxy-host">{t("common.host")}</label>
            <input id="proxy-host" bind:value={settings.proxyHost} placeholder="proxy.example.com" autocomplete="off" />
          </div>
          <div class="form-row port-field">
            <label for="proxy-port">{t("common.port")}</label>
            <input id="proxy-port" type="number" min="1" max="65535" bind:value={settings.proxyPort} placeholder="8080" inputmode="numeric" />
          </div>
          <div class="form-row">
            <label for="proxy-username">{t("common.username")} <span>({t("settings.optional")})</span></label>
            <input id="proxy-username" bind:value={settings.proxyUsername} autocomplete="username" />
          </div>
          <div class="form-row">
            <label for="proxy-password">{t("common.password")} <span>({t("settings.optional")})</span></label>
            <input id="proxy-password" type="password" bind:value={settings.proxyPassword} autocomplete="current-password" />
          </div>
          <p class="hint">{t("settings.proxyHint")}</p>
        </div>
      {:else}
        <p class="proxy-disabled">{t("settings.directConnection")}</p>
      {/if}
    </section>
  </div>
</main>

<style>
  .settings-page {
    box-sizing: border-box;
    display: flex;
    flex: 1;
    flex-direction: column;
    margin: 0 auto;
    max-width: 1080px;
    padding: 32px;
    width: 100%;
  }
  .page-header,
  .header-actions,
  .section-heading,
  .path-row {
    align-items: center;
    display: flex;
  }
  .page-header {
    justify-content: space-between;
  }
  .header-actions {
    gap: 8px;
  }
  .eyebrow {
    color: var(--color-east-bay-700);
    font-size: 11px;
    font-weight: 750;
    letter-spacing: 0.06em;
    margin: 0 0 8px;
    text-transform: uppercase;
  }
  h1 {
    color: var(--color-boulder-950);
    font-size: 30px;
    letter-spacing: 0;
    line-height: 1;
    margin: 0;
  }
  .page-description {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.5;
    margin: 10px 0 0;
  }
  :global(.primary-button),
  :global(.secondary-button) {
    align-items: center;
    border: 0;
    border-radius: 6px;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 700;
    gap: 7px;
    height: 36px;
    padding: 0 12px;
  }
  :global(.primary-button) {
    background: var(--color-east-bay-900);
    color: #fff;
  }
  :global(.secondary-button) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
  }
  .settings-grid {
    display: grid;
    gap: 16px;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    margin-top: 32px;
  }
  .settings-section {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    min-width: 0;
  }
  .proxy-section { grid-column: 1 / -1; }
  .section-heading {
    border-bottom: 1px solid var(--color-boulder-100);
    gap: 11px;
    padding: 18px;
  }
  .switch-label { align-items: center; color: var(--color-boulder-500); display: inline-flex; font-size: 11px; font-weight: 650; gap: 7px; margin-left: auto; }
  .section-icon {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-100);
    border-radius: 6px;
    color: var(--color-east-bay-700);
    display: flex;
    height: 34px;
    justify-content: center;
    width: 34px;
  }
  h2 {
    color: var(--color-boulder-900);
    font-size: 14px;
    margin: 0;
  }
  .section-heading p,
  .hint {
    color: var(--color-boulder-500);
    font-size: 12px;
    line-height: 1.45;
    margin: 4px 0 0;
  }
  .settings-list {
    padding: 4px 18px;
  }
  .setting-row {
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
    cursor: pointer;
    display: flex;
    gap: 16px;
    justify-content: space-between;
    padding: 15px 0;
  }
  .setting-row:last-child {
    border-bottom: 0;
  }
  .setting-row span {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }
  .setting-row strong {
    color: var(--color-boulder-800);
    font-size: 13px;
    font-weight: 650;
  }
  .setting-row small {
    color: var(--color-boulder-500);
    font-size: 11px;
    line-height: 1.4;
  }
  input[type="checkbox"] {
    accent-color: var(--color-east-bay-700);
    cursor: pointer;
    flex: 0 0 auto;
    height: 16px;
    width: 16px;
  }
  .workspace-control {
    padding: 18px;
  }
  .proxy-form { display: grid; gap: 15px 16px; grid-template-columns: minmax(0, 1fr) 150px; padding: 18px; }
  .form-row { display: flex; flex-direction: column; gap: 7px; }
  .form-row label { color: var(--color-boulder-700); font-size: 12px; font-weight: 650; }
  .form-row label span { color: var(--color-boulder-500); font-weight: 500; }
  .form-row input { background: #fff; border: 1px solid var(--color-boulder-200); border-radius: 6px; box-sizing: border-box; color: var(--color-boulder-800); font: inherit; font-size: 12px; height: 36px; padding: 0 10px; width: 100%; }
  .form-row input:focus { border-color: var(--color-east-bay-400); outline: 2px solid var(--color-east-bay-100); }
  .proxy-form .hint { grid-column: 1 / -1; margin-top: -2px; }
  .proxy-disabled { color: var(--color-boulder-500); font-size: 12px; margin: 0; padding: 18px; }
  .workspace-control label {
    color: var(--color-boulder-700);
    display: block;
    font-size: 12px;
    font-weight: 650;
    margin-bottom: 7px;
  }
  .path-row {
    gap: 8px;
  }
  .path-row input {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-700);
    flex: 1;
    font: inherit;
    font-size: 12px;
    height: 36px;
    min-width: 0;
    padding: 0 10px;
  }
  :global(.icon-button) {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: inline-flex;
    height: 36px;
    justify-content: center;
    width: 36px;
  }
  .error {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 6px;
    color: #b91c1c;
    font-size: 12px;
    margin: 20px 0 0;
    padding: 10px 12px;
  }
  @media (max-width: 760px) {
    .settings-page {
      padding: 24px 16px;
    }
    .page-header {
      align-items: flex-start;
      flex-direction: column;
      gap: 18px;
    }
    .settings-grid {
      grid-template-columns: 1fr;
      margin-top: 24px;
    }
    .proxy-section { grid-column: auto; }
    .proxy-form { grid-template-columns: 1fr; }
    .proxy-form .hint { grid-column: auto; }
    .header-actions {
      width: 100%;
    }
    :global(.header-actions button) {
      flex: 1;
      justify-content: center;
    }
  }
</style>
