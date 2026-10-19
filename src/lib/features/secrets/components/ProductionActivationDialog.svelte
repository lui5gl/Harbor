<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { ShieldAlert } from "@lucide/svelte";

  type ProductionActivationDialogProps = {
    open: boolean;
    environmentLabel: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };
  let {
    open = $bindable(),
    environmentLabel,
    onOpenChange,
    onConfirm,
  }: ProductionActivationDialogProps = $props();
</script>

<Dialog.Root bind:open role="alertdialog" onOpenChange={(details) => onOpenChange(details.open)}>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="w-full max-w-md rounded-lg border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
    >
      <div
        class="flex size-10.5 items-center justify-center rounded-md bg-amber-100 text-amber-800"
      >
        <ShieldAlert size={22} strokeWidth={2} />
      </div>
      <Dialog.Title class="mt-4.5 text-lg font-semibold text-boulder-950"
        >Activate this environment?</Dialog.Title
      >
      <Dialog.Description class="mt-2.5 text-sm leading-relaxed text-boulder-600"
        >{environmentLabel} will become available to new PowerShell sessions and system processes. Confirm
        to continue.</Dialog.Description
      >
      <div class="mt-6 flex items-center justify-end gap-2.5">
        <Dialog.CloseTrigger
          class="h-9 cursor-pointer rounded-md bg-boulder-100 px-3 text-sm font-semibold text-boulder-800 hover:bg-boulder-200"
          >Cancel</Dialog.CloseTrigger
        >
        <button
          class="h-9 cursor-pointer rounded-md bg-amber-700 px-3.5 text-sm font-semibold text-white hover:bg-amber-800"
          type="button"
          onclick={onConfirm}>Activate environment</button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
