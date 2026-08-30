<script lang="ts">
  import { Check, Download, Loader2, Search, X } from "@lucide/svelte";
  import { Dialog, ScrollArea } from "bits-ui";
  import { cleanVersion, parseVersionString } from "./types";

  type InstallVersionDialogProps = {
    open: boolean;
    serviceLabel: string;
    availableVersions: string[];
    installedVersions: string[];
    isInstalling: boolean;
    installProgress: number;
    installingVersion: string;
    installError?: string;
    onOpenChange: (open: boolean) => void;
    onInstall: (version: string) => Promise<void> | void;
  };

  let {
    open = $bindable(),
    serviceLabel,
    availableVersions,
    installedVersions,
    isInstalling,
    installProgress,
    installingVersion,
    installError = "",
    onOpenChange,
    onInstall
  }: InstallVersionDialogProps = $props();

  let searchQuery = $state("");
  let filterChannel = $state<"all" | "stable">("all");

  let parsedVersions = $derived(
    availableVersions.map((raw) => {
      const { version, channel } = parseVersionString(raw);
      const isInstalled = installedVersions.includes(version);
      return { raw, version, channel, isInstalled };
    })
  );

  let filteredVersions = $derived(
    parsedVersions.filter((item) => {
      if (searchQuery.trim()) {
        const query = searchQuery.trim().toLowerCase();
        const matchesVersion = item.version.toLowerCase().includes(query);
        const matchesChannel = item.channel.toLowerCase().includes(query);
        if (!matchesVersion && !matchesChannel) return false;
      }

      if (filterChannel === "stable") {
        const isStable =
          item.channel.startsWith("Active") ||
          item.channel.startsWith("LTS") ||
          item.channel.startsWith("Current");
        if (!isStable) return false;
      }

      return true;
    })
  );

  let displayedVersions = $derived(filteredVersions.slice(0, 60));

  $effect(() => {
    if (!open) {
      searchQuery = "";
      filterChannel = "all";
    }
  });

  function handleInstallClick(versionRaw: string) {
    if (isInstalling) return;
    void onInstall(versionRaw);
  }
</script>

<Dialog.Root bind:open {onOpenChange}>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="dialog-content">
      <div class="dialog-header">
        <div class="header-titles">
          <Dialog.Title class="dialog-title">Install {serviceLabel} Version</Dialog.Title>
          <Dialog.Description class="dialog-description">
            Download and install binary versions into your local Harbor runtimes repository.
          </Dialog.Description>
        </div>
        <Dialog.Close class="btn-close" aria-label="Close dialog">
          <X size={18} strokeWidth={2} />
        </Dialog.Close>
      </div>

      {#if installError}
        <div class="dialog-error" role="alert">
          {installError}
        </div>
      {/if}

      {#if isInstalling}
        <div class="installing-banner" role="status">
          <div class="installing-header">
            <div class="spinner-box">
              <Loader2 size={16} class="spinner" strokeWidth={2.4} />
            </div>
            <span class="installing-title">
              Downloading & installing {serviceLabel} {installingVersion}...
            </span>
            <span class="installing-percent">{installProgress}%</span>
          </div>
          <div class="progress-track" aria-hidden="true">
            <div class="progress-bar" style={`width: ${Math.max(4, installProgress)}%`}></div>
          </div>
        </div>
      {/if}

      <div class="filter-toolbar">
        <div class="search-field">
          <Search size={15} strokeWidth={2} class="search-icon" aria-hidden="true" />
          <input
            type="text"
            class="search-input"
            bind:value={searchQuery}
            placeholder={`Search ${serviceLabel} versions (e.g. 8.4, 22, LTS)...`}
          />
        </div>
        <div class="filter-pills">
          <button
            type="button"
            class={`filter-pill${filterChannel === "all" ? " active" : ""}`}
            onclick={() => (filterChannel = "all")}
          >
            All versions ({parsedVersions.length})
          </button>
          <button
            type="button"
            class={`filter-pill${filterChannel === "stable" ? " active" : ""}`}
            onclick={() => (filterChannel = "stable")}
          >
            Active / LTS only
          </button>
        </div>
      </div>

      <ScrollArea.Root class="version-list-scroll" type="auto">
        <ScrollArea.Viewport class="version-list-viewport">
          {#if filteredVersions.length === 0}
            <div class="empty-list">No versions match your search criteria.</div>
          {:else}
            <div class="version-list">
              {#each displayedVersions as item (item.version)}
                <div class={`version-row${item.isInstalled ? " is-installed" : ""}`}>
                  <div class="version-info">
                    <span class="version-number">{item.version}</span>
                    {#if item.channel}
                      <span
                        class={`channel-badge ${
                          item.channel.startsWith("Active")
                            ? "channel-active"
                            : item.channel.startsWith("LTS")
                            ? "channel-lts"
                            : item.channel.startsWith("Current")
                            ? "channel-current"
                            : item.channel.startsWith("Security")
                            ? "channel-security"
                            : "channel-eol"
                        }`}
                      >
                        {item.channel.replace("LTS - ", "LTS · ")}
                      </span>
                    {/if}
                  </div>

                  <div class="version-actions">
                    {#if item.isInstalled}
                      <span class="installed-tag">
                        <Check size={14} strokeWidth={2.4} aria-hidden="true" />
                        <span>Installed</span>
                      </span>
                    {:else}
                      <button
                        type="button"
                        class="btn-install"
                        disabled={isInstalling}
                        onclick={() => handleInstallClick(item.raw)}
                      >
                        <Download size={14} strokeWidth={2} aria-hidden="true" />
                        <span>Install</span>
                      </button>
                    {/if}
                  </div>
                </div>
              {/each}

              {#if filteredVersions.length > displayedVersions.length}
                <div class="truncated-notice">
                  Showing first {displayedVersions.length} of {filteredVersions.length} versions. Use the search field to filter.
                </div>
              {/if}
            </div>
          {/if}
        </ScrollArea.Viewport>
        <ScrollArea.Scrollbar class="dialog-scrollbar" orientation="vertical">
          <ScrollArea.Thumb class="dialog-scrollbar-thumb" />
        </ScrollArea.Scrollbar>
      </ScrollArea.Root>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.modal-backdrop) {
    background: rgb(11 11 11 / 45%);
    backdrop-filter: blur(2px);
    inset: 0;
    position: fixed;
    z-index: 100;
  }

  :global(.dialog-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 12px;
    box-shadow: 0 20px 48px rgb(11 11 11 / 22%);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    height: min(640px, calc(100vh - 48px));
    left: 50%;
    max-width: 580px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: calc(100% - 32px);
    z-index: 101;
  }

  .dialog-header {
    align-items: flex-start;
    display: flex;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .header-titles {
    min-width: 0;
  }

  :global(.dialog-title) {
    color: var(--color-boulder-950);
    font-size: 17px;
    font-weight: 650;
    line-height: 1.3;
    margin: 0;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 13.5px;
    line-height: 1.45;
    margin: 4px 0 0;
  }

  :global(.btn-close) {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 6px;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    height: 32px;
    justify-content: center;
    padding: 0;
    transition: background-color 150ms ease, color 150ms ease;
    width: 32px;
  }

  :global(.btn-close:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  .dialog-error {
    background: #fff3f1;
    border: 1px solid #efb5ad;
    border-radius: 6px;
    color: #913526;
    font-size: 13px;
    margin-bottom: 14px;
    padding: 10px 12px;
  }

  .installing-banner {
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 16px;
    padding: 12px 14px;
  }

  .installing-header {
    align-items: center;
    display: flex;
    gap: 8px;
  }

  .spinner-box {
    align-items: center;
    color: var(--color-east-bay-700);
    display: inline-flex;
  }

  :global(.spinner) {
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

  .installing-title {
    color: var(--color-east-bay-900);
    flex: 1;
    font-size: 13px;
    font-weight: 600;
  }

  .installing-percent {
    color: var(--color-east-bay-700);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
    font-weight: 700;
  }

  .progress-track {
    background: var(--color-east-bay-200);
    border-radius: 999px;
    height: 6px;
    overflow: hidden;
    width: 100%;
  }

  .progress-bar {
    background: var(--color-east-bay-600);
    border-radius: 999px;
    height: 100%;
    transition: width 200ms ease;
  }

  .filter-toolbar {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin-bottom: 14px;
  }

  .search-field {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 7px;
    box-sizing: border-box;
    display: flex;
    gap: 8px;
    padding: 0 10px;
    transition: border-color 150ms ease;
  }

  .search-field:focus-within {
    background: #ffffff;
    border-color: var(--color-east-bay-500);
  }

  :global(.search-icon) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
  }

  .search-input {
    background: transparent;
    border: 0;
    box-sizing: border-box;
    color: var(--color-boulder-950);
    font: inherit;
    font-size: 13px;
    height: 36px;
    outline: none;
    width: 100%;
  }

  .filter-pills {
    display: flex;
    gap: 6px;
  }

  .filter-pill {
    background: var(--color-boulder-100);
    border: 1px solid transparent;
    border-radius: 999px;
    color: var(--color-boulder-700);
    cursor: pointer;
    font: inherit;
    font-size: 12px;
    font-weight: 600;
    padding: 4px 10px;
    transition: background-color 150ms ease, color 150ms ease;
  }

  .filter-pill:hover {
    background: var(--color-boulder-200);
    color: var(--color-boulder-950);
  }

  .filter-pill.active {
    background: var(--color-east-bay-100);
    border-color: var(--color-east-bay-200);
    color: var(--color-east-bay-800);
  }

  :global(.version-list-scroll) {
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    display: flex;
    flex: 1;
    min-height: 0;
  }

  :global(.version-list-viewport) {
    height: 100%;
    width: 100%;
  }

  .version-list {
    display: flex;
    flex-direction: column;
    padding: 4px;
  }

  .empty-list {
    color: var(--color-boulder-500);
    font-size: 13px;
    padding: 32px 16px;
    text-align: center;
  }

  .truncated-notice {
    color: var(--color-boulder-500);
    font-size: 11.5px;
    padding: 10px 12px;
    text-align: center;
    background: var(--color-boulder-50);
    border-radius: 4px;
    margin: 4px;
  }

  .version-row {
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
    border-radius: 6px;
    display: flex;
    justify-content: space-between;
    padding: 10px 12px;
    transition: background-color 120ms ease;
  }

  .version-row:last-child {
    border-bottom: 0;
  }

  .version-row:hover {
    background: var(--color-boulder-50);
  }

  .version-info {
    align-items: center;
    display: flex;
    gap: 10px;
  }

  .version-number {
    color: var(--color-boulder-950);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 13.5px;
    font-weight: 600;
  }

  .channel-badge {
    border-radius: 999px;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.04em;
    padding: 2px 7px;
    text-transform: uppercase;
  }

  .channel-active {
    background: #ecfdf5;
    color: #065f46;
  }

  .channel-lts {
    background: var(--color-east-bay-100);
    color: var(--color-east-bay-800);
  }

  .channel-current {
    background: #f0fdf4;
    color: #166534;
  }

  .channel-security {
    background: #fffbeb;
    color: #92400e;
  }

  .channel-eol {
    background: #fef2f2;
    color: #991b1b;
  }

  .installed-tag {
    align-items: center;
    color: #059669;
    display: inline-flex;
    font-size: 12.5px;
    font-weight: 600;
    gap: 4px;
  }

  .btn-install {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 5px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 600;
    gap: 5px;
    height: 30px;
    padding: 0 12px;
    transition: background-color 150ms ease;
  }

  .btn-install:hover:not(:disabled) {
    background: var(--color-east-bay-950);
  }

  .btn-install:disabled {
    cursor: wait;
    opacity: 0.5;
  }

  :global(.dialog-scrollbar) {
    background: transparent;
    display: flex;
    padding: 2px;
    user-select: none;
    width: 8px;
  }

  :global(.dialog-scrollbar-thumb) {
    background: var(--color-boulder-300);
    border-radius: 4px;
    flex: 1;
  }
</style>
