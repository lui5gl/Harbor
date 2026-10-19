<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { AlertTriangle, Trash2 } from "@lucide/svelte";

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
    onConfirm,
  }: DeleteRuntimeDialogProps = $props();
  let inputValue = $state("");
  const confirmKeyword = "CONFIRMAR";
  let isMatch = $derived(inputValue.trim().toUpperCase() === confirmKeyword);

  $effect(() => {
    if (!open) inputValue = "";
  });
  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (isMatch) onConfirm();
  }
</script>

<Dialog.Root bind:open role="alertdialog" onOpenChange={(details) => onOpenChange(details.open)}>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="w-full max-w-lg rounded-xl border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
    >
      <form onsubmit={handleSubmit} class="flex flex-col gap-5">
        <div class="flex items-start gap-4">
          <div
            class="flex size-10 shrink-0 items-center justify-center rounded-lg border border-red-100 bg-red-50 text-red-600"
          >
            <AlertTriangle size={20} strokeWidth={2.2} />
          </div>
          <div class="min-w-0">
            <Dialog.Title class="text-base font-semibold leading-snug text-boulder-950"
              >Delete {serviceLabel} {version}?</Dialog.Title
            ><Dialog.Description class="mt-1.5 text-[13.5px] leading-relaxed text-boulder-600"
              >This will permanently delete the installed binaries and files for {serviceLabel}
              {version} from your Harbor runtimes directory.</Dialog.Description
            >
          </div>
        </div>
        <div
          class="flex flex-col gap-2 rounded-lg border border-boulder-200 bg-boulder-50 px-3.5 py-3"
        >
          <label
            class="text-xs font-medium text-boulder-700"
            for="runtime-delete-confirmation-input"
            >To confirm, type <span class="font-mono font-bold text-boulder-950"
              >{confirmKeyword}</span
            > below:</label
          >
          <input
            id="runtime-delete-confirmation-input"
            class="w-full rounded-md border border-boulder-300 bg-white px-2.5 py-2 font-mono text-[13.5px] font-semibold text-boulder-950 outline-none focus:border-red-600 focus:ring-2 focus:ring-red-600/15"
            type="text"
            bind:value={inputValue}
            placeholder={confirmKeyword}
            autocomplete="off"
            spellcheck="false"
          />
        </div>
        <div class="flex justify-end gap-2.5">
          <Dialog.CloseTrigger
            class="h-9 cursor-pointer rounded-md border border-boulder-200 bg-white px-3.5 text-[13px] font-semibold text-boulder-700 hover:bg-boulder-100"
            >Cancel</Dialog.CloseTrigger
          ><button
            class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md bg-red-600 px-3.5 text-[13px] font-semibold text-white hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-50"
            type="submit"
            disabled={!isMatch}><Trash2 size={14} /><span>Delete version</span></button
          >
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
