<script lang="ts">
  import { Download, HardDrive, Plus, Terminal } from "@lucide/svelte";
  import { Button } from "bits-ui";
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

<section class="editor-panel" aria-labelledby="node-editor-title">
  <!-- Header -->
  <header class="editor-header">
    <div class="header-main">
      <div class="header-titles">
        <p class="eyebrow">JavaScript Runtime</p>
        <h2 id="node-editor-title">Node.js Runtime</h2>
      </div>
      <p class="header-sub">
        Manage Node.js engine versions, active switch, and local tooling configurations.
      </p>
    </div>

    <div class="header-controls">
      {#if activeNodeVersion}
        <div class="status-pill active">
          <span class="status-indicator" aria-hidden="true"></span>
          <span class="status-label">Active: v{cleanActiveVersion}</span>
        </div>
      {/if}

      <Button.Root
        class="primary-button-sm"
        type="button"
        onclick={() => (isInstallDialogOpen = true)}
      >
        <Plus size={15} strokeWidth={2.2} aria-hidden="true" />
        <span>Install Node.js</span>
      </Button.Root>
    </div>
  </header>

  <!-- Content Card -->
  <div class="runtime-card">
    <div class="card-header">
      <div class="card-title-group">
        <div class="card-icon node-icon" aria-hidden="true">
          <Terminal size={17} strokeWidth={2.2} />
        </div>
        <div>
          <h3>Installed Node.js Versions</h3>
          <p>
            The selected active version is written to Harbor runtime config and available for CLI
            operations.
          </p>
        </div>
      </div>
    </div>

    {#if installedNodeVersions.length === 0}
      <div class="empty-runtime-box">
        <p>No Node.js versions are installed yet.</p>
        <Button.Root
          class="primary-button-sm"
          type="button"
          onclick={() => (isInstallDialogOpen = true)}
        >
          <Download size={14} strokeWidth={2} aria-hidden="true" />
          <span>Install Node.js LTS</span>
        </Button.Root>
      </div>
    {:else}
      <NodeInstalledVersions
        installedVersions={installedNodeVersions}
        availableVersions={availableNodeVersions}
        activeVersion={activeNodeVersion}
        onSelect={(version) => void onSelectNodeVersion(version)}
        onDelete={requestDelete}
      />
    {/if}

    <div class="runtime-footer-meta">
      <div class="meta-field">
        <span class="meta-field-label">Runtime Path</span>
        <span class="meta-field-value">
          {cleanActiveVersion
            ? `C:\\Harbor\\runtimes\\nodejs\\${cleanActiveVersion}`
            : "No active version"}
        </span>
      </div>
      <div class="meta-field">
        <span class="meta-field-label">Config Location</span>
        <span class="meta-field-value">C:\Harbor\config\active-runtimes.json</span>
      </div>
    </div>
  </div>
</section>

<!-- Dialogs -->
<InstallVersionDialog
  bind:open={isInstallDialogOpen}
  serviceLabel="Node.js"
  availableVersions={availableNodeVersions}
  installedVersions={installedNodeVersions}
  {isInstalling}
  {installProgress}
  {installingVersion}
  {installError}
  onOpenChange={(open) => {
    isInstallDialogOpen = open;
  }}
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

<style>
  .editor-panel {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 24px;
    padding: 24px;
    width: 100%;
  }

  .editor-header {
    align-items: flex-start;
    border-bottom: 1px solid var(--color-boulder-100);
    display: flex;
    justify-content: space-between;
    padding-bottom: 20px;
    gap: 20px;
  }

  .header-main {
    min-width: 0;
  }

  .eyebrow {
    color: var(--color-east-bay-700);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.08em;
    margin: 0 0 6px;
    text-transform: uppercase;
  }

  h2 {
    color: var(--color-boulder-950);
    font-size: 22px;
    font-weight: 650;
    line-height: 1.2;
    margin: 0;
  }

  .header-sub {
    color: var(--color-boulder-600);
    font-size: 13.5px;
    line-height: 1.5;
    margin: 6px 0 0;
  }

  .header-controls {
    align-items: center;
    display: flex;
    flex-shrink: 0;
    gap: 12px;
  }

  .status-pill {
    align-items: center;
    background: var(--color-boulder-100);
    border: 1px solid var(--color-boulder-200);
    border-radius: 999px;
    color: var(--color-boulder-700);
    display: inline-flex;
    font-size: 12px;
    font-weight: 600;
    gap: 7px;
    padding: 6px 12px;
  }

  .status-pill.active {
    background: #ecfdf5;
    border-color: #a7f3d0;
    color: #065f46;
  }

  .status-indicator {
    background: #10b981;
    border-radius: 50%;
    height: 7px;
    width: 7px;
  }

  :global(.primary-button-sm) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 6px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12.5px;
    font-weight: 600;
    gap: 6px;
    height: 34px;
    padding: 0 14px;
    transition: background-color 150ms ease;
  }

  :global(.primary-button-sm:hover) {
    background: var(--color-east-bay-950);
  }

  .runtime-card {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 18px 20px;
  }

  .card-header {
    align-items: flex-start;
    display: flex;
    justify-content: space-between;
  }

  .card-title-group {
    align-items: flex-start;
    display: flex;
    gap: 12px;
  }

  .card-icon {
    align-items: center;
    border-radius: 8px;
    display: flex;
    flex-shrink: 0;
    height: 36px;
    justify-content: center;
    width: 36px;
  }

  .node-icon {
    background: #ecfdf5;
    color: #065f46;
  }

  .card-title-group h3 {
    color: var(--color-boulder-950);
    font-size: 15px;
    font-weight: 650;
    margin: 0;
  }

  .card-title-group p {
    color: var(--color-boulder-600);
    font-size: 12.5px;
    line-height: 1.4;
    margin: 3px 0 0;
  }

  .empty-runtime-box {
    align-items: center;
    background: #ffffff;
    border: 1px dashed var(--color-boulder-300);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 20px;
    text-align: center;
  }

  .empty-runtime-box p {
    color: var(--color-boulder-500);
    font-size: 13px;
    margin: 0;
  }

  .runtime-footer-meta {
    border-top: 1px solid var(--color-boulder-200);
    display: grid;
    gap: 16px;
    grid-template-columns: 1fr 1fr;
    padding-top: 14px;
  }

  .meta-field {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .meta-field-label {
    color: var(--color-boulder-500);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
  }

  .meta-field-value {
    color: var(--color-boulder-900);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
    font-weight: 600;
  }

  @media (max-width: 720px) {
    .editor-header {
      flex-direction: column;
    }

    .runtime-footer-meta {
      grid-template-columns: 1fr;
    }
  }
</style>
