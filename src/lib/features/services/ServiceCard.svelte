<script lang="ts">
  import { ChevronDown, Check, Download, Play, Search, Square, Trash2 } from "@lucide/svelte";
  import { listen } from "@tauri-apps/api/event";
  import { Button, Combobox } from "bits-ui";
  import { onMount } from "svelte";
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
    onVersionSelect
  }: ServiceCardProps = $props();
  let serviceTitleId = $derived(`${serviceName.toLowerCase().replaceAll(" ", "-")}-service-title`);

  let selectedVersion = $state("");
  let searchValue = $state("");
  let downloadSearchValue = $state("");
  let downloadAnchor = $state<HTMLDivElement | null>(null);
  let isVersionMenuOpen = $state(false);
  let isDownloadMenuOpen = $state(false);
  let installingVersion = $state("");
  let downloadProgress = $state(0);
  let pendingVersion = $state("");
  let pendingRemovalVersion = $state("");
  let removalConfirmation = $state("");
  let isRemoving = $state(false);
  let downloadError = $state("");
  let isRunning = $state(false);
  let serviceRole = $derived(serviceName === "Apache" ? "Web server" : "Runtime");
  let servicePort = $derived(serviceName === "Apache" ? "HTTP :8080" : "CLI");
  $effect(() => {
    if (!installedCatalog.includes(selectedVersion)) {
      selectedVersion = installedCatalog[0] ?? "";
    }
  });
  let installedCatalog = $derived(versions.filter((version) => installedVersions.includes(getVersionNumber(version))));
  let downloadableCatalog = $derived(versions.filter((version) => !installedVersions.includes(getVersionNumber(version))));
  let filteredDownloadCatalog = $derived(downloadSearchValue === ""
    ? downloadableCatalog
    : downloadableCatalog.filter((version) => version.toLowerCase().includes(downloadSearchValue.toLowerCase())));

  function getVersionParts(version: string) {
    const match = version.match(/^(.*) \((.*)\)$/);
    if (!match) {
      return { number: version, channel: "" };
    }
    return { number: match[1], channel: match[2] };
  }

  function getVersionNumber(version: string) {
    return getVersionParts(version).number.trimStart().replace(/^v/, "");
  }

  function isInstalled(version: string) {
    return installedVersions.includes(getVersionNumber(version));
  }

  onMount(() => {
    if (serviceName === "Apache") {
      void getStatus().then((status) => (isRunning = status));
    }
    let unlisten: (() => void) | undefined;
    void listen<{ service: string; version: string; progress: number }>("runtime-download-progress", (event) => {
      if (event.payload.service === serviceName && event.payload.version === getVersionNumber(installingVersion)) {
        downloadProgress = event.payload.progress;
      }
    }).then((cleanup) => (unlisten = cleanup));
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
    removalConfirmation = "";
    pendingRemovalVersion = version;
  }

  async function removeVersion() {
    if (removalConfirmation !== "CONFIRMAR") {
      downloadError = "Escribe CONFIRMAR para eliminar esta versión";
      return;
    }
    isRemoving = true;
    try {
      await onRemove(pendingRemovalVersion);
      if (selectedVersion === pendingRemovalVersion) selectedVersion = "";
      pendingRemovalVersion = "";
      removalConfirmation = "";
    } catch (error) {
      downloadError = error instanceof Error ? error.message : String(error);
    } finally {
      isRemoving = false;
    }
  }

  async function toggleService() {
    if (serviceName !== "Apache" || !isInstalled(selectedVersion)) {
      return;
    }
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
    } catch (error) {
      downloadError = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<article class="service-card" aria-labelledby={serviceTitleId}>
  <div class="service-identity">
    <div class="service-icon" aria-hidden="true">
      <svg viewBox="0 0 24 24" role="img">
        <path d={serviceIconPath} />
      </svg>
    </div>
    <div class="service-copy">
      <h2 id={serviceTitleId}>{serviceName}</h2>
      <p>{serviceRole}</p>
    </div>
  </div>

  <div class="service-meta" aria-label={`${serviceName} details`}>
    <div class="meta-item">
      <span class="meta-label">Version</span>
      <span class="meta-value">{selectedVersion || "-"}</span>
    </div>
    <div class="meta-item">
      <span class="meta-label">Port</span>
      <span class="meta-value">{servicePort}</span>
    </div>
    {#if serviceName === "PHP"}
      <div class={`runtime-state${isInstalled(selectedVersion) ? " ready" : ""}`}>
        <span class="status-indicator" aria-hidden="true"></span>
        <span>{isInstalled(selectedVersion) ? "Ready" : "Not installed"}</span>
      </div>
    {/if}
  </div>

  <div class={`service-controls${serviceName === "Apache" ? "" : " runtime-controls"}`}>
    <div class="version-control-group" bind:this={downloadAnchor}>
    <Combobox.Root
      type="single"
      items={installedCatalog.map((version) => ({ value: version, label: version }))}
      bind:value={selectedVersion}
      onValueChange={(value) => value && void handleVersionSelect(value)}
      bind:open={isVersionMenuOpen}
      onOpenChangeComplete={(isOpen) => {
        if (!isOpen) searchValue = "";
      }}
    >
      <div class="version-anchor">
        <Combobox.Trigger class={`version-button${isVersionMenuOpen ? " version-button-open" : ""}`} aria-label={`Select installed ${serviceName} version`}>
          {#if selectedVersion}
            {@const selectedParts = getVersionParts(selectedVersion)}
            <span class="selected-version-label">
              <span>{selectedParts.number}</span>
              {#if selectedParts.channel}<span class="selected-version-channel">{selectedParts.channel.replace("LTS - ", "LTS · ")}</span>{/if}
            </span>
          {:else}
            <span>No version selected</span>
          {/if}
          <ChevronDown size={16} strokeWidth={2} aria-hidden="true" />
        </Combobox.Trigger>
      </div>

      <Combobox.Portal>
        <Combobox.Content class="version-content" customAnchor={downloadAnchor} sideOffset={0}>
          <div class="version-search-row">
            <Search class="version-search-icon" size={16} strokeWidth={2} aria-hidden="true" />
            <Combobox.Input
              class="version-search"
              oninput={(event) => (searchValue = event.currentTarget.value)}
              placeholder="Search versions"
              aria-label={`Search ${serviceName} versions`}
            />
          </div>
          <Combobox.Viewport>
            {#if installedCatalog.length > 0}
              {#each installedCatalog as version (version)}
                <Combobox.Item class="version-item" value={version} label={version}>
                  {#snippet children({ selected })}
                    {@const versionParts = getVersionParts(version)}
                    <span class="version-item-label">
                      <span>{versionParts.number}</span>
                      {#if versionParts.channel}
                        <span class={`version-channel ${versionParts.channel.startsWith("EOL") ? "version-channel-eol" : versionParts.channel.startsWith("LTS") ? "version-channel-lts" : versionParts.channel.startsWith("Security") ? "version-channel-security" : "version-channel-current"}`}>
                          {versionParts.channel.replace("LTS - ", "LTS · ")}
                        </span>
                      {/if}
                    </span>
                    {#if selected}
                      <Check size={16} strokeWidth={2} aria-hidden="true" />
                    {/if}
                    <button class="remove-version-button" type="button" aria-label={`Remove ${version}`} onclick={(event) => { event.stopPropagation(); requestRemoval(version); }}>
                      <Trash2 size={15} strokeWidth={2} aria-hidden="true" />
                    </button>
                  {/snippet}
                </Combobox.Item>
              {/each}
            {:else}
              <span class="version-empty">No installed versions</span>
            {/if}
          </Combobox.Viewport>
        </Combobox.Content>
      </Combobox.Portal>
    </Combobox.Root>

    <Combobox.Root
      type="single"
      items={downloadableCatalog.map((version) => ({ value: version, label: version }))}
      bind:open={isDownloadMenuOpen}
      onOpenChangeComplete={(isOpen) => {
        if (!isOpen) downloadSearchValue = "";
      }}
    >
      <div class="download-anchor">
        <Combobox.Trigger class="download-selected-button" aria-label={`Download ${serviceName} version`} disabled={Boolean(installingVersion)}>
          {#if installingVersion}
            <span class="download-progress" style={`--download-progress: ${downloadProgress * 3.6}deg`} aria-label={`${downloadProgress}% downloaded`}>
              <span>{downloadProgress}%</span>
            </span>
          {:else}
            <Download size={16} strokeWidth={2} aria-hidden="true" />
          {/if}
        </Combobox.Trigger>
      </div>
      <Combobox.Portal>
        <Combobox.Content class="version-content download-content" customAnchor={downloadAnchor} sideOffset={0}>
          <div class="version-search-row">
            <Search class="version-search-icon" size={16} strokeWidth={2} aria-hidden="true" />
            <Combobox.Input
              class="version-search"
              oninput={(event) => (downloadSearchValue = event.currentTarget.value)}
              placeholder="Search versions"
              aria-label={`Search downloadable ${serviceName} versions`}
            />
          </div>
          <Combobox.Viewport>
            {#each filteredDownloadCatalog as version (version)}
              <Combobox.Item class="version-item" value={version} label={version} onclick={() => requestInstall(version)}>
                {@const versionParts = getVersionParts(version)}
                <span class="version-item-label">
                  <span>{versionParts.number}</span>
                  {#if versionParts.channel}
                    <span class={`version-channel ${versionParts.channel.startsWith("EOL") ? "version-channel-eol" : versionParts.channel.startsWith("LTS") ? "version-channel-lts" : versionParts.channel.startsWith("Active") ? "version-channel-active" : versionParts.channel.startsWith("Security") ? "version-channel-security" : "version-channel-current"}`}>
                      {versionParts.channel.replace("LTS - ", "LTS · ")}
                    </span>
                  {/if}
                </span>
                <Download size={16} strokeWidth={2} aria-hidden="true" />
              </Combobox.Item>
            {:else}
              <span class="version-empty">No downloadable versions found</span>
            {/each}
          </Combobox.Viewport>
        </Combobox.Content>
      </Combobox.Portal>
    </Combobox.Root>
    </div>

    {#if serviceName === "Apache"}
      <Button.Root class={`start-button${isRunning ? " running" : ""}`} type="button" aria-label={`${isRunning ? "Stop" : "Start"} ${serviceName}`} onclick={() => void toggleService()}>
        {#if isRunning}
          <Square size={16} strokeWidth={1.8} aria-hidden="true" />
        {:else}
          <Play size={18} strokeWidth={1.8} aria-hidden="true" />
        {/if}
      </Button.Root>
    {/if}
  </div>
</article>

{#if pendingVersion}
  <div class="modal-backdrop">
    <div class="download-modal" role="dialog" aria-modal="true" aria-labelledby={`${serviceTitleId}-download-title`}>
      <h3 id={`${serviceTitleId}-download-title`}>Download {serviceName} {pendingVersion}?</h3>
      <p>The runtime will be installed in the Harbor runtimes folder.</p>
      <div class="modal-actions">
        <Button.Root class="modal-cancel" type="button" onclick={() => (pendingVersion = "")}>Cancel</Button.Root>
        <Button.Root class="modal-confirm" type="button" onclick={() => { const version = pendingVersion; pendingVersion = ""; void installVersion(version); }}>Confirm</Button.Root>
      </div>
    </div>
  </div>
{/if}

{#if pendingRemovalVersion}
  <div class="modal-backdrop">
    <div class="download-modal" role="dialog" aria-modal="true" aria-labelledby={`${serviceTitleId}-remove-title`}>
      <h3 id={`${serviceTitleId}-remove-title`}>Eliminar {serviceName} {getVersionNumber(pendingRemovalVersion)}?</h3>
      <p>Se borrará permanentemente la versión instalada y sus archivos.</p>
      <label class="confirmation-label" for={`${serviceTitleId}-remove-confirmation`}>Escribe CONFIRMAR</label>
      <input
        id={`${serviceTitleId}-remove-confirmation`}
        class="confirmation-input"
        bind:value={removalConfirmation}
        autocomplete="off"
        spellcheck="false"
      />
      <div class="modal-actions">
        <Button.Root class="modal-cancel" type="button" onclick={() => (pendingRemovalVersion = "")}>Cancelar</Button.Root>
        <Button.Root class="modal-confirm modal-danger" type="button" disabled={isRemoving} onclick={() => void removeVersion()}>{isRemoving ? "Eliminando..." : "Eliminar"}</Button.Root>
      </div>
    </div>
  </div>
{/if}

{#if downloadError}
  <div class="download-error" role="alert">{downloadError}</div>
{/if}

<style>
  .service-card {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 12px;
    box-sizing: border-box;
    display: grid;
    gap: 28px;
    grid-template-columns: minmax(260px, 1fr) minmax(280px, 320px) minmax(340px, 420px);
    min-height: 92px;
    padding: 18px 28px;
  }

  .service-meta {
    align-items: center;
    display: grid;
    flex-shrink: 0;
    gap: 24px;
    grid-template-columns: 1fr 1fr 1fr;
    min-width: 0;
  }

  .meta-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 76px;
  }

  .meta-label {
    color: var(--color-boulder-600);
    font-size: 11px;
    text-transform: uppercase;
  }

  .meta-value {
    color: var(--color-boulder-950);
    font-size: 13px;
    font-weight: 600;
  }

  .runtime-state {
    align-items: center;
    color: var(--color-boulder-600);
    display: flex;
    font-size: 12px;
    gap: 6px;
    white-space: nowrap;
  }

  .runtime-state.ready {
    color: var(--color-east-bay-700);
  }

  .status-indicator {
    background: currentColor;
    border-radius: 50%;
    height: 7px;
    width: 7px;
  }

  .download-progress {
    align-items: center;
    background: conic-gradient(var(--color-east-bay-500) var(--download-progress), var(--color-boulder-200) 0deg);
    border-radius: 50%;
    display: inline-flex;
    height: 24px;
    justify-content: center;
    position: relative;
    width: 24px;
  }

  .download-progress::after {
    background: #ffffff;
    border-radius: 50%;
    content: "";
    inset: 3px;
    position: absolute;
  }

  .download-progress span {
    font-size: 7px;
    position: relative;
    z-index: 1;
  }

  .modal-backdrop {
    align-items: center;
    background: rgb(11 11 11 / 35%);
    display: flex;
    inset: 0;
    justify-content: center;
    position: fixed;
    z-index: 20;
  }

  .download-modal {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow: 0 18px 40px rgb(11 11 11 / 20%);
    max-width: 360px;
    padding: 24px;
    width: calc(100% - 32px);
  }

  .download-modal h3 { margin: 0; }
  .download-modal p { margin: 10px 0 20px; }
  .modal-actions { display: flex; gap: 8px; justify-content: flex-end; }
  :global(.modal-cancel), :global(.modal-confirm) { border: 1px solid var(--color-boulder-200); border-radius: 6px; padding: 8px 14px; }
  :global(.modal-confirm) { background: var(--color-east-bay-500); color: #ffffff; }
  :global(.modal-danger) { background: #a33c2c; }
  .confirmation-label { display: block; font-size: 12px; font-weight: 600; margin-bottom: 6px; }
  .confirmation-input { border: 1px solid var(--color-boulder-200); border-radius: 6px; box-sizing: border-box; font: inherit; padding: 9px 10px; width: 100%; }
  .confirmation-input:focus { border-color: #a33c2c; outline: 2px solid rgb(163 60 44 / 18%); }

  .service-identity {
    align-items: center;
    display: flex;
    flex: 1;
    gap: 20px;
    min-width: 0;
  }

  .service-icon {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-100);
    border-radius: 10px;
    color: var(--color-east-bay-500);
    display: flex;
    flex-shrink: 0;
    height: 48px;
    justify-content: center;
    width: 48px;
  }

  .service-icon svg {
    fill: currentColor;
    height: 25px;
    width: 25px;
  }

  .service-copy {
    min-width: 0;
  }

  h2 {
    color: var(--color-boulder-950);
    font-size: 17px;
    font-weight: 600;
    line-height: 1.25;
    margin: 0;
  }

  p {
    color: var(--color-east-bay-800);
    font-size: 14px;
    line-height: 1.4;
    margin: 4px 0 0;
  }

  .service-controls {
    align-items: center;
    display: flex;
    flex-shrink: 0;
    gap: 0;
    justify-content: stretch;
    justify-self: end;
    width: 435px;
  }

  .version-control-group {
    align-items: center;
    display: flex;
    flex: 1;
    min-width: 0;
  }

  .runtime-controls :global(.version-button) {
    flex: 1;
  }

  .service-controls > :global([data-combobox-root]) {
    display: flex;
  }

  .service-controls .version-anchor {
    flex: 1;
  }

  .service-controls :global(.version-button) {
    border-bottom-right-radius: 0;
    border-top-right-radius: 0;
  }

  .download-anchor {
    display: flex;
    width: 40px;
  }

  @media (max-width: 840px) {
    .service-card {
      align-items: flex-start;
      flex-wrap: wrap;
      grid-template-columns: minmax(220px, 1fr) auto;
    }

    .service-meta {
      margin-left: 68px;
      order: 3;
      width: calc(100% - 68px);
    }

    .service-controls {
      grid-column: 2;
      grid-row: 1;
      width: min(435px, 100%);
    }

    .runtime-controls {
      grid-column: 1 / -1;
      grid-row: 2;
    }
  }

  @media (max-width: 560px) {
    .service-card {
      gap: 16px;
      grid-template-columns: 1fr auto;
      padding: 16px;
    }

    .service-meta {
      gap: 12px;
      grid-column: 1 / -1;
      grid-template-columns: repeat(3, minmax(0, 1fr));
      margin-left: 0;
      width: 100%;
    }

    .service-controls {
      width: 100%;
    }

    .meta-value {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  :global(.version-button),
  :global(.start-button) {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    box-sizing: border-box;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font: inherit;
    justify-content: center;
    min-height: 40px;
    transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
  }

  :global(.version-button:hover),
  :global(.start-button:hover) {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-200);
    color: var(--color-east-bay-700);
  }

  :global(.version-button) {
    appearance: none;
    border-radius: 7px;
    justify-content: space-between;
    width: 100%;
    padding: 0 14px;
  }

  .selected-version-label {
    align-items: center;
    display: inline-flex;
    gap: 8px;
    min-width: 0;
  }

  .selected-version-channel {
    background: var(--color-east-bay-100);
    border-radius: 999px;
    color: var(--color-east-bay-800);
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.04em;
    line-height: 20px;
    padding: 0 7px;
    text-transform: uppercase;
  }

  :global(.version-button-open) {
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
    border-bottom-color: var(--color-boulder-200);
  }

  .version-anchor {
    align-items: center;
    display: flex;
    flex: 1;
    gap: 8px;
    min-width: 0;
  }

  :global(.download-selected-button) {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 0 7px 7px 0;
    color: var(--color-east-bay-700);
    display: inline-flex;
    flex-shrink: 0;
    height: 40px;
    justify-content: center;
    width: 40px;
  }

  :global(.download-selected-button:hover) {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-200);
  }

  :global(.version-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 0 0 8px 8px;
    box-shadow: 0 10px 24px rgb(11 11 11 / 14%);
    box-sizing: border-box;
    max-height: 320px;
    min-width: var(--bits-combobox-anchor-width);
    overflow-y: auto;
    padding: 6px 4px 8px;
    width: var(--bits-combobox-anchor-width);
    z-index: 10;
  }

  :global(.download-content) {
    min-width: var(--bits-combobox-anchor-width);
    width: var(--bits-combobox-anchor-width);
  }

  .version-content-heading {
    color: var(--color-boulder-950);
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.02em;
    padding: 7px 12px 6px;
  }

  :global(.version-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-950);
    cursor: pointer;
    display: flex;
    font: inherit;
    justify-content: space-between;
    gap: 12px;
    height: 40px;
    min-height: 40px;
    padding: 0 12px;
    user-select: none;
  }

  :global(.version-item[data-highlighted]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-800);
  }

  .remove-version-button {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    flex-shrink: 0;
    justify-content: center;
    padding: 4px;
  }

  .remove-version-button:hover {
    background: #fff0ed;
    color: #a33c2c;
  }

  .version-item-label {
    align-items: center;
    display: inline-flex;
    gap: 10px;
    min-width: 0;
  }

  .version-channel {
    border-radius: 999px;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.04em;
    line-height: 20px;
    padding: 0 7px;
    text-transform: uppercase;
  }

  .version-channel-current {
    background: #edf4ff;
    color: #3568a8;
  }

  .version-channel-lts {
    background: #eaf7f0;
    color: #28734a;
  }

  .version-channel-security {
    background: #fff7e6;
    color: #9a6700;
  }

  .version-channel-active {
    background: #e8f7f8;
    color: #18727a;
  }

  .version-channel-eol {
    background: #fff0ed;
    color: #a33c2c;
  }

  .download-status-dot {
    border: 1.5px solid var(--color-east-bay-400);
    border-radius: 50%;
    display: inline-block;
    flex: 0 0 8px;
    height: 8px;
    width: 8px;
  }

  :global(.download-version-icon) {
    color: var(--color-east-bay-700);
    flex-shrink: 0;
  }

  .version-search-row {
    align-items: center;
    display: flex;
    position: relative;
  }

  :global(.version-search-icon) {
    color: var(--color-east-bay-800);
    left: 12px;
    pointer-events: none;
    position: absolute;
    z-index: 1;
  }

  :global(.version-search) {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 5px;
    box-sizing: border-box;
    color: var(--color-boulder-950);
    font: inherit;
    height: 36px;
    outline: none;
    padding: 0 12px 0 36px;
    width: 100%;
  }

  :global(.version-search:focus) {
    border-color: var(--color-east-bay-500);
  }

  .version-empty {
    color: var(--color-east-bay-800);
    display: block;
    padding: 9px;
  }

  :global(.start-button) {
    border-radius: 7px;
    width: 40px;
  }

  :global(.start-button) {
    margin-left: 0;
  }

  @media (max-width: 720px) {
    .service-card {
      align-items: stretch;
      flex-direction: column;
      padding: 18px;
    }

    .service-controls {
      width: 100%;
      min-width: 0;
    }

    :global(.version-button) {
      flex: 1;
      min-width: 0;
    }
  }
</style>
