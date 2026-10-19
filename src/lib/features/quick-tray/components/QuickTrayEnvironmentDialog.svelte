<script lang="ts">
  import { Check, Layers } from "@lucide/svelte";
  import { Dialog } from "@ark-ui/svelte/dialog";

  type Props = {
    open: boolean;
    name: string;
    onCreate: () => void;
  };

  let { open = $bindable(), name = $bindable(), onCreate }: Props = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/40 backdrop-blur-xs transition-opacity" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="bg-white border border-boulder-200 rounded-xl shadow-2xl p-5 max-w-sm w-full flex flex-col gap-3 outline-none"
    >
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 bg-east-bay-50 border border-east-bay-200 rounded-lg flex items-center justify-center text-east-bay-700 shrink-0"
        >
          <Layers size={20} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="text-sm font-bold text-boulder-950">Nuevo entorno</Dialog.Title>
          <Dialog.Description class="text-xs text-boulder-500">
            Crea un entorno aislado para gestionar sus variables.
          </Dialog.Description>
        </div>
      </div>

      <div class="flex flex-col gap-1.5 mt-1">
        <label class="text-xs font-semibold text-boulder-700" for="dialog-prof-name"
          >Nombre del entorno</label
        >
        <input
          id="dialog-prof-name"
          class="h-9 px-3 bg-white border border-boulder-300 focus:border-east-bay-400 rounded-md text-xs text-boulder-900 outline-none transition-all placeholder:text-boulder-400 focus:ring-2 focus:ring-east-bay-400/20"
          placeholder="EJ: Staging, Local, QA"
          bind:value={name}
        />
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
          disabled={!name.trim()}
          onclick={onCreate}
        >
          <Check size={14} strokeWidth={2.2} />
          <span>Crear entorno</span>
        </button>
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
