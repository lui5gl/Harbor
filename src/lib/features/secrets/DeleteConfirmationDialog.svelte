<script lang="ts">
  import { AlertTriangle } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type DeleteConfirmationDialogProps = {
    open: boolean;
    title: string;
    description: string;
    actionLabel: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };

  let { open = $bindable(), title, description, actionLabel, onOpenChange, onConfirm }: DeleteConfirmationDialogProps = $props();
  let confirmation = $state("");
  let requiresTypedConfirmation = $derived(actionLabel === "Delete profile");
  let canConfirm = $derived(!requiresTypedConfirmation || confirmation.trim().toUpperCase() === "CONFIRMO");

  $effect(() => {
    if (!open) confirmation = "";
  });

  function confirmDeletion() {
    if (!canConfirm) return;
    onConfirm();
  }
</script>

<AlertDialog.Root bind:open onOpenChange={onOpenChange}>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="confirmation-dialog">
      <div class="warning-icon" aria-hidden="true"><AlertTriangle size={22} strokeWidth={2} /></div>
      <AlertDialog.Title class="confirmation-dialog-title">{title}</AlertDialog.Title>
      <AlertDialog.Description class="confirmation-dialog-description">{description}</AlertDialog.Description>
      {#if requiresTypedConfirmation}
        <label class="confirmation-label" for="profile-deletion-confirmation">Type CONFIRMO to delete this profile</label>
        <input id="profile-deletion-confirmation" class="confirmation-input" bind:value={confirmation} autocomplete="off" />
      {/if}
      <div class="dialog-actions">
        <AlertDialog.Cancel class="secondary-button">Cancel</AlertDialog.Cancel>
        <AlertDialog.Action class="danger-button" disabled={!canConfirm} onclick={confirmDeletion}>{actionLabel}</AlertDialog.Action>
      </div>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>

<style>
  :global(.modal-backdrop) {
    align-items: center;
    background: rgb(11 11 11 / 35%);
    display: flex;
    inset: 0;
    justify-content: center;
    padding: 24px;
    position: fixed;
    z-index: 10;
  }

  :global(.confirmation-dialog) {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 20px 60px rgb(11 11 11 / 20%);
    left: 50%;
    max-width: 420px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(calc(100% - 48px), 420px);
    z-index: 11;
  }

  .warning-icon {
    align-items: center;
    background: #fff3f1;
    border-radius: 6px;
    color: #a13f32;
    display: flex;
    height: 42px;
    justify-content: center;
    width: 42px;
  }

  :global(.confirmation-dialog-title) {
    color: var(--color-boulder-950);
    font-size: 19px;
    font-weight: 650;
    margin-top: 18px;
  }

  :global(.confirmation-dialog-description) {
    color: var(--color-boulder-600);
    font-size: 14px;
    line-height: 1.5;
    margin-top: 10px;
  }

  .confirmation-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 650;
    margin-top: 12px;
  }

  .confirmation-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    min-height: 38px;
    outline: none;
    padding: 0 10px;
    width: 100%;
  }

  .confirmation-input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 16%);
  }

  .dialog-actions {
    align-items: center;
    display: flex;
    gap: 10px;
    justify-content: flex-end;
    margin-top: 24px;
  }

  :global(.secondary-button),
  :global(.danger-button) {
    align-items: center;
    border: 0;
    border-radius: 6px;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-weight: 650;
    justify-content: center;
  }

  :global(.secondary-button) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
    min-height: 36px;
    padding: 0 12px;
  }

  :global(.danger-button) {
    background: #a13f32;
    color: #ffffff;
    min-height: 38px;
    padding: 0 14px;
  }

  :global(.danger-button:hover) {
    background: #7f3026;
  }

  :global(.danger-button:disabled) {
    cursor: not-allowed;
    opacity: 0.45;
  }
</style>