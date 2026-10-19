<script lang="ts">
  import { Check } from "@lucide/svelte";
  import { Dialog } from "@ark-ui/svelte/dialog";

  type Props = {
    open: boolean;
    editingSecretId: number | null;
    formKey: string;
    formValue: string;
    formError: string;
    onSave: () => void;
  };

  let {
    open = $bindable(),
    editingSecretId,
    formKey = $bindable(),
    formValue = $bindable(),
    formError,
    onSave,
  }: Props = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/40 backdrop-blur-xs transition-opacity" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="bg-white border border-boulder-200 rounded-xl shadow-2xl p-5 max-w-sm w-full flex flex-col gap-3 outline-none"
    >
      <div class="dialog-header">
        <Dialog.Title class="text-sm font-bold text-boulder-950">
          {editingSecretId !== null ? "Editar variable" : "Nueva variable"}
        </Dialog.Title>
        <Dialog.Description class="text-xs text-boulder-500">
          Define la clave y el valor para el entorno seleccionado.
        </Dialog.Description>
      </div>

      {#if formError}
        <div
          class="bg-red-50 border border-red-200 text-red-700 text-xs rounded-md p-2 font-medium"
        >
          {formError}
        </div>
      {/if}

      <div class="flex flex-col gap-3">
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-semibold text-boulder-700" for="dialog-var-key"
            >Nombre de Clave (KEY)</label
          >
          <input
            id="dialog-var-key"
            class="h-9 px-3 bg-white border border-boulder-300 focus:border-east-bay-400 rounded-md text-xs font-mono font-semibold text-boulder-900 outline-none transition-all placeholder:text-boulder-400 focus:ring-2 focus:ring-east-bay-400/20"
            placeholder="EJ: DATABASE_URL, API_KEY"
            bind:value={formKey}
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-semibold text-boulder-700" for="dialog-var-value"
            >Valor (VALUE)</label
          >
          <input
            id="dialog-var-value"
            class="h-9 px-3 bg-white border border-boulder-300 focus:border-east-bay-400 rounded-md text-xs font-mono text-boulder-900 outline-none transition-all placeholder:text-boulder-400 focus:ring-2 focus:ring-east-bay-400/20"
            placeholder="Valor del secreto"
            bind:value={formValue}
          />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 mt-2">
        <Dialog.CloseTrigger
          class="h-8 px-3 text-xs font-semibold text-boulder-700 hover:text-boulder-950 bg-boulder-100 hover:bg-boulder-200 rounded-lg transition-colors cursor-pointer"
        >
          Cancelar
        </Dialog.CloseTrigger>
        <button
          type="button"
          class="h-8 px-3.5 bg-east-bay-900 hover:bg-east-bay-950 text-white rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-colors cursor-pointer shadow-xs disabled:opacity-50"
          disabled={!formKey.trim()}
          onclick={onSave}
        >
          <Check size={14} strokeWidth={2.2} />
          <span>Guardar variable</span>
        </button>
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
