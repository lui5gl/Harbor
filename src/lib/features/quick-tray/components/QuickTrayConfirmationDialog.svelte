<script lang="ts">
  import { ShieldAlert, Trash2 } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type Props = {
    open: boolean;
    kind: "production" | "delete";
    onConfirm: () => void;
  };

  let { open = $bindable(), kind, onConfirm }: Props = $props();
  let isProduction = $derived(kind === "production");
</script>

<AlertDialog.Root bind:open>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="dialog-content confirmation-dialog">
      <div class:danger={!isProduction} class="warning-icon-wrapper" aria-hidden="true">
        {#if isProduction}<ShieldAlert size={22} strokeWidth={2.2} />{:else}<Trash2
            size={22}
            strokeWidth={2.2}
          />{/if}
      </div>
      <AlertDialog.Title class="dialog-title">
        {isProduction ? "¿Activar entorno de producción?" : "¿Eliminar variable?"}
      </AlertDialog.Title>
      <AlertDialog.Description class="dialog-description">
        {isProduction
          ? "Esto cargará las variables de producción en el sistema y en PowerShell. Confirma solo si es intencional."
          : "Esta acción eliminará la variable del entorno. Esta acción no se puede deshacer."}
      </AlertDialog.Description>
      <div class="dialog-footer">
        <AlertDialog.Cancel class="secondary-button btn-sm">Cancelar</AlertDialog.Cancel>
        <AlertDialog.Action
          class={`primary-button btn-sm warning-action-btn${isProduction ? "" : " danger-action-btn"}`}
          onclick={onConfirm}
        >
          <span>{isProduction ? "Activar producción" : "Eliminar"}</span>
        </AlertDialog.Action>
      </div>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>
