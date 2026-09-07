<script lang="ts">
  import { ShieldAlert, Trash2 } from "@lucide/svelte";
  import { AlertDialog } from "bits-ui";

  type Props = {
    open: boolean;
    kind: "activation" | "delete";
    onConfirm: () => void;
  };

  let { open = $bindable(), kind, onConfirm }: Props = $props();
  let isActivation = $derived(kind === "activation");
</script>

<AlertDialog.Root bind:open>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="dialog-content confirmation-dialog">
      <div class:danger={!isActivation} class="warning-icon-wrapper" aria-hidden="true">
        {#if isActivation}<ShieldAlert size={22} strokeWidth={2.2} />{:else}<Trash2
            size={22}
            strokeWidth={2.2}
          />{/if}
      </div>
      <AlertDialog.Title class="dialog-title">
        {isActivation ? "¿Activar este entorno?" : "¿Eliminar variable?"}
      </AlertDialog.Title>
      <AlertDialog.Description class="dialog-description">
        {isActivation
          ? "Esto cargará las variables del entorno en el sistema y en PowerShell. Confirma para continuar."
          : "Esta acción eliminará la variable del entorno. Esta acción no se puede deshacer."}
      </AlertDialog.Description>
      <div class="dialog-footer">
        <AlertDialog.Cancel class="secondary-button btn-sm">Cancelar</AlertDialog.Cancel>
        <AlertDialog.Action
          class={`primary-button btn-sm warning-action-btn${isActivation ? "" : " danger-action-btn"}`}
          onclick={onConfirm}
        >
          <span>{isActivation ? "Activar entorno" : "Eliminar"}</span>
        </AlertDialog.Action>
      </div>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>
