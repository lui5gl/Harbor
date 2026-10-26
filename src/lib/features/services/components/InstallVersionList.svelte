<script lang="ts">
  import { Check, Download, Search } from "@lucide/svelte";
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

  function channelClass(channel: string) {
    const kind = getRuntimeChannelClass(channel);
    if (kind === "active" || kind === "current") return "bg-emerald-50 text-emerald-800";
    if (kind === "lts") return "bg-east-bay-100 text-east-bay-800";
    if (kind === "security") return "bg-amber-50 text-amber-800";
    if (kind === "eol") return "bg-red-50 text-red-800";
    return "bg-boulder-100 text-boulder-700";
  }
</script>

<div class="mb-3.5 flex flex-col gap-2.5">
  <div class="flex items-center gap-2 rounded-lg border border-boulder-200 bg-boulder-50 px-2.5">
    <Search size={15} class="shrink-0 text-boulder-400" /><input
      type="text"
      class="h-9 w-full border-0 bg-transparent text-[13px] text-boulder-950 outline-none"
      bind:value={searchQuery}
      placeholder={`Search ${serviceLabel} versions (e.g. 8.4, 22, LTS)...`}
    />
  </div>
  <div class="flex gap-1.5">
    <button
      type="button"
      class={`cursor-pointer rounded-full border px-2.5 py-1 text-xs font-semibold ${filterChannel === "all" ? "border-east-bay-200 bg-east-bay-100 text-east-bay-800" : "border-transparent bg-boulder-100 text-boulder-700"}`}
      onclick={() => (filterChannel = "all")}>All versions ({parsedVersions.length})</button
    ><button
      type="button"
      class={`cursor-pointer rounded-full border px-2.5 py-1 text-xs font-semibold ${filterChannel === "stable" ? "border-east-bay-200 bg-east-bay-100 text-east-bay-800" : "border-transparent bg-boulder-100 text-boulder-700"}`}
      onclick={() => (filterChannel = "stable")}>Active / LTS only</button
    >
  </div>
</div>

<div class="min-h-0 flex-1 overflow-y-auto rounded-lg border border-boulder-200 p-1">
  {#if filteredVersions.length === 0}<div
      class="px-4 py-8 text-center text-[13px] text-boulder-500"
    >
      No versions match your search criteria.
    </div>{:else}
    <div class="flex flex-col">
      {#each displayedVersions as item (item.number)}
        <div
          class={`flex items-center justify-between rounded-md border-b border-boulder-100 px-3 py-2.5 ${item.isInstalled ? "bg-boulder-50/60" : ""}`}
        >
          <div class="flex items-center gap-2.5">
            <span class="font-mono text-[13.5px] font-semibold text-boulder-950">{item.number}</span
            >{#if item.channel}<span
                class={`rounded-full px-2 py-0.5 text-[11px] font-semibold ${channelClass(item.channel)}`}
                >{formatRuntimeChannel(item.channel)}</span
              >{/if}
          </div>
          {#if item.isInstalled}<span
              class="inline-flex items-center gap-1 text-xs font-semibold text-emerald-600"
              ><Check size={14} strokeWidth={2.4} /><span>Installed</span></span
            >{:else}<button
              type="button"
              class="inline-flex h-7.5 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-2.5 text-xs font-semibold text-white hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
              disabled={isInstalling}
              onclick={() => onInstall(item.raw)}><Download size={14} /><span>Install</span></button
            >{/if}
        </div>
      {/each}
      {#if filteredVersions.length > displayedVersions.length}<div
          class="m-1 rounded bg-boulder-50 px-3 py-2.5 text-center text-[11.5px] text-boulder-500"
        >
          Showing first {displayedVersions.length} of {filteredVersions.length} versions. Use the search
          field to filter.
        </div>{/if}
    </div>
  {/if}
</div>
