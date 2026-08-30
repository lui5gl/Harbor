<script lang="ts">
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { RefreshCw } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import { onMount } from "svelte";
  import NodeRuntimeEditor from "$lib/features/services/NodeRuntimeEditor.svelte";
  import PhpWebStackEditor from "$lib/features/services/PhpWebStackEditor.svelte";
  import ServicesPanel from "$lib/features/services/ServicesPanel.svelte";
  import { cleanVersion, type ServiceId } from "$lib/features/services/types";

  const isNativeApp = isTauri();

  const mockPhpCatalog = [
    "8.4.4 (Active)",
    "8.3.17 (Active)",
    "8.2.27 (Security)",
    "8.1.31 (EOL)",
    "7.4.33 (EOL)"
  ];
  const mockApacheCatalog = [
    "2.4.62 (Active)",
    "2.4.61 (Active)",
    "2.4.58 (Active)"
  ];
  const mockNodeCatalog = [
    "22.14.0 (LTS - Jod)",
    "23.8.0 (Current)",
    "20.18.3 (LTS - Iron)",
    "18.20.7 (EOL)"
  ];

  let selectedServiceId = $state<ServiceId>("php-web");

  // PHP State
  let activePhpVersion = $state<string | null>(null);
  let installedPhpVersions = $state<string[]>([]);
  let availablePhpVersions = $state<string[]>([]);
  let isPhpRunning = $state(false);
  const fastCgiAddress = "127.0.0.1:9070";

  // Apache State
  let activeApacheVersion = $state<string | null>(null);
  let installedApacheVersions = $state<string[]>([]);
  let availableApacheVersions = $state<string[]>([]);

  // Node.js State
  let activeNodeVersion = $state<string | null>(null);
  let installedNodeVersions = $state<string[]>([]);
  let availableNodeVersions = $state<string[]>([]);

  // Async & Operations State
  let isCatalogLoading = $state(true);
  let catalogError = $state("");
  let isInstalling = $state(false);
  let installProgress = $state(0);
  let installingService = $state("");
  let installingVersion = $state("");
  let installError = $state("");

  onMount(() => {
    void initializeAndLoad();

    let unlistenProgress: (() => void) | undefined;
    if (isNativeApp) {
      listen<{ service: string; version: string; progress: number }>(
        "runtime-download-progress",
        (event) => {
          if (event.payload.service === installingService) {
            installProgress = event.payload.progress;
          }
        }
      ).then((cleanup) => {
        unlistenProgress = cleanup;
      });
    }

    return () => {
      if (unlistenProgress) unlistenProgress();
    };
  });

  async function initializeAndLoad() {
    if (!isNativeApp) {
      availablePhpVersions = mockPhpCatalog;
      installedPhpVersions = ["8.3.17"];
      activePhpVersion = "8.3.17";

      availableApacheVersions = mockApacheCatalog;
      installedApacheVersions = ["2.4.62"];
      activeApacheVersion = "2.4.62";

      availableNodeVersions = mockNodeCatalog;
      installedNodeVersions = ["22.14.0"];
      activeNodeVersion = "22.14.0";

      isCatalogLoading = false;
      return;
    }

    try {
      await invoke<string>("initialize_harbor_workspace");
    } catch (err) {
      catalogError = err instanceof Error ? err.message : String(err);
    }

    await loadAllCatalogs();
    await updatePhpStatus();
  }

  async function loadAllCatalogs() {
    if (!isNativeApp) return;

    isCatalogLoading = true;
    catalogError = "";

    let savedRuntimes: { php?: string; nodejs?: string; apache?: string } = {};
    try {
      savedRuntimes = await invoke("get_active_runtimes");
    } catch {
      // Config not initialized yet
    }

    const catalogRequests = [
      ["PHP", "get_php_versions"],
      ["Apache", "get_apache_versions"],
      ["Node.js", "get_node_versions"]
    ] as const;

    const results = await Promise.allSettled(
      catalogRequests.map(async ([serviceName, command]) => [
        serviceName,
        await invoke<string[]>(command),
        await invoke<string[]>("get_installed_versions", { service: serviceName })
      ] as const)
    );

    const failures: string[] = [];

    for (const result of results) {
      if (result.status === "fulfilled") {
        const [serviceName, catalog, installed] = result.value;
        if (serviceName === "PHP") {
          availablePhpVersions = catalog;
          installedPhpVersions = installed;
          const target = savedRuntimes.php && installed.includes(savedRuntimes.php)
            ? savedRuntimes.php
            : installed[0];
          if (target) {
            activePhpVersion = target;
            void invoke("set_active_php_version", { version: cleanVersion(target) });
          }
        } else if (serviceName === "Apache") {
          availableApacheVersions = catalog;
          installedApacheVersions = installed;
          const target = savedRuntimes.apache && installed.includes(savedRuntimes.apache)
            ? savedRuntimes.apache
            : installed[0];
          if (target) {
            activeApacheVersion = target;
            void invoke("set_active_apache_version", { version: cleanVersion(target) });
          }
        } else if (serviceName === "Node.js") {
          availableNodeVersions = catalog;
          installedNodeVersions = installed;
          const target = savedRuntimes.nodejs && installed.includes(savedRuntimes.nodejs)
            ? savedRuntimes.nodejs
            : installed[0];
          if (target) {
            activeNodeVersion = target;
            void invoke("set_active_node_version", { version: cleanVersion(target) });
          }
        }
      } else {
        failures.push(result.reason instanceof Error ? result.reason.message : String(result.reason));
      }
    }

    if (failures.length > 0) {
      catalogError = `Some runtime catalogs could not be loaded: ${failures.join(" | ")}`;
    }

    isCatalogLoading = false;
  }

  async function updatePhpStatus() {
    if (!isNativeApp) return;
    try {
      isPhpRunning = await invoke<boolean>("get_php_status");
    } catch {
      isPhpRunning = false;
    }
  }

  // PHP Actions
  async function handleSelectPhpVersion(version: string) {
    const clean = cleanVersion(version);
    activePhpVersion = clean;
    if (!isNativeApp) return;

    try {
      await invoke<string>("set_active_php_version", { version: clean });
      if (isPhpRunning) {
        await invoke("start_php", { version: clean });
      }
    } catch (err) {
      catalogError = err instanceof Error ? err.message : String(err);
    }
  }

  async function handleTogglePhpFastCgi() {
    if (!activePhpVersion) return;
    const clean = cleanVersion(activePhpVersion);

    if (!isNativeApp) {
      isPhpRunning = !isPhpRunning;
      return;
    }

    try {
      if (isPhpRunning) {
        await invoke("stop_php");
        isPhpRunning = false;
      } else {
        await invoke<string>("start_php", { version: clean });
        isPhpRunning = true;
      }
    } catch (err) {
      catalogError = err instanceof Error ? err.message : String(err);
      await updatePhpStatus();
    }
  }

  // Node.js Actions
  async function handleSelectNodeVersion(version: string) {
    const clean = cleanVersion(version);
    activeNodeVersion = clean;
    if (!isNativeApp) return;

    try {
      await invoke<string>("set_active_node_version", { version: clean });
    } catch (err) {
      catalogError = err instanceof Error ? err.message : String(err);
    }
  }

  // General Install / Delete Actions
  async function handleInstallVersion(service: "PHP" | "Apache" | "Node.js", rawVersion: string) {
    const clean = cleanVersion(rawVersion);
    isInstalling = true;
    installingService = service;
    installingVersion = clean;
    installProgress = 0;
    installError = "";

    if (!isNativeApp) {
      setTimeout(() => {
        if (service === "PHP") {
          installedPhpVersions = [...installedPhpVersions, clean];
          if (!activePhpVersion) activePhpVersion = clean;
        } else if (service === "Apache") {
          installedApacheVersions = [...installedApacheVersions, clean];
          if (!activeApacheVersion) activeApacheVersion = clean;
        } else if (service === "Node.js") {
          installedNodeVersions = [...installedNodeVersions, clean];
          if (!activeNodeVersion) activeNodeVersion = clean;
        }
        isInstalling = false;
        installingService = "";
        installingVersion = "";
      }, 500);
      return;
    }

    const commandMap = {
      PHP: "install_php",
      Apache: "install_apache",
      "Node.js": "install_node"
    } as const;

    try {
      await invoke<string>(commandMap[service], { version: clean });

      const updated = await invoke<string[]>("get_installed_versions", { service });
      if (service === "PHP") {
        installedPhpVersions = updated;
        if (!activePhpVersion) void handleSelectPhpVersion(clean);
      } else if (service === "Apache") {
        installedApacheVersions = updated;
        if (!activeApacheVersion) activeApacheVersion = clean;
      } else if (service === "Node.js") {
        installedNodeVersions = updated;
        if (!activeNodeVersion) void handleSelectNodeVersion(clean);
      }
    } catch (err) {
      installError = err instanceof Error ? err.message : String(err);
    } finally {
      isInstalling = false;
      installingService = "";
      installingVersion = "";
      installProgress = 0;
    }
  }

  async function handleDeleteVersion(service: "PHP" | "Apache" | "Node.js", cleanVer: string) {
    if (!isNativeApp) {
      if (service === "PHP") {
        installedPhpVersions = installedPhpVersions.filter((v) => cleanVersion(v) !== cleanVer);
        if (activePhpVersion === cleanVer) activePhpVersion = installedPhpVersions[0] ?? null;
      } else if (service === "Apache") {
        installedApacheVersions = installedApacheVersions.filter((v) => cleanVersion(v) !== cleanVer);
        if (activeApacheVersion === cleanVer) activeApacheVersion = installedApacheVersions[0] ?? null;
      } else if (service === "Node.js") {
        installedNodeVersions = installedNodeVersions.filter((v) => cleanVersion(v) !== cleanVer);
        if (activeNodeVersion === cleanVer) activeNodeVersion = installedNodeVersions[0] ?? null;
      }
      return;
    }

    try {
      if (service === "PHP" && isPhpRunning && activePhpVersion === cleanVer) {
        await invoke("stop_php");
        isPhpRunning = false;
      }

      await invoke("remove_runtime", { service, version: cleanVer });

      const updated = await invoke<string[]>("get_installed_versions", { service });
      if (service === "PHP") {
        installedPhpVersions = updated;
        if (activePhpVersion === cleanVer) {
          activePhpVersion = updated[0] ?? null;
          if (activePhpVersion) void handleSelectPhpVersion(activePhpVersion);
        }
      } else if (service === "Apache") {
        installedApacheVersions = updated;
        if (activeApacheVersion === cleanVer) {
          activeApacheVersion = updated[0] ?? null;
        }
      } else if (service === "Node.js") {
        installedNodeVersions = updated;
        if (activeNodeVersion === cleanVer) {
          activeNodeVersion = updated[0] ?? null;
          if (activeNodeVersion) void handleSelectNodeVersion(activeNodeVersion);
        }
      }
    } catch (err) {
      catalogError = err instanceof Error ? err.message : String(err);
    }
  }
</script>

<svelte:head>
  <title>Harbor | Services</title>
  <meta name="description" content="Manage local runtimes and development services in Harbor." />
</svelte:head>

<main class="services-page" aria-labelledby="services-title">
  <header class="page-header">
    <div>
      <p class="eyebrow">Local environment</p>
      <h1 id="services-title">Services & Runtimes</h1>
      <p class="page-description">
        Manage local runtimes, active versions, and the integrated PHP + Apache Web Stack.
      </p>
    </div>
    <Button.Root
      class="secondary-button"
      type="button"
      onclick={() => void loadAllCatalogs()}
      disabled={isCatalogLoading}
    >
      <RefreshCw size={15} strokeWidth={2} class={isCatalogLoading ? "spin" : ""} aria-hidden="true" />
      <span>{isCatalogLoading ? "Refreshing..." : "Refresh catalog"}</span>
    </Button.Root>
  </header>

  {#if catalogError}
    <p class="catalog-error" role="alert">{catalogError}</p>
  {/if}

  {#if isCatalogLoading && installedPhpVersions.length === 0 && installedNodeVersions.length === 0}
    <div class="loading-state" role="status">Loading services and runtime catalogs...</div>
  {:else}
    <div class="services-workspace">
      <ServicesPanel
        {selectedServiceId}
        {activePhpVersion}
        {isPhpRunning}
        {activeNodeVersion}
        installedPhpCount={installedPhpVersions.length}
        installedNodeCount={installedNodeVersions.length}
        onSelect={(id) => (selectedServiceId = id)}
      />

      {#if selectedServiceId === "php-web"}
        <PhpWebStackEditor
          {activePhpVersion}
          {installedPhpVersions}
          {availablePhpVersions}
          {activeApacheVersion}
          {installedApacheVersions}
          {availableApacheVersions}
          {isPhpRunning}
          {fastCgiAddress}
          {isInstalling}
          {installProgress}
          {installingService}
          {installingVersion}
          {installError}
          onSelectPhpVersion={handleSelectPhpVersion}
          onTogglePhpFastCgi={handleTogglePhpFastCgi}
          onInstallVersion={handleInstallVersion}
          onDeleteVersion={handleDeleteVersion}
        />
      {:else if selectedServiceId === "nodejs"}
        <NodeRuntimeEditor
          {activeNodeVersion}
          {installedNodeVersions}
          {availableNodeVersions}
          {isInstalling}
          {installProgress}
          {installingVersion}
          {installError}
          onSelectNodeVersion={handleSelectNodeVersion}
          onInstallNodeVersion={(v) => handleInstallVersion("Node.js", v)}
          onDeleteNodeVersion={(v) => handleDeleteVersion("Node.js", v)}
        />
      {/if}
    </div>
  {/if}
</main>

<style>
  .services-page {
    box-sizing: border-box;
    display: flex;
    flex: 1;
    flex-direction: column;
    margin: 0 auto;
    max-width: 1240px;
    padding: 32px;
    width: 100%;
  }

  .page-header {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }

  .eyebrow {
    color: var(--color-east-bay-700);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.08em;
    margin: 0 0 8px;
    text-transform: uppercase;
  }

  h1,
  p {
    margin: 0;
  }

  h1 {
    color: var(--color-boulder-950);
    font-size: 30px;
    font-weight: 650;
    line-height: 1.2;
  }

  .page-description {
    color: var(--color-boulder-600);
    font-size: 15px;
    line-height: 1.5;
    margin-top: 8px;
  }

  :global(.secondary-button) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 13px;
    font-weight: 600;
    gap: 8px;
    justify-content: center;
    min-height: 38px;
    padding: 0 14px;
    transition: background-color 150ms ease, border-color 150ms ease;
  }

  :global(.secondary-button:hover:not(:disabled)) {
    background: var(--color-boulder-50);
    border-color: var(--color-boulder-300);
    color: var(--color-boulder-950);
  }

  :global(.secondary-button:disabled) {
    cursor: wait;
    opacity: 0.6;
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .catalog-error {
    background: #fff3f1;
    border: 1px solid #efb5ad;
    border-radius: 6px;
    color: #913526;
    font-size: 13px;
    margin: 20px 0 0;
    padding: 10px 12px;
  }

  .loading-state {
    align-items: center;
    color: var(--color-boulder-600);
    display: flex;
    flex: 1;
    font-size: 14px;
    justify-content: center;
    min-height: 280px;
  }

  .services-workspace {
    align-items: start;
    display: grid;
    gap: 20px;
    grid-template-columns: minmax(230px, 280px) minmax(0, 1fr);
    margin-top: 30px;
  }

  @media (max-width: 840px) {
    .services-workspace {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 640px) {
    .services-page {
      padding: 24px 16px;
    }

    .page-header {
      align-items: flex-start;
      flex-direction: column;
      gap: 14px;
    }
  }
</style>
