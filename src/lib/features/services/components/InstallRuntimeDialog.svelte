<script lang="ts">
  import { Download } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type Props = {
    open: boolean;
    serviceName: string;
    version: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };

  let { open = $bindable(), serviceName, version, onOpenChange, onConfirm }: Props = $props();
</script>

<AlertDialog.Root bind:open {onOpenChange}>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="dialog-content">
      <AlertDialog.Title class="dialog-title">
        Descargar {serviceName}
        {version}?
      </AlertDialog.Title>
      <AlertDialog.Description class="dialog-description">
        El runtime se instalará en la carpeta de runtimes de Harbor.
      </AlertDialog.Description>
      <div class="dialog-actions">
        <AlertDialog.Cancel class="btn-cancel">Cancelar</AlertDialog.Cancel>
        <AlertDialog.Action class="btn-confirm" onclick={onConfirm}>
          <Download size={14} strokeWidth={2} aria-hidden="true" />
          <span>Confirmar</span>
        </AlertDialog.Action>
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
    position: fixed;
    z-index: 20;
  }

  :global(.dialog-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow: 0 18px 40px rgb(11 11 11 / 20%);
    max-width: 360px;
    padding: 24px;
    width: calc(100% - 32px);
  }

  :global(.dialog-title) {
    color: var(--color-boulder-950);
    font-size: 16px;
    margin: 0;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.45;
    margin: 10px 0 20px;
  }

  .dialog-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  :global(.btn-cancel),
  :global(.btn-confirm) {
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    font: inherit;
    font-size: 13px;
    min-height: 36px;
    padding: 0 14px;
  }

  :global(.btn-confirm) {
    align-items: center;
    background: var(--color-east-bay-500);
    color: #ffffff;
    display: inline-flex;
    gap: 6px;
  }
</style>
