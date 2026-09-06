<script lang="ts">
  import { RefreshCw } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import { onMount } from "svelte";
  import { createRuntimesStore } from "$lib/features/services/application/runtimesStore.svelte";
  import NodeRuntimeEditor from "$lib/features/services/components/NodeRuntimeEditor.svelte";
  import PhpWebStackEditor from "$lib/features/services/components/PhpWebStackEditor.svelte";
  import ServicesPanel from "$lib/features/services/components/ServicesPanel.svelte";
  import type { ServiceId } from "$lib/features/services/types";
  import { t } from "$lib/i18n";

  const runtimes = createRuntimesStore();
  let selectedServiceId = $state<ServiceId>("php-web");
  const fastCgiAddress = "127.0.0.1:9070";

  onMount(() => {
    runtimes.start();
    void runtimes.load();
    return () => runtimes.dispose();
  });

  const activePhpVersion = $derived(runtimes.php.active);
  const installedPhpVersions = $derived(runtimes.php.installed);
  const availablePhpVersions = $derived(runtimes.php.available);
  const activeApacheVersion = $derived(runtimes.apache.active);
  const installedApacheVersions = $derived(runtimes.apache.installed);
  const availableApacheVersions = $derived(runtimes.apache.available);
  const activeNodeVersion = $derived(runtimes.node.active);
  const installedNodeVersions = $derived(runtimes.node.installed);
  const availableNodeVersions = $derived(runtimes.node.available);
  const isPhpRunning = $derived(runtimes.isPhpRunning);
  const isCatalogLoading = $derived(runtimes.isCatalogLoading);
  const catalogError = $derived(runtimes.catalogError);
  const isInstalling = $derived(runtimes.isInstalling);
  const installProgress = $derived(runtimes.installProgress);
  const installingService = $derived(runtimes.installingService);
  const installingVersion = $derived(runtimes.installingVersion);
  const installError = $derived(runtimes.installError);

  const loadAllCatalogs = runtimes.refresh;
  const handleSelectPhpVersion = runtimes.selectPhpVersion;
  const handleTogglePhpFastCgi = runtimes.togglePhp;
  const handleSelectNodeVersion = runtimes.selectNodeVersion;
  const handleInstallVersion = runtimes.install;
  const handleDeleteVersion = runtimes.remove;
</script>

<svelte:head>
  <title>Harbor | {t("services.title")}</title>
  <meta name="description" content={t("services.description")} />
</svelte:head>

<main class="services-page" aria-labelledby="services-title">
  <header class="page-header">
    <div>
      <p class="eyebrow">{t("services.eyebrow")}</p>
      <h1 id="services-title">{t("services.title")}</h1>
      <p class="page-description">
        {t("services.description")}
      </p>
    </div>
    <Button.Root
      class="secondary-button"
      type="button"
      onclick={() => void loadAllCatalogs()}
      disabled={isCatalogLoading}
    >
      <RefreshCw
        size={15}
        strokeWidth={2}
        class={isCatalogLoading ? "spin" : ""}
        aria-hidden="true"
      />
      <span>{isCatalogLoading ? t("services.refreshing") : t("services.refreshCatalog")}</span>
    </Button.Root>
  </header>

  {#if catalogError}
    <p class="catalog-error" role="alert">{catalogError}</p>
  {/if}

  {#if isCatalogLoading && installedPhpVersions.length === 0 && installedNodeVersions.length === 0}
    <div class="loading-state" role="status">{t("services.loading")}</div>
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
    transition:
      background-color 150ms ease,
      border-color 150ms ease;
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
