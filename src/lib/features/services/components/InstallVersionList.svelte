<script lang="ts">
  import { Check, Download, Search } from "@lucide/svelte";
  import { ScrollArea } from "bits-ui";
  import {
    formatRuntimeChannel,
    getRuntimeChannelClass,
    parseRuntimeVersion,
  } from "../application/runtimeVersions";

  type Props = {
    serviceLabel: string;
    availableVersions: readonly string[];
    installedVersions: readonly string[];
    isInstalling: boolean;
    onInstall: (version: string) => void;
  };

  let { serviceLabel, availableVersions, installedVersions, isInstalling, onInstall }: Props =
    $props();
  let searchQuery = $state("");
  let filterChannel = $state<"all" | "stable">("all");

  let parsedVersions = $derived(
    availableVersions.map((raw) => {
      const metadata = parseRuntimeVersion(raw);
      return {
        raw,
        ...metadata,
        isInstalled: installedVersions.includes(metadata.number.trimStart().replace(/^v/, "")),
      };
    }),
  );

  let filteredVersions = $derived(
    parsedVersions.filter((item) => {
      const query = searchQuery.trim().toLowerCase();
      const matchesQuery =
        !query ||
        item.number.toLowerCase().includes(query) ||
        item.channel.toLowerCase().includes(query);
      const isStable = ["Active", "LTS", "Current"].some((channel) =>
        item.channel.startsWith(channel),
      );
      return matchesQuery && (filterChannel === "all" || isStable);
    }),
  );

  let displayedVersions = $derived(filteredVersions.slice(0, 60));
</script>

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
        {#each displayedVersions as item (item.number)}
          <div class={`version-row${item.isInstalled ? " is-installed" : ""}`}>
            <div class="version-info">
              <span class="version-number">{item.number}</span>
              {#if item.channel}
                <span class={`channel-badge channel-${getRuntimeChannelClass(item.channel)}`}>
                  {formatRuntimeChannel(item.channel)}
                </span>
              {/if}
            </div>
            <div class="version-actions">
              {#if item.isInstalled}
                <span class="installed-tag"
                  ><Check size={14} strokeWidth={2.4} aria-hidden="true" /><span>Installed</span
                  ></span
                >
              {:else}
                <button
                  type="button"
                  class="btn-install"
                  disabled={isInstalling}
                  onclick={() => onInstall(item.raw)}
                >
                  <Download size={14} strokeWidth={2} aria-hidden="true" /><span>Install</span>
                </button>
              {/if}
            </div>
          </div>
        {/each}
        {#if filteredVersions.length > displayedVersions.length}
          <div class="truncated-notice">
            Showing first {displayedVersions.length} of {filteredVersions.length} versions. Use the search
            field to filter.
          </div>
        {/if}
      </div>
    {/if}
  </ScrollArea.Viewport>
  <ScrollArea.Scrollbar class="dialog-scrollbar" orientation="vertical">
    <ScrollArea.Thumb class="dialog-scrollbar-thumb" />
  </ScrollArea.Scrollbar>
</ScrollArea.Root>

<style>
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
    background: var(--color-boulder-50);
    border-radius: 4px;
    color: var(--color-boulder-500);
    font-size: 11.5px;
    margin: 4px;
    padding: 10px 12px;
    text-align: center;
  }
  .version-row {
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
    border-radius: 6px;
    display: flex;
    justify-content: space-between;
    padding: 10px 12px;
  }
  .version-info {
    align-items: center;
    display: flex;
    gap: 10px;
  }
  .version-number {
    color: var(--color-boulder-950);
    font-family: var(--font-mono);
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
  }
</style>
