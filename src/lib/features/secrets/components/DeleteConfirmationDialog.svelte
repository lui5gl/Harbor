<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { AlertTriangle, Trash2 } from "@lucide/svelte";

  type DeleteConfirmationDialogProps = {
    open: boolean;
    title: string;
    description: string;
    actionLabel?: string;
    confirmKeyword?: string;
    onOpenChange: (open: boolean) => void;
    onConfirm: () => void;
  };

  let {
    open = $bindable(),
    title,
    description,
    actionLabel = "Delete",
    confirmKeyword,
    onOpenChange,
    onConfirm,
  }: DeleteConfirmationDialogProps = $props();
  let inputValue = $state("");
  let hasTypedConfirmation = $derived(Boolean(confirmKeyword));
  let isMatch = $derived(
    !hasTypedConfirmation || inputValue.trim().toUpperCase() === confirmKeyword?.toUpperCase(),
  );

  $effect(() => {
    if (!open) inputValue = "";
  });

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!isMatch) return;
    onConfirm();
  }
</script>

<Dialog.Root bind:open role="alertdialog" onOpenChange={(details) => onOpenChange(details.open)}>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="w-full max-w-md rounded-xl border border-boulder-200 bg-white p-5 shadow-2xl outline-none"
    >
      <form onsubmit={handleSubmit} class="flex flex-col gap-4">
        <div class="flex gap-3.5">
          <div
            class="flex size-10 shrink-0 items-center justify-center rounded-lg border border-red-100 bg-red-50 text-red-600"
          >
            <AlertTriangle size={20} strokeWidth={2.2} />
          </div>
          <div class="flex min-w-0 flex-1 flex-col gap-1">
            <Dialog.Title class="text-base font-bold text-boulder-950">{title}</Dialog.Title>
            <Dialog.Description class="text-[13.5px] leading-relaxed text-boulder-600"
              >{description}</Dialog.Description
            >
          </div>
        </div>
        {#if hasTypedConfirmation}
          <div class="flex flex-col gap-2 rounded-md border border-boulder-200 bg-boulder-50 p-3">
            <label class="text-xs font-medium text-boulder-700" for="delete-confirmation-input"
              >To confirm, type <span
                class="rounded bg-boulder-200 px-1.5 py-0.5 font-mono text-[11.5px] font-bold text-boulder-950"
                >{confirmKeyword}</span
              > below:</label
            >
            <input
              id="delete-confirmation-input"
              class="h-8.5 w-full rounded-md border border-boulder-300 bg-white px-2.5 text-[13px] text-boulder-900 outline-none focus:border-red-600 focus:ring-3 focus:ring-red-600/15"
              type="text"
              bind:value={inputValue}
              placeholder={confirmKeyword}
              autocomplete="off"
              spellcheck="false"
            />
          </div>
        {/if}
        <div class="flex items-center justify-end gap-2">
          <Dialog.CloseTrigger
            class="h-8.5 cursor-pointer rounded-md bg-boulder-100 px-3 text-xs font-semibold text-boulder-800 hover:bg-boulder-200"
            >Cancel</Dialog.CloseTrigger
          >
          <button
            class="inline-flex h-8.5 cursor-pointer items-center gap-1.5 rounded-md bg-red-600 px-3 text-xs font-semibold text-white hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-50"
            type="submit"
            disabled={!isMatch}
            ><Trash2 size={14} strokeWidth={2} aria-hidden="true" /><span>{actionLabel}</span
            ></button
          >
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
