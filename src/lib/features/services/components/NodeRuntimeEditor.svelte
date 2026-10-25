<script lang="ts">
  import { Download, Plus, Terminal } from "@lucide/svelte";
  import DeleteRuntimeDialog from "./DeleteRuntimeDialog.svelte";
  import InstallVersionDialog from "./InstallVersionDialog.svelte";
  import NodeInstalledVersions from "./NodeInstalledVersions.svelte";
  import { cleanVersion } from "../types";
  type NodeRuntimeEditorProps = {
    activeNodeVersion: string | null;
    installedNodeVersions: string[];
    availableNodeVersions: string[];
    isInstalling: boolean;
    installProgress: number;
    installingVersion: string;
    installError: string;
    onSelectNodeVersion: (version: string) => Promise<void>;
    onInstallNodeVersion: (version: string) => Promise<void>;
    onDeleteNodeVersion: (version: string) => Promise<void>;
  };
  let {
    activeNodeVersion,
    installedNodeVersions,
    availableNodeVersions,
    isInstalling,
    installProgress,
    installingVersion,
    installError,
    onSelectNodeVersion,
    onInstallNodeVersion,
    onDeleteNodeVersion,
  }: NodeRuntimeEditorProps = $props();
  let isInstallDialogOpen = $state(false);
  let pendingDeleteVersion = $state<string | null>(null);
  let cleanActiveVersion = $derived(activeNodeVersion ? cleanVersion(activeNodeVersion) : null);
  function requestDelete(version: string) {
    pendingDeleteVersion = cleanVersion(version);
  }
  function confirmDelete() {
    if (pendingDeleteVersion) {
      void onDeleteNodeVersion(pendingDeleteVersion);
      pendingDeleteVersion = null;
    }
  }
</script>

<section
  class="flex w-full flex-col gap-6 rounded-lg border border-boulder-200 bg-white p-6"
  aria-labelledby="node-editor-title"
>
  <header class="flex items-start justify-between gap-5 border-b border-boulder-100 pb-5">
    <div class="min-w-0">
      <p class="text-shine mb-1.5 text-[11px] font-bold uppercase tracking-[0.08em]">
        JavaScript Runtime
      </p>
      <h2 id="node-editor-title" class="text-[22px] leading-tight font-semibold text-boulder-950">
        Node.js Runtime
      </h2>
      <p class="mt-1.5 text-[13.5px] leading-relaxed text-boulder-600">
        Manage Node.js engine versions, active switch, and local tooling configurations.
      </p>
    </div>
    <div class="flex shrink-0 items-center gap-3">
      {#if activeNodeVersion}<div
          class="inline-flex items-center gap-2 rounded-full border border-emerald-200 bg-emerald-50 px-3 py-1.5 text-xs font-semibold text-emerald-800"
        >
          <span class="size-1.5 rounded-full bg-emerald-500"></span><span
            >Active: v{cleanActiveVersion}</span
          >
        </div>{/if}<button
        class="inline-flex h-8.5 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3.5 text-[12.5px] font-semibold text-white hover:bg-east-bay-950"
        type="button"
        onclick={() => (isInstallDialogOpen = true)}
        ><Plus size={15} /><span>Install Node.js</span></button
      >
    </div>
  </header>
  <div class="flex flex-col gap-4 rounded-lg border border-boulder-200 bg-boulder-50 px-5 py-4.5">
    <div class="flex items-start gap-3">
      <div
        class="flex size-9 shrink-0 items-center justify-center rounded-lg bg-emerald-50 text-emerald-800"
      >
        <Terminal size={17} />
      </div>
      <div>
        <h3 class="text-[15px] font-semibold text-boulder-950">Installed Node.js Versions</h3>
        <p class="mt-1 text-[12.5px] leading-snug text-boulder-600">
          The selected active version is written to Harbor runtime config and available for CLI
          operations.
        </p>
      </div>
    </div>
    {#if installedNodeVersions.length === 0}<div
        class="flex flex-col items-center gap-2.5 rounded-md border border-dashed border-boulder-300 bg-white p-5 text-center"
      >
        <p class="text-[13px] text-boulder-500">No Node.js versions are installed yet.</p>
        <button
          class="inline-flex h-8.5 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3.5 text-[12.5px] font-semibold text-white"
          type="button"
          onclick={() => (isInstallDialogOpen = true)}
          ><Download size={14} /><span>Install Node.js LTS</span></button
        >
      </div>{:else}<NodeInstalledVersions
        installedVersions={installedNodeVersions}
        availableVersions={availableNodeVersions}
        activeVersion={activeNodeVersion}
        onSelect={(version) => void onSelectNodeVersion(version)}
        onDelete={requestDelete}
      />{/if}
    <div class="grid grid-cols-2 gap-4 border-t border-boulder-200 pt-3.5">
      <div class="flex flex-col gap-0.5">
        <span class="text-[11px] font-semibold uppercase text-boulder-500">Runtime Path</span><span
          class="truncate font-mono text-xs text-boulder-700"
          >{cleanActiveVersion
            ? `C:\Harbor\runtimes\nodejs\${cleanActiveVersion}`
            : "No active version"}</span
        >
      </div>
      <div class="flex flex-col gap-0.5">
        <span class="text-[11px] font-semibold uppercase text-boulder-500">Config Location</span
        ><span class="truncate font-mono text-xs text-boulder-700"
          >C:\Harbor\config\active-runtimes.json</span
        >
      </div>
    </div>
  </div>
</section>

<InstallVersionDialog
  bind:open={isInstallDialogOpen}
  serviceLabel="Node.js"
  availableVersions={availableNodeVersions}
  installedVersions={installedNodeVersions}
  {isInstalling}
  {installProgress}
  {installingVersion}
  {installError}
  onOpenChange={(open) => (isInstallDialogOpen = open)}
  onInstall={(version) => void onInstallNodeVersion(version)}
/>
<DeleteRuntimeDialog
  open={Boolean(pendingDeleteVersion)}
  serviceLabel="Node.js"
  version={pendingDeleteVersion ?? ""}
  onOpenChange={(open) => {
    if (!open) pendingDeleteVersion = null;
  }}
  onConfirm={confirmDelete}
/>
