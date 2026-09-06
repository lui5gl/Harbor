<script lang="ts">
  import { Loader2, X } from "@lucide/svelte";
  import { Dialog } from "bits-ui";
  import InstallVersionList from "./InstallVersionList.svelte";

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
    onInstall,
  }: InstallVersionDialogProps = $props();

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
              Downloading & installing {serviceLabel}
              {installingVersion}...
            </span>
            <span class="installing-percent">{installProgress}%</span>
          </div>
          <div class="progress-track" aria-hidden="true">
            <div class="progress-bar" style={`width: ${Math.max(4, installProgress)}%`}></div>
          </div>
        </div>
      {/if}

      <InstallVersionList
        {serviceLabel}
        {availableVersions}
        {installedVersions}
        {isInstalling}
        onInstall={handleInstallClick}
      />
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
    transition:
      background-color 150ms ease,
      color 150ms ease;
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
</style>
