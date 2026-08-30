<script lang="ts">
  import { ShieldAlert } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type ProductionActivationDialogProps = {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };

  let { open = $bindable(), onOpenChange, onConfirm }: ProductionActivationDialogProps = $props();
</script>

<AlertDialog.Root bind:open onOpenChange={onOpenChange}>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="confirmation-dialog">
      <div class="warning-icon" aria-hidden="true"><ShieldAlert size={22} strokeWidth={2} /></div>
      <AlertDialog.Title class="confirmation-dialog-title">Activate production profile?</AlertDialog.Title>
      <AlertDialog.Description class="confirmation-dialog-description">This may connect your development tools to production services. Confirm only when this is intentional.</AlertDialog.Description>
      <div class="dialog-actions">
        <AlertDialog.Cancel class="secondary-button">Cancel</AlertDialog.Cancel>
        <AlertDialog.Action class="primary-button warning-button" onclick={onConfirm}>Activate production</AlertDialog.Action>
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
    background: #fff1dc;
    border-radius: 6px;
    color: #965d00;
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

  .dialog-actions {
    align-items: center;
    display: flex;
    gap: 10px;
    justify-content: flex-end;
    margin-top: 24px;
  }

  :global(.primary-button),
  :global(.secondary-button) {
    align-items: center;
    border: 0;
    border-radius: 6px;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-weight: 650;
    justify-content: center;
  }

  :global(.primary-button) {
    background: var(--color-east-bay-900);
    color: #ffffff;
    min-height: 38px;
    padding: 0 14px;
  }

  :global(.secondary-button) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
    min-height: 36px;
    padding: 0 12px;
  }

  :global(.warning-button) {
    background: #9b5c00;
  }

  :global(.warning-button:hover) {
    background: #764600;
  }
</style>
