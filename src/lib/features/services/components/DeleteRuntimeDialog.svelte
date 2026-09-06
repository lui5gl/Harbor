<script lang="ts">
  import { AlertTriangle, Trash2 } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type DeleteRuntimeDialogProps = {
    open: boolean;
    serviceLabel: string;
    version: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };

  let {
    open = $bindable(),
    serviceLabel,
    version,
    onOpenChange,
    onConfirm
  }: DeleteRuntimeDialogProps = $props();

  let inputValue = $state("");
  const confirmKeyword = "CONFIRMAR";
  let isMatch = $derived(inputValue.trim().toUpperCase() === confirmKeyword);

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
            <AlertDialog.Title class="dialog-title">
              Delete {serviceLabel} {version}?
            </AlertDialog.Title>
            <AlertDialog.Description class="dialog-description">
              This will permanently delete the installed binaries and files for {serviceLabel} {version} from your Harbor runtimes directory.
            </AlertDialog.Description>
          </div>
        </div>

        <div class="confirmation-box">
          <label class="confirmation-label" for="runtime-delete-confirmation-input">
            To confirm, type <span class="keyword-highlight">{confirmKeyword}</span> below:
          </label>
          <input
            id="runtime-delete-confirmation-input"
            class="confirmation-input"
            type="text"
            bind:value={inputValue}
            placeholder={confirmKeyword}
            autocomplete="off"
            spellcheck="false"
          />
        </div>

        <div class="dialog-actions">
          <AlertDialog.Cancel class="btn-cancel">Cancel</AlertDialog.Cancel>
          <AlertDialog.Action
            class="btn-delete"
            type="submit"
            disabled={!isMatch}
            onclick={() => { if (isMatch) onConfirm(); }}
          >
            <Trash2 size={14} strokeWidth={2} aria-hidden="true" />
            <span>Delete version</span>
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
    border-radius: 12px;
    box-shadow: 0 20px 48px rgb(11 11 11 / 22%);
    box-sizing: border-box;
    left: 50%;
    max-width: 480px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: calc(100% - 32px);
    z-index: 101;
  }

  .dialog-form {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .dialog-header {
    align-items: flex-start;
    display: flex;
    gap: 16px;
  }

  .alert-icon-box {
    align-items: center;
    background: #fef2f2;
    border: 1px solid #fee2e2;
    border-radius: 10px;
    color: #dc2626;
    display: flex;
    flex-shrink: 0;
    height: 40px;
    justify-content: center;
    width: 40px;
  }

  .dialog-heading {
    min-width: 0;
  }

  :global(.dialog-title) {
    color: var(--color-boulder-950);
    font-size: 16px;
    font-weight: 650;
    line-height: 1.3;
    margin: 0;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 13.5px;
    line-height: 1.5;
    margin: 6px 0 0;
  }

  .confirmation-box {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 12px 14px;
  }

  .confirmation-label {
    color: var(--color-boulder-700);
    font-size: 12.5px;
    font-weight: 500;
  }

  .keyword-highlight {
    color: var(--color-boulder-950);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-weight: 700;
  }

  .confirmation-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-950);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 13.5px;
    font-weight: 600;
    outline: none;
    padding: 8px 10px;
    transition: border-color 150ms ease;
    width: 100%;
  }

  .confirmation-input:focus {
    border-color: #dc2626;
  }

  .dialog-actions {
    display: flex;
    gap: 10px;
    justify-content: flex-end;
  }

  :global(.btn-cancel) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-700);
    cursor: pointer;
    font: inherit;
    font-size: 13px;
    font-weight: 600;
    min-height: 36px;
    padding: 0 14px;
  }

  :global(.btn-cancel:hover) {
    background: var(--color-boulder-50);
    color: var(--color-boulder-950);
  }

  :global(.btn-delete) {
    align-items: center;
    background: #dc2626;
    border: 0;
    border-radius: 6px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 13px;
    font-weight: 600;
    gap: 6px;
    justify-content: center;
    min-height: 36px;
    padding: 0 16px;
    transition: background-color 150ms ease;
  }

  :global(.btn-delete:hover:not(:disabled)) {
    background: #b91c1c;
  }

  :global(.btn-delete:disabled) {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>
