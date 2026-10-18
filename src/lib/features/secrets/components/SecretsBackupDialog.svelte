<script lang="ts">
  import { Button, Dialog } from "bits-ui";
  import { Download, Upload } from "@lucide/svelte";
  import { decryptBackup, encryptBackup } from "../application/secretsBackup";
  import type { SecretsConfiguration } from "../types";

  let {
    open = $bindable(),
    mode,
    configuration,
    onRestore,
  }: {
    open: boolean;
    mode: "export" | "restore";
    configuration: () => SecretsConfiguration;
    onRestore: (configuration: SecretsConfiguration) => Promise<boolean>;
  } = $props();
  let password = $state("");
  let confirmation = $state("");
  let file = $state<File | null>(null);
  let confirmed = $state(false);
  let busy = $state(false);
  let error = $state("");

  $effect(() => {
    if (!open) {
      password = "";
      confirmation = "";
      file = null;
      confirmed = false;
      error = "";
    }
  });

  async function submit() {
    if (busy) return;
    busy = true;
    error = "";
    try {
      if (mode === "export") {
        if (password !== confirmation) throw new Error("Passphrases do not match");
        const contents = await encryptBackup(configuration(), password);
        const url = URL.createObjectURL(new Blob([contents], { type: "application/json" }));
        const anchor = document.createElement("a");
        anchor.href = url;
        anchor.download = `harbor-secrets-${new Date().toISOString().slice(0, 10)}.harbor`;
        anchor.click();
        window.setTimeout(() => URL.revokeObjectURL(url), 1000);
      } else {
        if (!file || !confirmed) throw new Error("Select a backup and confirm replacement");
        if (file.size > 8_388_608) throw new Error("Backup file is too large");
        const restored = await decryptBackup(await file.text(), password);
        if (!(await onRestore(restored)))
          throw new Error(
            "Unable to save the restored backup. Check the Secrets error before continuing.",
          );
      }
      open = false;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    } finally {
      busy = false;
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content
      class="backup-dialog"
      onInteractOutside={(event) => {
        if (busy) event.preventDefault();
      }}
      onEscapeKeydown={(event) => {
        if (busy) event.preventDefault();
      }}
    >
      <Dialog.Title class="backup-title"
        >{mode === "export" ? "Encrypted backup" : "Restore encrypted backup"}</Dialog.Title
      >
      <Dialog.Description class="backup-description"
        >{mode === "export"
          ? "Keep the passphrase separately. It cannot be recovered by Harbor."
          : "Restoring replaces all projects. Published environments must be deactivated first."}</Dialog.Description
      >
      {#if mode === "restore"}<label
          >Backup file<input
            type="file"
            accept=".harbor,application/json"
            disabled={busy}
            onchange={(event) => (file = event.currentTarget.files?.[0] ?? null)}
          /></label
        >{/if}
      <label
        >Passphrase<input
          type="password"
          autocomplete="off"
          bind:value={password}
          disabled={busy}
        /></label
      >
      {#if mode === "export"}<label
          >Confirm passphrase<input
            type="password"
            autocomplete="off"
            bind:value={confirmation}
            disabled={busy}
          /></label
        >{/if}
      {#if mode === "restore"}<label class="backup-confirm"
          ><input type="checkbox" bind:checked={confirmed} disabled={busy} />Replace all current
          projects and environments</label
        >{/if}
      {#if error}<p role="alert">{error}</p>{/if}
      <div class="backup-actions">
        <Dialog.Close disabled={busy}>Cancel</Dialog.Close><Button.Root
          type="button"
          disabled={busy ||
            !password ||
            (mode === "export"
              ? password.length < 12 || password !== confirmation
              : !file || !confirmed)}
          onclick={() => void submit()}
          >{#if mode === "export"}<Download size={15} />{:else}<Upload size={15} />{/if}{busy
            ? "Processing..."
            : mode === "export"
              ? "Download backup"
              : "Restore backup"}</Button.Root
        >
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.backup-dialog) {
    background: white;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 14px;
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(460px, calc(100vw - 32px));
    max-height: calc(100dvh - 32px);
    overflow: auto;
    padding: 24px;
    z-index: 101;
  }
  :global(.backup-title) {
    font-size: 18px;
    font-weight: 700;
  }
  :global(.backup-description) {
    font-size: 13px;
    color: var(--color-boulder-600);
    margin: 0;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 6px;
    font-size: 12px;
  }
  input:not([type="checkbox"]) {
    box-sizing: border-box;
    width: 100%;
    padding: 9px;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    font: inherit;
  }
  .backup-confirm {
    flex-direction: row;
    align-items: center;
  }
  .backup-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
  .backup-actions :global(button) {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    padding: 8px 12px;
    background: white;
    cursor: pointer;
  }
  p[role="alert"] {
    color: #b91c1c;
    font-size: 12px;
  }
</style>
