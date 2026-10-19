<script lang="ts">
  import { ShieldAlert, Trash2 } from "@lucide/svelte";
  import { Dialog } from "@ark-ui/svelte/dialog";

  type Props = {
    open: boolean;
    kind: "activation" | "delete";
    onConfirm: () => void;
  };

  let { open = $bindable(), kind, onConfirm }: Props = $props();
  let isActivation = $derived(kind === "activation");
</script>

<Dialog.Root bind:open role="alertdialog">
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/40 backdrop-blur-xs transition-opacity" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="bg-white border border-boulder-200 rounded-xl shadow-2xl p-5 max-w-sm w-full flex flex-col gap-3 outline-none"
    >
      <div class="flex items-center gap-3">
        <div
          class={`w-10 h-10 rounded-full flex items-center justify-center shrink-0 ${isActivation ? "bg-amber-100 text-amber-800" : "bg-red-100 text-red-600"}`}
        >
          {#if isActivation}
            <ShieldAlert size={20} strokeWidth={2.2} />
          {:else}
            <Trash2 size={20} strokeWidth={2.2} />
          {/if}
        </div>
        <div>
          <Dialog.Title class="text-sm font-bold text-boulder-950">
            {isActivation ? "¿Activar este entorno?" : "¿Eliminar variable?"}
          </Dialog.Title>
        </div>
      </div>
      <Dialog.Description class="text-xs text-boulder-600 leading-relaxed">
        {isActivation
          ? "Esto cargará las variables del entorno en el sistema y en PowerShell. Confirma para continuar."
          : "Esta acción eliminará la variable del entorno. Esta acción no se puede deshacer."}
      </Dialog.Description>
      <div class="flex items-center justify-end gap-2 mt-2">
        <Dialog.CloseTrigger
          class="h-8 px-3 text-xs font-semibold text-boulder-700 hover:text-boulder-950 bg-boulder-100 hover:bg-boulder-200 rounded-lg transition-colors cursor-pointer"
        >
          Cancelar
        </Dialog.CloseTrigger>
        <button
          type="button"
          class={`h-8 px-3 text-xs font-semibold text-white rounded-lg transition-colors cursor-pointer shadow-xs ${
            isActivation ? "bg-amber-700 hover:bg-amber-800" : "bg-red-600 hover:bg-red-700"
          }`}
          onclick={() => {
            onConfirm();
            open = false;
          }}
        >
          {isActivation ? "Activar entorno" : "Eliminar"}
        </button>
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
