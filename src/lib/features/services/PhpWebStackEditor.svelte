<script lang="ts">
  import {
    Activity,
    Check,
    Download,
    FolderGit2,
    HardDrive,
    Info,
    Play,
    Plus,
    RefreshCw,
    Server,
    ShieldCheck,
    Square,
    Terminal,
    Trash2
  } from "@lucide/svelte";
  import { Button, Tooltip } from "bits-ui";
  import DeleteRuntimeDialog from "./DeleteRuntimeDialog.svelte";
  import InstallVersionDialog from "./InstallVersionDialog.svelte";
  import {
    cleanVersion,
    getCompatibleApacheVersions,
    parseVersionString,
    type VersionItem
  } from "./types";

  type PhpWebStackEditorProps = {
    activePhpVersion: string | null;
    installedPhpVersions: string[];
    availablePhpVersions: string[];
    activeApacheVersion: string | null;
    installedApacheVersions: string[];
    availableApacheVersions: string[];
    isPhpRunning: boolean;
    fastCgiAddress: string;
    isInstalling: boolean;
    installProgress: number;
    installingService: string;
    installingVersion: string;
    installError: string;
    onSelectPhpVersion: (version: string) => Promise<void>;
    onTogglePhpFastCgi: () => Promise<void>;
    onInstallVersion: (service: "PHP" | "Apache", version: string) => Promise<void>;
    onDeleteVersion: (service: "PHP" | "Apache", version: string) => Promise<void>;
  };

  let {
    activePhpVersion,
    installedPhpVersions,
    availablePhpVersions,
    activeApacheVersion,
    installedApacheVersions,
    availableApacheVersions,
    isPhpRunning,
    fastCgiAddress,
    isInstalling,
    installProgress,
    installingService,
    installingVersion,
    installError,
    onSelectPhpVersion,
    onTogglePhpFastCgi,
    onInstallVersion,
    onDeleteVersion
  }: PhpWebStackEditorProps = $props();

  let isPhpInstallDialogOpen = $state(false);
  let isApacheInstallDialogOpen = $state(false);
  let pendingDelete = $state<{ service: "PHP" | "Apache"; version: string } | null>(null);

  let compatibility = $derived(
    getCompatibleApacheVersions(activePhpVersion, availableApacheVersions)
  );

  let cleanActivePhp = $derived(
    activePhpVersion ? cleanVersion(activePhpVersion) : null
  );

  let activePhpChannel = $derived(() => {
    if (!activePhpVersion) return "";
    const raw = availablePhpVersions.find((v) => cleanVersion(v) === cleanActivePhp);
    return raw ? parseVersionString(raw).channel : "";
  });

  function requestDelete(service: "PHP" | "Apache", version: string) {
    pendingDelete = { service, version: cleanVersion(version) };
  }

  function confirmDelete() {
    if (pendingDelete) {
      void onDeleteVersion(pendingDelete.service, pendingDelete.version);
      pendingDelete = null;
    }
  }
</script>

<section class="editor-panel" aria-labelledby="web-stack-title">
  <!-- Header -->
  <header class="editor-header">
    <div class="header-main">
      <div class="header-titles">
        <p class="eyebrow">Web Server & FastCGI Stack</p>
        <h2 id="web-stack-title">PHP & Web Server</h2>
      </div>
      <p class="header-sub">
        Harbor orchestrates PHP FastCGI and the Apache web server together for seamless local development.
      </p>
    </div>

    <div class="header-controls">
      <div class={`status-pill${isPhpRunning ? " running" : ""}`}>
        <span class="status-indicator" aria-hidden="true"></span>
        <span class="status-label">FastCGI: {isPhpRunning ? fastCgiAddress : "Stopped"}</span>
      </div>

      <Button.Root
        class={`toggle-service-btn${isPhpRunning ? " is-stop" : ""}`}
        type="button"
        disabled={!activePhpVersion}
        onclick={() => void onTogglePhpFastCgi()}
      >
        {#if isPhpRunning}
          <Square size={14} strokeWidth={2.4} aria-hidden="true" />
          <span>Stop FastCGI</span>
        {:else}
          <Play size={14} strokeWidth={2.4} aria-hidden="true" />
          <span>Start FastCGI</span>
        {/if}
      </Button.Root>
    </div>
  </header>

  <!-- Stack Layout -->
  <div class="stack-grid">
    <!-- PHP Runtime Card -->
    <div class="stack-card">
      <div class="card-header">
        <div class="card-title-group">
          <div class="card-icon php-icon" aria-hidden="true">
            <Terminal size={17} strokeWidth={2.2} />
          </div>
          <div>
            <h3>PHP Runtime (CLI & FastCGI)</h3>
            <p>Select which PHP version runs FastCGI and aliases to the system PATH.</p>
          </div>
        </div>

        <Button.Root
          class="secondary-button"
          type="button"
          onclick={() => (isPhpInstallDialogOpen = true)}
        >
          <Plus size={15} strokeWidth={2.2} aria-hidden="true" />
          <span>Add PHP</span>
        </Button.Root>
      </div>

      {#if installedPhpVersions.length === 0}
        <div class="empty-runtime-box">
          <p>No PHP versions are installed yet.</p>
          <Button.Root
            class="primary-button-sm"
            type="button"
            onclick={() => (isPhpInstallDialogOpen = true)}
          >
            <Download size={14} strokeWidth={2} aria-hidden="true" />
            <span>Install PHP Version</span>
          </Button.Root>
        </div>
      {:else}
        <div class="installed-versions-list">
          {#each installedPhpVersions as phpVer (phpVer)}
            {@const isActive = cleanVersion(phpVer) === cleanActivePhp}
            <div class={`runtime-row${isActive ? " is-active" : ""}`}>
              <div class="runtime-info">
                <span class="version-name">PHP {phpVer}</span>
                {#if isActive}
                  <span class="active-tag">Active & PATH</span>
                {/if}
              </div>

              <div class="runtime-actions">
                {#if !isActive}
                  <button
                    type="button"
                    class="btn-activate"
                    onclick={() => void onSelectPhpVersion(phpVer)}
                  >
                    <span>Set as active</span>
                  </button>
                {/if}

                <button
                  type="button"
                  class="btn-delete-icon"
                  title={`Delete PHP ${phpVer}`}
                  aria-label={`Delete PHP ${phpVer}`}
                  onclick={() => requestDelete("PHP", phpVer)}
                >
                  <Trash2 size={15} strokeWidth={2} aria-hidden="true" />
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Apache Web Server & Compatibility Card -->
    <div class="stack-card">
      <div class="card-header">
        <div class="card-title-group">
          <div class="card-icon apache-icon" aria-hidden="true">
            <Server size={17} strokeWidth={2.2} />
          </div>
          <div>
            <h3>Apache Web Server (FastCGI Companion)</h3>
            <p>Local HTTP server configured to route scripts to active PHP FastCGI.</p>
          </div>
        </div>

        <Button.Root
          class="secondary-button"
          type="button"
          onclick={() => (isApacheInstallDialogOpen = true)}
        >
          <Plus size={15} strokeWidth={2.2} aria-hidden="true" />
          <span>Add Apache</span>
        </Button.Root>
      </div>

      <!-- Compatibility Banner -->
      <div class="compatibility-banner">
        <div class="banner-icon-box" aria-hidden="true">
          <ShieldCheck size={18} strokeWidth={2.2} />
        </div>
        <div class="banner-content">
          <span class="banner-title">Compatibility Engine</span>
          <p class="banner-text">{compatibility.compatibilityNote}</p>
        </div>
      </div>

      <div class="installed-versions-list">
        {#if installedApacheVersions.length === 0}
          <div class="empty-runtime-box">
            <p>No Apache version installed yet. Choose a compatible build below.</p>
            {#if compatibility.recommended.length > 0}
              <Button.Root
                class="primary-button-sm"
                type="button"
                disabled={isInstalling}
                onclick={() => void onInstallVersion("Apache", compatibility.recommended[0])}
              >
                <Download size={14} strokeWidth={2} aria-hidden="true" />
                <span>Install Recommended Apache ({cleanVersion(compatibility.recommended[0])})</span>
              </Button.Root>
            {/if}
          </div>
        {:else}
          {#each installedApacheVersions as apacheVer (apacheVer)}
            <div class="runtime-row is-active">
              <div class="runtime-info">
                <span class="version-name">Apache {apacheVer}</span>
                <span class="compatible-tag">
                  <Check size={12} strokeWidth={2.4} aria-hidden="true" />
                  <span>Compatible with PHP {cleanActivePhp ?? "8.x"}</span>
                </span>
              </div>

              <div class="runtime-actions">
                <button
                  type="button"
                  class="btn-delete-icon"
                  title={`Delete Apache ${apacheVer}`}
                  aria-label={`Delete Apache ${apacheVer}`}
                  onclick={() => requestDelete("Apache", apacheVer)}
                >
                  <Trash2 size={15} strokeWidth={2} aria-hidden="true" />
                </button>
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <!-- Path & Configuration details -->
      <div class="stack-footer-meta">
        <div class="meta-field">
          <span class="meta-field-label">Document Root</span>
          <span class="meta-field-value">C:\Harbor\www</span>
        </div>
        <div class="meta-field">
          <span class="meta-field-label">Port & Proxy</span>
          <span class="meta-field-value">HTTP :8080 &rarr; FastCGI :9070</span>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- Dialogs -->
<InstallVersionDialog
  bind:open={isPhpInstallDialogOpen}
  serviceLabel="PHP"
  availableVersions={availablePhpVersions}
  installedVersions={installedPhpVersions}
  {isInstalling}
  {installProgress}
  {installingVersion}
  {installError}
  onOpenChange={(open) => { isPhpInstallDialogOpen = open; }}
  onInstall={(version) => void onInstallVersion("PHP", version)}
/>

<InstallVersionDialog
  bind:open={isApacheInstallDialogOpen}
  serviceLabel="Apache"
  availableVersions={availableApacheVersions}
  installedVersions={installedApacheVersions}
  {isInstalling}
  {installProgress}
  {installingVersion}
  {installError}
  onOpenChange={(open) => { isApacheInstallDialogOpen = open; }}
  onInstall={(version) => void onInstallVersion("Apache", version)}
/>

<DeleteRuntimeDialog
  open={Boolean(pendingDelete)}
  serviceLabel={pendingDelete?.service ?? ""}
  version={pendingDelete?.version ?? ""}
  onOpenChange={(open) => { if (!open) pendingDelete = null; }}
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

  .status-pill.running {
    background: #ecfdf5;
    border-color: #a7f3d0;
    color: #065f46;
  }

  .status-indicator {
    background: #9ca3af;
    border-radius: 50%;
    height: 7px;
    width: 7px;
  }

  .status-pill.running .status-indicator {
    background: #10b981;
    box-shadow: 0 0 0 3px rgb(16 185 129 / 20%);
  }

  :global(.toggle-service-btn) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 6px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 13px;
    font-weight: 600;
    gap: 7px;
    height: 36px;
    padding: 0 14px;
    transition: background-color 150ms ease;
  }

  :global(.toggle-service-btn:hover:not(:disabled)) {
    background: var(--color-east-bay-950);
  }

  :global(.toggle-service-btn.is-stop) {
    background: #b91c1c;
  }

  :global(.toggle-service-btn.is-stop:hover:not(:disabled)) {
    background: #991b1b;
  }

  :global(.toggle-service-btn:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .stack-grid {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .stack-card {
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
    gap: 12px;
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

  .php-icon {
    background: var(--color-east-bay-100);
    color: var(--color-east-bay-800);
  }

  .apache-icon {
    background: #fef3c7;
    color: #92400e;
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

  :global(.secondary-button) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12.5px;
    font-weight: 600;
    gap: 6px;
    height: 32px;
    padding: 0 12px;
    transition: background-color 150ms ease, border-color 150ms ease;
  }

  :global(.secondary-button:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
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
    height: 32px;
    padding: 0 14px;
  }

  :global(.primary-button-sm:hover) {
    background: var(--color-east-bay-950);
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
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 13.5px;
    font-weight: 650;
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

  .compatible-tag {
    align-items: center;
    background: #ecfdf5;
    border-radius: 999px;
    color: #065f46;
    display: inline-flex;
    font-size: 11px;
    font-weight: 600;
    gap: 4px;
    padding: 2px 8px;
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
    transition: background-color 150ms ease, color 150ms ease;
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
    transition: background-color 150ms ease, color 150ms ease;
    width: 28px;
  }

  .btn-delete-icon:hover {
    background: #fef2f2;
    color: #dc2626;
  }

  .compatibility-banner {
    align-items: flex-start;
    background: #f0fdf4;
    border: 1px solid #bbf7d0;
    border-radius: 6px;
    display: flex;
    gap: 12px;
    padding: 12px 14px;
  }

  .banner-icon-box {
    color: #16a34a;
    flex-shrink: 0;
    margin-top: 1px;
  }

  .banner-content {
    min-width: 0;
  }

  .banner-title {
    color: #166534;
    display: block;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }

  .banner-text {
    color: #15803d;
    font-size: 12.5px;
    line-height: 1.45;
    margin: 2px 0 0;
  }

  .stack-footer-meta {
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

    .stack-footer-meta {
      grid-template-columns: 1fr;
    }
  }
</style>
