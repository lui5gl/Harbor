<script lang="ts">
  import { ShieldAlert } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type ProductionActivationDialogProps = {
    open: boolean;
    environmentLabel: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };

  let { open = $bindable(), environmentLabel, onOpenChange, onConfirm }: ProductionActivationDialogProps = $props();
</script>

<AlertDialog.Root bind:open onOpenChange={onOpenChange}>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="confirmation-dialog">
      <div class="warning-icon" aria-hidden="true"><ShieldAlert size={22} strokeWidth={2} /></div>
      <AlertDialog.Title class="confirmation-dialog-title">Activate production environment?</AlertDialog.Title>
      <AlertDialog.Description class="confirmation-dialog-description">{environmentLabel} will become available to new PowerShell sessions. This may connect development tools to production services.</AlertDialog.Description>
      <div class="dialog-actions">
        <AlertDialog.Cancel class="secondary-button">Cancel</AlertDialog.Cancel>
        <AlertDialog.Action class="primary-button warning-button" onclick={onConfirm}>Activate production</AlertDialog.Action>
      </div>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>

<style>
  :global(.modal-backdrop) {
    background: rgb(11 11 11 / 45%);
    inset: 0;
    position: fixed;
    z-index: 100;
  }

  :global(.confirmation-dialog) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-shadow: 0 20px 60px rgb(11 11 11 / 20%);
    box-sizing: border-box;
    left: 50%;
    max-width: 420px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(calc(100% - 48px), 420px);
    z-index: 101;
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
