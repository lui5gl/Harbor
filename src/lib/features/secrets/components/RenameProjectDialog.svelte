<script lang="ts">
  import { t } from "#lib/i18n";
  import { Dialog } from "@ark-ui/svelte/dialog";

  type Props = { open: boolean; initialName: string; onSave: (name: string) => void };
  let { open = $bindable(), initialName, onSave }: Props = $props();
  let draftName = $state("");

  $effect(() => {
    if (open) draftName = initialName;
  });

  function saveProject() {
    const name = draftName.trim();
    if (!name) return;
    onSave(name);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/35 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="w-full max-w-sm rounded-lg border border-boulder-200 bg-white p-4.5 shadow-2xl outline-none"
      aria-describedby="rename-project-description"
    >
      <Dialog.Title class="mb-3 text-[15px] font-bold text-boulder-900"
        >{t("secrets.renameProject")}</Dialog.Title
      >
      <Dialog.Description id="rename-project-description" class="sr-only"
        >{t("secrets.renameProject")}</Dialog.Description
      >
      <form
        onsubmit={(event) => {
          event.preventDefault();
          saveProject();
        }}
      >
        <label class="sr-only" for="project-name">{t("secrets.renameProject")}</label>
        <input
          id="project-name"
          class="h-8 w-full rounded-md border border-boulder-300 px-2 text-xs text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-2 focus:ring-east-bay-500/15"
          bind:value={draftName}
        />
        <div class="mt-3 flex justify-end gap-2">
          <Dialog.CloseTrigger
            class="h-8 cursor-pointer rounded-md bg-boulder-100 px-2.5 text-xs font-semibold text-boulder-700 hover:bg-boulder-200"
            >{t("common.cancel")}</Dialog.CloseTrigger
          >
          <button
            class="h-8 cursor-pointer rounded-md bg-east-bay-900 px-2.5 text-xs font-semibold text-white hover:bg-east-bay-950 disabled:opacity-50"
            type="submit"
            disabled={!draftName.trim()}>{t("common.save")}</button
          >
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
