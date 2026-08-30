<script lang="ts">
  import { AlertTriangle, Trash2 } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type DeleteConfirmationDialogProps = {
    open: boolean;
    title: string;
    description: string;
    actionLabel?: string;
    confirmKeyword?: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };

  let {
    open = $bindable(),
    title,
    description,
    actionLabel = "Delete",
    confirmKeyword,
    onOpenChange,
    onConfirm
  }: DeleteConfirmationDialogProps = $props();

  let inputValue = $state("");
  let hasTypedConfirmation = $derived(Boolean(confirmKeyword));
  let isMatch = $derived(
    !hasTypedConfirmation || inputValue.trim().toUpperCase() === confirmKeyword?.toUpperCase()
  );

  $effect(() => {
    if (!open) inputValue = "";
  });

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!isMatch) return;
    onConfirm();
  }
</script>

<AlertDialog.Root bind:open {onOpenChange}>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="dialog-content">
      <form onsubmit={handleSubmit} class="dialog-form">
        <div class="dialog-header">
          <div class="alert-icon-box" aria-hidden="true">
            <AlertTriangle size={20} strokeWidth={2.2} />
          </div>
          <div class="dialog-heading">
            <AlertDialog.Title class="dialog-title">{title}</AlertDialog.Title>
            <AlertDialog.Description class="dialog-description">
              {description}
            </AlertDialog.Description>
          </div>
        </div>

        {#if hasTypedConfirmation}
          <div class="confirmation-box">
            <label class="confirmation-label" for="delete-confirmation-input">
              To confirm, type <span class="keyword-highlight">{confirmKeyword}</span> below:
            </label>
            <input
              id="delete-confirmation-input"
              class="confirmation-input"
              type="text"
              bind:value={inputValue}
              placeholder={confirmKeyword}
              autocomplete="off"
              spellcheck="false"
            />
          </div>
        {/if}

        <div class="dialog-actions">
          <AlertDialog.Cancel class="btn-cancel">Cancel</AlertDialog.Cancel>
          <AlertDialog.Action
            class="btn-delete"
            type="submit"
            disabled={!isMatch}
            onclick={() => { if (isMatch) onConfirm(); }}
          >
            <Trash2 size={14} strokeWidth={2} aria-hidden="true" />
            <span>{actionLabel}</span>
          </AlertDialog.Action>
        </div>
      </form>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>

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
    border-radius: 10px;
    box-shadow:
      0 20px 25px -5px rgb(0 0 0 / 15%),
      0 8px 10px -6px rgb(0 0 0 / 10%);
    box-sizing: border-box;
    left: 50%;
    max-width: 440px;
    padding: 20px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(calc(100% - 32px), 440px);
    z-index: 101;
  }

  .dialog-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin: 0;
  }

  .dialog-header {
    display: flex;
    gap: 14px;
  }

  .alert-icon-box {
    align-items: center;
    background: #fef2f2;
    border: 1px solid #fee2e2;
    border-radius: 8px;
    color: #dc2626;
    display: flex;
    flex-shrink: 0;
    height: 40px;
    justify-content: center;
    width: 40px;
  }

  .dialog-heading {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  :global(.dialog-title) {
    color: var(--color-boulder-950);
    font-size: 16px;
    font-weight: 700;
    letter-spacing: -0.01em;
    margin: 0;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 13.5px;
    line-height: 1.45;
    margin: 0;
  }

  .confirmation-box {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px;
  }

  .confirmation-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 500;
  }

  .keyword-highlight {
    background: var(--color-boulder-200);
    border-radius: 3px;
    color: var(--color-boulder-950);
    font-family: ui-monospace, monospace;
    font-size: 11.5px;
    font-weight: 700;
    padding: 1px 5px;
  }

  .confirmation-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 13px;
    height: 34px;
    outline: none;
    padding: 0 10px;
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
    width: 100%;
  }

  .confirmation-input:focus {
    border-color: #dc2626;
    box-shadow: 0 0 0 3px rgb(220 38 38 / 15%);
  }

  .dialog-actions {
    align-items: center;
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 4px;
  }

  :global(.btn-cancel),
  :global(.btn-delete) {
    align-items: center;
    border-radius: 6px;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 13px;
    font-weight: 600;
    gap: 6px;
    height: 34px;
    justify-content: center;
    padding: 0 14px;
    transition: background 0.12s ease, border-color 0.12s ease, opacity 0.12s ease;
  }

  :global(.btn-cancel) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    color: var(--color-boulder-700);
  }

  :global(.btn-cancel:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-400);
    color: var(--color-boulder-900);
  }

  :global(.btn-delete) {
    background: #dc2626;
    border: 1px solid #dc2626;
    color: #ffffff;
  }

  :global(.btn-delete:hover:not(:disabled)) {
    background: #b91c1c;
    border-color: #b91c1c;
  }

  :global(.btn-delete:disabled) {
    cursor: not-allowed;
    opacity: 0.45;
  }
</style>