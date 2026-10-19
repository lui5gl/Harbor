<script lang="ts">
  import { Download, Trash2 } from "@lucide/svelte";
  import { formatRuntimeChannel, parseRuntimeVersion } from "../application/runtimeVersions";
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
  let installSelection = $state("");

  function selectInstalled(value: string) {
    selectedVersion = value;
    if (value) onSelect(value);
  }
  function requestInstall(value: string) {
    if (!value) return;
    onRequestInstall(value);
    installSelection = "";
  }
</script>

<div class="flex min-w-0 flex-1 items-center">
  <div class="relative min-w-0 flex-1">
    <select
      class="h-10 w-full cursor-pointer appearance-none rounded-l-lg border border-boulder-200 bg-boulder-50 px-3 pr-8 text-[13px] font-semibold text-east-bay-800 outline-none hover:border-east-bay-200 hover:bg-east-bay-50 focus:border-east-bay-400"
      aria-label={`Select installed ${serviceName} version`}
      value={selectedVersion}
      onchange={(event) => selectInstalled(event.currentTarget.value)}
    >
      {#if installedVersions.length === 0}<option value="">No installed versions</option>{/if}
      {#each installedVersions as version (version)}{@const parts =
          parseRuntimeVersion(version)}<option value={version}
          >{parts.number}{parts.channel ? ` · ${formatRuntimeChannel(parts.channel)}` : ""}</option
        >{/each}
    </select>
  </div>
  <div
    class="relative flex h-10 w-10 shrink-0 items-center justify-center rounded-r-lg border border-l-0 border-boulder-200 bg-boulder-50 text-east-bay-700 hover:bg-east-bay-50"
  >
    {#if installingVersion}<span class="text-[9px] font-bold">{downloadProgress}%</span
      >{:else}<Download size={16} />{/if}
    <select
      class="absolute inset-0 cursor-pointer opacity-0"
      aria-label={`Install another ${serviceName} version`}
      value={installSelection}
      disabled={Boolean(installingVersion)}
      onchange={(event) => requestInstall(event.currentTarget.value)}
    >
      <option value="">Install version</option>
      {#each downloadableVersions as version (version)}<option value={version}>{version}</option
        >{/each}
    </select>
  </div>
  {#if selectedVersion}
    <button
      class="ml-1 inline-flex size-8 cursor-pointer items-center justify-center rounded-md text-boulder-400 hover:bg-red-50 hover:text-red-600"
      type="button"
      aria-label={`Remove ${selectedVersion}`}
      title={`Remove ${selectedVersion}`}
      onclick={() => onRequestRemoval(selectedVersion)}><Trash2 size={15} /></button
    >
  {/if}
</div>
