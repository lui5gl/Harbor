<script lang="ts">
  import { Check, ChevronDown, Download, Search, Trash2 } from "@lucide/svelte";
  import { Combobox } from "bits-ui";
  import {
    formatRuntimeChannel,
    getRuntimeChannelClass,
    parseRuntimeVersion,
  } from "../application/runtimeVersions";

  type Props = {
    serviceName: string;
    installedVersions: readonly string[];
    downloadableVersions: readonly string[];
    selectedVersion: string;
    installingVersion: string;
    downloadProgress: number;
    onSelect: (version: string) => void;
    onRequestInstall: (version: string) => void;
    onRequestRemoval: (version: string) => void;
  };

  let {
    serviceName,
    installedVersions,
    downloadableVersions,
    selectedVersion = $bindable(),
    installingVersion,
    downloadProgress,
    onSelect,
    onRequestInstall,
    onRequestRemoval,
  }: Props = $props();

  let searchValue = $state("");
  let downloadSearchValue = $state("");
  let downloadAnchor = $state<HTMLDivElement | null>(null);
  let isVersionMenuOpen = $state(false);
  let isDownloadMenuOpen = $state(false);

  let filteredDownloadVersions = $derived(
    downloadSearchValue === ""
      ? downloadableVersions
      : downloadableVersions.filter((version) =>
          version.toLowerCase().includes(downloadSearchValue.toLowerCase()),
        ),
  );
</script>

<div class="version-control-group" bind:this={downloadAnchor}>
  <Combobox.Root
    type="single"
    items={installedVersions.map((version) => ({ value: version, label: version }))}
    bind:value={selectedVersion}
    onValueChange={(value) => value && onSelect(value)}
    bind:open={isVersionMenuOpen}
    onOpenChangeComplete={(isOpen) => {
      if (!isOpen) searchValue = "";
    }}
  >
    <div class="version-anchor">
      <Combobox.Trigger
        class={`version-button${isVersionMenuOpen ? " version-button-open" : ""}`}
        aria-label={`Select installed ${serviceName} version`}
      >
        {#if selectedVersion}
          {@const selectedParts = parseRuntimeVersion(selectedVersion)}
          <span class="selected-version-label">
            <span>{selectedParts.number}</span>
            {#if selectedParts.channel}
              <span class="selected-version-channel">
                {formatRuntimeChannel(selectedParts.channel)}
              </span>
            {/if}
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
          {#if installedVersions.length > 0}
            {#each installedVersions as version (version)}
              {@const versionParts = parseRuntimeVersion(version)}
              <Combobox.Item class="version-item" value={version} label={version}>
                {#snippet children({ selected })}
                  <span class="version-item-label">
                    <span>{versionParts.number}</span>
                    {#if versionParts.channel}
                      <span
                        class={`version-channel version-channel-${getRuntimeChannelClass(versionParts.channel)}`}
                      >
                        {formatRuntimeChannel(versionParts.channel)}
                      </span>
                    {/if}
                  </span>
                  {#if selected}<Check size={16} strokeWidth={2} aria-hidden="true" />{/if}
                  <button
                    class="remove-version-button"
                    type="button"
                    aria-label={`Remove ${version}`}
                    onclick={(event) => {
                      event.stopPropagation();
                      onRequestRemoval(version);
                    }}
                  >
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
    items={downloadableVersions.map((version) => ({ value: version, label: version }))}
    bind:open={isDownloadMenuOpen}
    onOpenChangeComplete={(isOpen) => {
      if (!isOpen) downloadSearchValue = "";
    }}
  >
    <div class="download-anchor">
      <Combobox.Trigger
        class="download-selected-button"
        aria-label={`Install another ${serviceName} version`}
        disabled={Boolean(installingVersion)}
      >
        {#if installingVersion}
          <span
            class="download-progress"
            style={`--download-progress: ${downloadProgress * 3.6}deg`}
            aria-label={`${downloadProgress}% downloaded`}><span>{downloadProgress}%</span></span
          >
        {:else}
          <Download size={16} strokeWidth={2} aria-hidden="true" />
          <span>Install another version</span>
        {/if}
      </Combobox.Trigger>
    </div>
    <Combobox.Portal>
      <Combobox.Content
        class="version-content download-content"
        customAnchor={downloadAnchor}
        sideOffset={0}
      >
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
          {#each filteredDownloadVersions as version (version)}
            {@const versionParts = parseRuntimeVersion(version)}
            <Combobox.Item
              class="version-item"
              value={version}
              label={version}
              onclick={() => onRequestInstall(version)}
            >
              <span class="version-item-label">
                <span>{versionParts.number}</span>
                {#if versionParts.channel}
                  <span
                    class={`version-channel version-channel-${getRuntimeChannelClass(versionParts.channel)}`}
                  >
                    {formatRuntimeChannel(versionParts.channel)}
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

<style>
  .version-control-group {
    align-items: center;
    display: flex;
    flex: 1;
    min-width: 0;
  }
  .version-anchor {
    align-items: center;
    display: flex;
    flex: 1;
    gap: 8px;
    min-width: 0;
  }
  .download-anchor {
    display: flex;
    width: 40px;
  }
  :global(.version-button),
  :global(.download-selected-button) {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    box-sizing: border-box;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font: inherit;
    justify-content: center;
    min-height: 40px;
    transition:
      background-color 150ms ease,
      border-color 150ms ease,
      color 150ms ease;
  }
  :global(.version-button:hover),
  :global(.download-selected-button:hover) {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-200);
    color: var(--color-east-bay-700);
  }
  :global(.version-button) {
    appearance: none;
    border-radius: 7px 0 0 7px;
    justify-content: space-between;
    padding: 0 14px;
    width: 100%;
  }
  :global(.version-button-open) {
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
    border-bottom-color: var(--color-boulder-200);
  }
  :global(.download-selected-button) {
    border-radius: 0 7px 7px 0;
    color: var(--color-east-bay-700);
    flex-shrink: 0;
    height: 40px;
    width: 40px;
  }
  .selected-version-label,
  .version-item-label {
    align-items: center;
    display: inline-flex;
    gap: 8px;
    min-width: 0;
  }
  .selected-version-channel,
  .version-channel {
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
  .version-search-row {
    align-items: center;
    display: flex;
    position: relative;
  }
  :global(.version-search-icon) {
    color: var(--color-east-bay-800);
    left: 10px;
    position: absolute;
  }
  :global(.version-search) {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 5px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 12px;
    height: 32px;
    outline: none;
    padding: 0 10px 0 32px;
    width: 100%;
  }
  :global(.version-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-950);
    cursor: pointer;
    display: flex;
    font: inherit;
    gap: 12px;
    height: 40px;
    justify-content: space-between;
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
  .version-empty {
    color: var(--color-boulder-500);
    display: block;
    font-size: 12px;
    padding: 12px;
  }
  .download-progress {
    align-items: center;
    background: conic-gradient(
      var(--color-east-bay-500) var(--download-progress),
      var(--color-boulder-200) 0deg
    );
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
</style>
