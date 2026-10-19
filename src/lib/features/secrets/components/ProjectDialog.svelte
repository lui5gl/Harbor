<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Check, FolderPlus } from "@lucide/svelte";

  type ProjectDialogProps = {
    open: boolean;
    onCreate: (projectName: string) => void;
  };

  let { open = $bindable(), onCreate }: ProjectDialogProps = $props();
  let projectName = $state("");

  $effect(() => {
    if (!open) projectName = "";
  });

  function submit() {
    const normalizedProjectName = projectName.trim();
    if (!normalizedProjectName) return;
    onCreate(normalizedProjectName);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="flex w-full max-w-md flex-col gap-2.5 rounded-lg border border-boulder-200 bg-white p-5 shadow-2xl outline-none"
      aria-describedby="new-project-description"
    >
      <div class="mb-1 flex items-start gap-3">
        <div
          class="flex size-9 shrink-0 items-center justify-center rounded-lg border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        >
          <FolderPlus size={19} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="text-base font-bold text-boulder-950">New project</Dialog.Title
          ><Dialog.Description
            id="new-project-description"
            class="mt-0.5 text-xs leading-relaxed text-boulder-600"
            >Create an application with Development, Staging, and Production environments.</Dialog.Description
          >
        </div>
      </div>
      <label class="mt-1 text-xs font-semibold text-boulder-700" for="project-name"
        >Project / application name</label
      >
      <input
        id="project-name"
        class="h-9 w-full rounded-md border border-boulder-300 px-2.5 text-[13px] text-boulder-900 outline-none transition focus:border-east-bay-500 focus:ring-3 focus:ring-east-bay-500/15"
        placeholder="e.g. Acme Store API"
        bind:value={projectName}
      />
      <div class="mt-2.5 flex justify-end gap-2">
        <Dialog.CloseTrigger
          class="inline-flex h-8.5 cursor-pointer items-center justify-center rounded-md bg-boulder-100 px-3 text-xs font-semibold text-boulder-800 hover:bg-boulder-200"
          >Cancel</Dialog.CloseTrigger
        >
        <button
          class="inline-flex h-8.5 cursor-pointer items-center justify-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-semibold text-white hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
          type="button"
          onclick={submit}
          disabled={!projectName.trim()}
          ><Check size={15} strokeWidth={2.2} /><span>Create project</span></button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
