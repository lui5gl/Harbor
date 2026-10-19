<script lang="ts">
  import { Play, Square } from "@lucide/svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import DeleteRuntimeDialog from "./DeleteRuntimeDialog.svelte";
  import InstallRuntimeDialog from "./InstallRuntimeDialog.svelte";
  import RuntimeVersionSelector from "./RuntimeVersionSelector.svelte";
  import { cleanRuntimeVersion } from "../application/runtimeVersions";
  type ServiceCardProps = {
    serviceName: string;
    serviceDescription: string;
    serviceIconPath: string;
    versions: string[];
    installedVersions: string[];
    onInstall: (version: string) => Promise<void>;
    onRemove: (version: string) => Promise<void>;
    onStart: (version: string) => Promise<void>;
    onStop: () => Promise<void>;
    getStatus: () => Promise<boolean>;
    onVersionSelect: (version: string) => Promise<void>;
  };
  let {
    serviceName,
    serviceDescription,
    serviceIconPath,
    versions,
    installedVersions,
    onInstall,
    onRemove,
    onStart,
    onStop,
    getStatus,
    onVersionSelect,
  }: ServiceCardProps = $props();
  let serviceTitleId = $derived(`${serviceName.toLowerCase().replaceAll(" ", "-")}-service-title`);
  let selectedVersion = $state("");
  let installingVersion = $state("");
  let downloadProgress = $state(0);
  let pendingVersion = $state("");
  let pendingRemovalVersion = $state("");
  let isRemoving = $state(false);
  let downloadError = $state("");
  let isRunning = $state(false);
  let canStartStop = $derived(serviceName === "PHP" || serviceName === "Apache");
  let serviceRole = $derived(
    serviceName === "PHP"
      ? "FastCGI / CLI"
      : serviceName === "Apache"
        ? "Web server"
        : serviceDescription || "Runtime",
  );
  let servicePort = $derived(
    serviceName === "PHP" ? "FastCGI :9070" : serviceName === "Apache" ? "HTTP :8080" : "CLI",
  );
  let installedCatalog = $derived(
    versions.filter((version) => installedVersions.includes(getVersionNumber(version))),
  );
  let downloadableCatalog = $derived(
    versions.filter((version) => !installedVersions.includes(getVersionNumber(version))),
  );
  $effect(() => {
    if (!installedCatalog.includes(selectedVersion)) selectedVersion = installedCatalog[0] ?? "";
  });
  function getVersionNumber(version: string) {
    return cleanRuntimeVersion(version);
  }
  function isInstalled(version: string) {
    return installedVersions.includes(getVersionNumber(version));
  }
  onMount(() => {
    if (canStartStop) void getStatus().then((status) => (isRunning = status));
    let unlisten: (() => void) | undefined;
    void listen<{ service: string; version: string; progress: number }>(
      "runtime-download-progress",
      (event) => {
        if (
          event.payload.service === serviceName &&
          event.payload.version === getVersionNumber(installingVersion)
        )
          downloadProgress = event.payload.progress;
      },
    ).then((cleanup) => (unlisten = cleanup));
    return () => unlisten?.();
  });
  async function installVersion(version: string) {
    installingVersion = getVersionNumber(version);
    downloadProgress = 0;
    downloadError = "";
    try {
      await onInstall(version);
    } catch (error) {
      downloadError = error instanceof Error ? error.message : String(error);
    } finally {
      installingVersion = "";
      downloadProgress = 0;
    }
  }
  function requestInstall(version: string) {
    downloadError = "";
    pendingVersion = version;
  }
  function requestRemoval(version: string) {
    downloadError = "";
    pendingRemovalVersion = version;
  }
  async function removeVersion() {
    isRemoving = true;
    try {
      await onRemove(pendingRemovalVersion);
      if (selectedVersion === pendingRemovalVersion) selectedVersion = "";
      pendingRemovalVersion = "";
    } catch (error) {
      downloadError = error instanceof Error ? error.message : String(error);
    } finally {
      isRemoving = false;
    }
  }
  async function toggleService() {
    if (!canStartStop || !isInstalled(selectedVersion)) return;
    downloadError = "";
    try {
      if (isRunning) {
        await onStop();
        isRunning = false;
        return;
      }
      await onStart(selectedVersion);
      isRunning = true;
    } catch (error) {
      downloadError = error instanceof Error ? error.message : String(error);
    }
  }
  async function handleVersionSelect(version: string) {
    selectedVersion = version;
    try {
      await onVersionSelect(version);
      if (isRunning && serviceName === "PHP") await onStart(version);
    } catch (error) {
      downloadError = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<article
  class="grid min-h-23 grid-cols-[minmax(220px,1fr)_minmax(260px,320px)_minmax(320px,435px)] items-center gap-7 rounded-xl border border-boulder-200 bg-white px-7 py-4.5 max-xl:grid-cols-[minmax(200px,1fr)_minmax(300px,435px)] max-md:grid-cols-1 max-md:gap-4 max-md:p-4"
  aria-labelledby={serviceTitleId}
>
  <div class="flex min-w-0 items-center gap-5">
    <div
      class="flex size-12 shrink-0 items-center justify-center rounded-[10px] border border-east-bay-100 bg-east-bay-50 text-east-bay-500"
    >
      <svg class="size-6 fill-current" viewBox="0 0 24 24" role="img"
        ><path d={serviceIconPath} /></svg
      >
    </div>
    <div class="min-w-0">
      <h2 id={serviceTitleId} class="text-[17px] leading-tight font-semibold text-boulder-950">
        {serviceName}
      </h2>
      <p class="mt-1 text-sm leading-snug text-east-bay-800">{serviceRole}</p>
    </div>
  </div>
  <div
    class="grid min-w-0 grid-cols-3 items-center gap-4 max-xl:col-start-1 max-xl:ml-17 max-md:col-auto max-md:ml-0"
  >
    <div class="flex min-w-0 flex-col gap-1">
      <span class="text-[11px] uppercase text-boulder-600">Version</span><span
        class="truncate text-[13px] font-semibold text-boulder-950">{selectedVersion || "-"}</span
      >
    </div>
    <div class="flex min-w-0 flex-col gap-1">
      <span class="text-[11px] uppercase text-boulder-600">Port</span><span
        class="truncate text-[13px] font-semibold text-boulder-950">{servicePort}</span
      >
    </div>
    {#if serviceName === "PHP"}<div
        class={`flex items-center gap-1.5 whitespace-nowrap text-xs ${isInstalled(selectedVersion) ? "text-east-bay-700" : "text-boulder-600"}`}
      >
        <span class="size-1.5 rounded-full bg-current"></span><span
          >{isInstalled(selectedVersion) ? "Ready" : "Not installed"}</span
        >
      </div>{/if}
  </div>
  <div
    class="flex w-full max-w-108.75 items-center justify-self-end max-xl:col-start-2 max-xl:row-start-1 max-md:col-auto max-md:row-auto max-md:max-w-none"
  >
    <RuntimeVersionSelector
      {serviceName}
      installedVersions={installedCatalog}
      downloadableVersions={downloadableCatalog}
      bind:selectedVersion
      {installingVersion}
      {downloadProgress}
      onSelect={(version) => void handleVersionSelect(version)}
      onRequestInstall={requestInstall}
      onRequestRemoval={requestRemoval}
    />{#if canStartStop}<button
        class={`ml-1 inline-flex size-10 shrink-0 cursor-pointer items-center justify-center rounded-lg border transition disabled:cursor-not-allowed disabled:opacity-40 ${isRunning ? "border-red-200 bg-red-50 text-red-700 hover:bg-red-100" : "border-boulder-200 bg-boulder-50 text-east-bay-800 hover:border-east-bay-200 hover:bg-east-bay-50"}`}
        type="button"
        aria-label={`${isRunning ? "Stop" : "Start"} ${serviceName}`}
        disabled={!isInstalled(selectedVersion)}
        onclick={() => void toggleService()}
        >{#if isRunning}<Square size={16} />{:else}<Play size={18} />{/if}</button
      >{/if}
  </div>
</article>

<InstallRuntimeDialog
  open={Boolean(pendingVersion)}
  {serviceName}
  version={pendingVersion}
  onOpenChange={(open) => {
    if (!open) pendingVersion = "";
  }}
  onConfirm={() => {
    const version = pendingVersion;
    pendingVersion = "";
    void installVersion(version);
  }}
/>
<DeleteRuntimeDialog
  open={Boolean(pendingRemovalVersion)}
  serviceLabel={serviceName}
  version={getVersionNumber(pendingRemovalVersion)}
  onOpenChange={(open) => {
    if (!open && !isRemoving) pendingRemovalVersion = "";
  }}
  onConfirm={() => void removeVersion()}
/>
{#if downloadError}<div
    class="mt-2 rounded-md border border-red-200 bg-red-50 px-3 py-2 text-[13px] text-red-800"
    role="alert"
  >
    {downloadError}
  </div>{/if}
