<script lang="ts">
  import { Trash2 } from "@lucide/svelte";
  import {
    cleanRuntimeVersion,
    formatRuntimeChannel,
    parseRuntimeVersion,
  } from "../application/runtimeVersions";

  type Props = {
    installedVersions: readonly string[];
    availableVersions: readonly string[];
    activeVersion: string | null;
    onSelect: (version: string) => void;
    onDelete: (version: string) => void;
  };

  let { installedVersions, availableVersions, activeVersion, onSelect, onDelete }: Props = $props();
  let cleanActiveVersion = $derived(activeVersion ? cleanRuntimeVersion(activeVersion) : null);
</script>

<div class="installed-versions-list">
  {#each installedVersions as version (version)}
    {@const cleanVersion = cleanRuntimeVersion(version)}
    {@const isActive = cleanVersion === cleanActiveVersion}
    {@const metadata = parseRuntimeVersion(
      availableVersions.find((candidate) => cleanRuntimeVersion(candidate) === cleanVersion) ??
        version,
    )}
    <div class={`runtime-row${isActive ? " is-active" : ""}`}>
      <div class="runtime-info">
        <span class="version-name">v{cleanVersion}</span>
        {#if metadata.channel}<span class="channel-tag"
            >{formatRuntimeChannel(metadata.channel)}</span
          >{/if}
        {#if isActive}<span class="active-tag">Active</span>{/if}
      </div>
      <div class="runtime-actions">
        {#if !isActive}
          <button type="button" class="btn-activate" onclick={() => onSelect(version)}>
            <span>Set as active</span>
          </button>
        {/if}
        <button
          type="button"
          class="btn-delete-icon"
          title={`Delete Node.js ${version}`}
          aria-label={`Delete Node.js ${version}`}
          onclick={() => onDelete(version)}
        >
          <Trash2 size={15} strokeWidth={2} aria-hidden="true" />
        </button>
      </div>
    </div>
  {/each}
</div>

<style>
  .installed-versions-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .runtime-row {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    display: flex;
    justify-content: space-between;
    padding: 10px 14px;
    transition: border-color 150ms ease;
  }
  .runtime-row.is-active {
    border-color: var(--color-east-bay-300);
    box-shadow: 0 1px 3px rgb(0 0 0 / 4%);
  }
  .runtime-info {
    align-items: center;
    display: flex;
    gap: 10px;
  }
  .version-name {
    color: var(--color-boulder-950);
    font-family: var(--font-mono);
    font-size: 13.5px;
    font-weight: 650;
  }
  .channel-tag {
    background: var(--color-boulder-100);
    border-radius: 999px;
    color: var(--color-boulder-700);
    font-size: 10.5px;
    font-weight: 600;
    padding: 2px 7px;
  }
  .active-tag {
    background: var(--color-east-bay-100);
    border-radius: 999px;
    color: var(--color-east-bay-800);
    font-size: 10.5px;
    font-weight: 700;
    letter-spacing: 0.04em;
    padding: 2px 8px;
    text-transform: uppercase;
  }
  .runtime-actions {
    align-items: center;
    display: flex;
    gap: 8px;
  }
  .btn-activate {
    background: var(--color-boulder-100);
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-700);
    cursor: pointer;
    font: inherit;
    font-size: 11.5px;
    font-weight: 600;
    padding: 5px 10px;
    transition:
      background-color 150ms ease,
      color 150ms ease;
  }
  .btn-activate:hover {
    background: var(--color-east-bay-100);
    color: var(--color-east-bay-800);
  }
  .btn-delete-icon {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: inline-flex;
    height: 28px;
    justify-content: center;
    padding: 0;
    transition:
      background-color 150ms ease,
      color 150ms ease;
    width: 28px;
  }
  .btn-delete-icon:hover {
    background: #fef2f2;
    color: #dc2626;
  }
</style>
