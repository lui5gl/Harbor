<script lang="ts">
  import { RefreshCw } from "@lucide/svelte";
  import { onMount } from "svelte";
  import { createRuntimesStore } from "#lib/features/services/application/runtimesStore.svelte";
  import NodeRuntimeEditor from "#lib/features/services/components/NodeRuntimeEditor.svelte";
  import PhpWebStackEditor from "#lib/features/services/components/PhpWebStackEditor.svelte";
  import ServicesPanel from "#lib/features/services/components/ServicesPanel.svelte";
  import type { ServiceId } from "#lib/features/services/types";
  import { t } from "#lib/i18n";

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

<main
  class="mx-auto flex w-full max-w-310 flex-1 flex-col p-8 max-sm:px-4 max-sm:py-6"
  aria-labelledby="services-title"
>
  <header class="flex items-center justify-between gap-4 max-sm:flex-col max-sm:items-start">
    <div>
      <h1
        id="services-title"
        class="text-3xl leading-tight font-semibold tracking-[-0.02em] text-boulder-950"
      >
        {t("services.title")}
      </h1>
      <p class="mt-2 text-[15px] leading-relaxed text-boulder-600">
        {t("services.description")}
      </p>
    </div>
    <button
      class="inline-flex min-h-9.5 cursor-pointer items-center justify-center gap-2 rounded-md border border-boulder-200 bg-white px-3.5 text-[13px] font-semibold text-boulder-800 hover:border-boulder-300 hover:bg-boulder-50 disabled:cursor-wait disabled:opacity-60"
      type="button"
      onclick={() => void loadAllCatalogs()}
      disabled={isCatalogLoading}
    >
      <RefreshCw
        size={15}
        strokeWidth={2}
        class={isCatalogLoading ? "animate-spin" : ""}
        aria-hidden="true"
      />
      <span>{isCatalogLoading ? t("services.refreshing") : t("services.refreshCatalog")}</span>
    </button>
  </header>

  {#if catalogError}
    <p
      class="mt-5 rounded-md border border-red-200 bg-red-50 px-3 py-2.5 text-[13px] text-red-800"
      role="alert"
    >
      {catalogError}
    </p>
  {/if}

  {#if isCatalogLoading && installedPhpVersions.length === 0 && installedNodeVersions.length === 0}
    <div
      class="flex min-h-70 flex-1 items-center justify-center text-sm text-boulder-600"
      role="status"
    >
      {t("services.loading")}
    </div>
  {:else}
    <div
      class="mt-7.5 grid grid-cols-[minmax(230px,280px)_minmax(0,1fr)] items-start gap-5 max-lg:grid-cols-1"
    >
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
