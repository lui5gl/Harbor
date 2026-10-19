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

<div class="flex flex-col gap-1.5">
  {#each installedVersions as version (version)}
    {@const cleanVersion = cleanRuntimeVersion(version)}
    {@const isActive = cleanVersion === cleanActiveVersion}
    {@const metadata = parseRuntimeVersion(
      availableVersions.find((candidate) => cleanRuntimeVersion(candidate) === cleanVersion) ??
        version,
    )}
    <div
      class={`flex items-center justify-between rounded-md border bg-white px-3.5 py-2.5 ${isActive ? "border-east-bay-300 shadow-sm" : "border-boulder-200"}`}
    >
      <div class="flex items-center gap-2.5">
        <span class="font-mono text-[13.5px] font-semibold text-boulder-950">v{cleanVersion}</span
        >{#if metadata.channel}<span
            class="rounded-full bg-boulder-100 px-2 py-0.5 text-[10.5px] font-semibold text-boulder-700"
            >{formatRuntimeChannel(metadata.channel)}</span
          >{/if}{#if isActive}<span
            class="rounded-full bg-east-bay-100 px-2 py-0.5 text-[10.5px] font-bold uppercase tracking-[0.04em] text-east-bay-800"
            >Active</span
          >{/if}
      </div>
      <div class="flex items-center gap-2">
        {#if !isActive}<button
            type="button"
            class="cursor-pointer rounded bg-boulder-100 px-2.5 py-1.5 text-[11.5px] font-semibold text-boulder-700 hover:bg-east-bay-100 hover:text-east-bay-800"
            onclick={() => onSelect(version)}>Set as active</button
          >{/if}<button
          type="button"
          class="inline-flex size-7 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-red-50 hover:text-red-600"
          title={`Delete Node.js ${version}`}
          aria-label={`Delete Node.js ${version}`}
          onclick={() => onDelete(version)}><Trash2 size={15} /></button
        >
      </div>
    </div>
  {/each}
</div>
