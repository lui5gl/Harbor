<script lang="ts">
  import { t } from "#lib/i18n";
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Copy } from "@lucide/svelte";

  type CloneEnvironmentDialogProps = {
    open: boolean;
    projectName: string;
    sourceEnvironmentName: string;
    variableCount: number;
    onClone: (environmentName: string) => void;
  };

  let {
    open = $bindable(),
    projectName,
    sourceEnvironmentName,
    variableCount,
    onClone,
  }: CloneEnvironmentDialogProps = $props();
  let environmentName = $state("");

  $effect(() => {
    if (open) environmentName = `${sourceEnvironmentName} (${t("secrets.cloneButton")})`;
  });

  function submit() {
    const normalized = environmentName.trim();
    if (!normalized) return;
    onClone(normalized);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="flex w-full max-w-md flex-col gap-2.5 rounded-lg border border-boulder-200 bg-white p-5 shadow-2xl outline-none"
      aria-describedby="clone-environment-description"
    >
      <div class="mb-1 flex items-start gap-3">
        <div
          class="flex size-9 shrink-0 items-center justify-center rounded-lg border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        >
          <Copy size={19} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="text-base font-bold text-boulder-950"
            >{t("secrets.cloneEnvironmentTitle")}</Dialog.Title
          >
          <Dialog.Description
            id="clone-environment-description"
            class="mt-0.5 text-xs leading-relaxed text-boulder-600"
            >"{sourceEnvironmentName}" ({variableCount}
            {variableCount === 1 ? t("secrets.variable") : t("secrets.variables")}) • {projectName}</Dialog.Description
          >
        </div>
      </div>
      <label class="mt-1 text-xs font-semibold text-boulder-700" for="clone-environment-name"
        >{t("secrets.newEnvironmentName")}</label
      >
      <input
        id="clone-environment-name"
        class="h-9 w-full rounded-md border border-boulder-300 px-2.5 text-[13px] text-boulder-900 outline-none transition focus:border-east-bay-500 focus:ring-3 focus:ring-east-bay-500/15"
        placeholder="e.g. Staging"
        bind:value={environmentName}
      />
      <div class="mt-2.5 flex justify-end gap-2">
        <Dialog.CloseTrigger
          class="inline-flex h-8.5 cursor-pointer items-center justify-center rounded-md bg-boulder-100 px-3 text-xs font-semibold text-boulder-800 hover:bg-boulder-200"
          >{t("secrets.cancel")}</Dialog.CloseTrigger
        >
        <button
          class="inline-flex h-8.5 cursor-pointer items-center justify-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-semibold text-white hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
          type="button"
          onclick={submit}
          disabled={!environmentName.trim()}
          ><Copy size={14} strokeWidth={2.2} /><span>{t("secrets.cloneButton")}</span></button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
