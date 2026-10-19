<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Check } from "@lucide/svelte";

  type ProfileSettingsDialogProps = {
    open: boolean;
    name: string;
    isProduction: boolean;
    onSave: (name: string, isProduction: boolean) => void;
  };
  let { open = $bindable(), name, isProduction, onSave }: ProfileSettingsDialogProps = $props();
  let draftName = $state("");
  let draftIsProduction = $state(false);

  $effect(() => {
    if (open) {
      draftName = name;
      draftIsProduction = isProduction;
    }
  });

  function saveSettings() {
    onSave(draftName.trim() || "Untitled environment", draftIsProduction);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="grid w-full max-w-md gap-3 rounded-lg border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
      aria-describedby="profile-settings-description"
    >
      <Dialog.Title class="text-lg font-semibold text-boulder-950"
        >Environment settings</Dialog.Title
      >
      <Dialog.Description
        id="profile-settings-description"
        class="mb-2.5 text-[13px] leading-relaxed text-boulder-600"
        >Update the selected environment and its production protection.</Dialog.Description
      >
      <label class="text-xs font-semibold text-boulder-700" for="settings-profile-name"
        >Environment name</label
      >
      <input
        id="settings-profile-name"
        class="h-9.5 w-full rounded-md border border-boulder-300 bg-white px-2.5 text-[13px] text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-3 focus:ring-east-bay-500/15"
        placeholder="e.g. Development, Staging, Production"
        bind:value={draftName}
      />
      <label
        class="mt-1.5 flex min-h-12 cursor-pointer items-center justify-between gap-4 rounded-md border border-boulder-200 bg-boulder-50 px-2.5 py-2"
      >
        <span class="grid gap-0.5"
          ><strong class="text-xs text-boulder-800">Production environment</strong><small
            class="text-[11px] text-boulder-500"
            >Requires confirmation before activation in the shell.</small
          ></span
        >
        <input
          class="peer sr-only"
          type="checkbox"
          bind:checked={draftIsProduction}
          aria-label="Production environment"
        />
        <span
          class="relative h-5 w-9 shrink-0 rounded-full border border-boulder-300 bg-white transition-colors after:absolute after:top-0.5 after:left-0.5 after:size-3.5 after:rounded-full after:bg-boulder-400 after:transition-transform peer-checked:border-east-bay-700 peer-checked:bg-east-bay-700 peer-checked:after:translate-x-4 peer-checked:after:bg-white"
        ></span>
      </label>
      <div class="mt-3 flex items-center justify-end gap-2.5">
        <Dialog.CloseTrigger
          class="h-9 cursor-pointer rounded-md bg-boulder-100 px-3 text-sm font-semibold text-boulder-800 hover:bg-boulder-200"
          >Cancel</Dialog.CloseTrigger
        >
        <button
          class="inline-flex h-9 cursor-pointer items-center gap-2 rounded-md bg-east-bay-900 px-3.5 text-sm font-semibold text-white hover:bg-east-bay-950"
          type="button"
          onclick={saveSettings}
          ><Check size={16} strokeWidth={2.2} aria-hidden="true" /><span>Save changes</span></button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
