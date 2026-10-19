<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Download } from "@lucide/svelte";
  type Props = {
    open: boolean;
    serviceName: string;
    version: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };
  let { open = $bindable(), serviceName, version, onOpenChange, onConfirm }: Props = $props();
</script>

<Dialog.Root bind:open role="alertdialog" onOpenChange={(details) => onOpenChange(details.open)}>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/35 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="w-full max-w-sm rounded-xl border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
    >
      <Dialog.Title class="text-base font-bold text-boulder-950"
        >Descargar {serviceName} {version}?</Dialog.Title
      >
      <Dialog.Description class="my-3 text-[13px] leading-relaxed text-boulder-600"
        >El runtime se instalará en la carpeta de runtimes de Harbor.</Dialog.Description
      >
      <div class="flex justify-end gap-2">
        <Dialog.CloseTrigger
          class="h-9 cursor-pointer rounded-md border border-boulder-200 bg-white px-3.5 text-[13px] font-semibold text-boulder-700 hover:bg-boulder-100"
          >Cancelar</Dialog.CloseTrigger
        ><button
          class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-700 px-3.5 text-[13px] font-semibold text-white hover:bg-east-bay-800"
          type="button"
          onclick={onConfirm}><Download size={14} /><span>Confirmar</span></button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
