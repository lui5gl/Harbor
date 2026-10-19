<script lang="ts">
  import { t } from "#lib/i18n";
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Copy, Eye, EyeOff, GitCompare } from "@lucide/svelte";
  import type { Environment } from "../types";

  type CompareDialogProps = {
    open: boolean;
    currentEnvironment: Environment;
    availableEnvironments: Environment[];
    onImportMissing: (variables: { key: string; value: string }[]) => void;
  };

  let {
    open = $bindable(),
    currentEnvironment,
    availableEnvironments,
    onImportMissing,
  }: CompareDialogProps = $props();
  let targetIdStr = $state("");
  let showValues = $state(false);
  let targetEnvironment = $derived(
    availableEnvironments.find((environment) => String(environment.id) === targetIdStr) ??
      availableEnvironments[0],
  );

  $effect(() => {
    if (open && availableEnvironments.length > 0 && !targetIdStr)
      targetIdStr = String(availableEnvironments[0].id);
  });

  type ComparisonRow = {
    key: string;
    currentValue?: string;
    targetValue?: string;
    status: "only-current" | "only-target" | "match" | "different";
  };
  let rows = $derived.by<ComparisonRow[]>(() => {
    if (!targetEnvironment) return [];
    const currentMap = new Map(
      currentEnvironment.secrets.map((secret) => [secret.key.toUpperCase(), secret]),
    );
    const targetMap = new Map(
      targetEnvironment.secrets.map((secret) => [secret.key.toUpperCase(), secret]),
    );
    return Array.from(new Set([...currentMap.keys(), ...targetMap.keys()]))
      .sort()
      .map((normalizedKey) => {
        const current = currentMap.get(normalizedKey);
        const target = targetMap.get(normalizedKey);
        const key = current?.key ?? target?.key ?? normalizedKey;
        if (current && !target)
          return { key, currentValue: current.value, status: "only-current" as const };
        if (!current && target)
          return { key, targetValue: target.value, status: "only-target" as const };
        return {
          key,
          currentValue: current?.value,
          targetValue: target?.value,
          status: current?.value === target?.value ? ("match" as const) : ("different" as const),
        };
      });
  });
  let missingInCurrent = $derived(
    rows
      .filter((row) => row.status === "only-target" && row.targetValue !== undefined)
      .map((row) => ({ key: row.key, value: row.targetValue! })),
  );

  function importMissing() {
    if (missingInCurrent.length === 0) return;
    onImportMissing(missingInCurrent);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="flex max-h-[calc(100dvh-2rem)] w-full max-w-3xl flex-col gap-3 rounded-xl border border-boulder-200 bg-white p-5.5 shadow-2xl outline-none"
      aria-describedby="compare-description"
    >
      <div class="flex items-center gap-3">
        <div
          class="flex size-9.5 shrink-0 items-center justify-center rounded-lg border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        >
          <GitCompare size={20} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="text-[17px] font-bold text-boulder-950"
            >{t("secrets.compareEnvironmentsTitle")}</Dialog.Title
          ><Dialog.Description id="compare-description" class="mt-0.5 text-[13px] text-boulder-600"
            >{t("secrets.comparingAgainst")} <strong>{currentEnvironment.name}</strong>
            {t("secrets.against")}
            {targetEnvironment?.name ?? ""}.</Dialog.Description
          >
        </div>
      </div>

      <div class="flex flex-wrap items-center gap-2.5 text-xs">
        <span class="font-semibold text-boulder-700">{t("secrets.compareWith")}</span>
        <select
          class="h-8 rounded-md border border-boulder-300 bg-white px-2.5 text-xs text-boulder-900 outline-none focus:border-east-bay-500"
          bind:value={targetIdStr}
          >{#each availableEnvironments as environment (environment.id)}<option
              value={String(environment.id)}
              >{environment.name} ({environment.secrets.length} {t("secrets.variables")})</option
            >{/each}</select
        >
        <button
          class="ml-auto inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-md border border-boulder-300 bg-white px-2.5 text-[11px] font-semibold text-boulder-700 hover:bg-boulder-100"
          type="button"
          onclick={() => (showValues = !showValues)}
          >{#if showValues}<EyeOff size={13} />{t("secrets.hideValues")}{:else}<Eye size={13} />{t(
              "secrets.revealValues",
            )}{/if}</button
        >
      </div>

      <div class="flex flex-wrap gap-2 text-[11px] font-semibold">
        <span class="rounded-full bg-blue-50 px-2 py-1 text-blue-800"
          >{rows.filter((row) => row.status === "only-current").length}
          {t("secrets.onlyIn")}
          {currentEnvironment.name}</span
        >
        <span class="rounded-full bg-amber-100 px-2 py-1 text-amber-800"
          >{rows.filter((row) => row.status === "only-target").length}
          {t("secrets.onlyIn")}
          {targetEnvironment?.name ?? "target"}</span
        >
        <span class="rounded-full bg-red-100 px-2 py-1 text-red-800"
          >{rows.filter((row) => row.status === "different").length}
          {t("secrets.valuesDiffer")}</span
        >
        <span class="rounded-full bg-emerald-50 px-2 py-1 text-emerald-800"
          >{rows.filter((row) => row.status === "match").length} {t("secrets.identical")}</span
        >
      </div>

      <div class="min-h-60 flex-1 overflow-auto rounded-md border border-boulder-200">
        <div class="min-w-170">
          <div
            class="grid grid-cols-[180px_100px_1fr_1fr] gap-2.5 border-b border-boulder-200 bg-boulder-50 px-3 py-2 text-[10px] font-bold uppercase text-boulder-600"
          >
            <span>{t("secrets.key")}</span><span>Estado</span><span>{currentEnvironment.name}</span
            ><span>{targetEnvironment?.name ?? "Target"}</span>
          </div>
          {#each rows as row (row.key)}
            <div
              class="grid grid-cols-[180px_100px_1fr_1fr] items-center gap-2.5 border-b border-boulder-100 px-3 py-2 text-xs last:border-b-0"
            >
              <span class="break-all font-semibold text-boulder-900">{row.key}</span>
              <span
                >{#if row.status === "only-current"}<span
                    class="rounded bg-blue-100 px-1.5 py-0.5 text-[10.5px] font-semibold text-blue-800"
                    >{t("secrets.statusOnlyHere")}</span
                  >{:else if row.status === "only-target"}<span
                    class="rounded bg-amber-100 px-1.5 py-0.5 text-[10.5px] font-semibold text-amber-700"
                    >{t("secrets.statusMissing")}</span
                  >{:else if row.status === "different"}<span
                    class="rounded bg-red-100 px-1.5 py-0.5 text-[10.5px] font-semibold text-red-600"
                    >{t("secrets.statusDifferent")}</span
                  >{:else}<span
                    class="rounded bg-emerald-100 px-1.5 py-0.5 text-[10.5px] font-semibold text-emerald-700"
                    >{t("secrets.statusMatch")}</span
                  >{/if}</span
              >
              <span class="break-all font-mono text-[11px] text-boulder-700"
                >{#if row.currentValue !== undefined}{showValues
                    ? row.currentValue || "<vacío>"
                    : "••••••••"}{:else}<em class="text-boulder-400">—</em>{/if}</span
              >
              <span class="break-all font-mono text-[11px] text-boulder-700"
                >{#if row.targetValue !== undefined}{showValues
                    ? row.targetValue || "<vacío>"
                    : "••••••••"}{:else}<em class="text-boulder-400">—</em>{/if}</span
              >
            </div>
          {/each}
        </div>
      </div>

      <div class="mt-1.5 flex justify-end gap-2">
        {#if missingInCurrent.length > 0}<button
            class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-semibold text-white hover:bg-east-bay-950"
            type="button"
            onclick={importMissing}
            ><Copy size={14} /><span
              >{t("secrets.copyMissingVars")} ({missingInCurrent.length})</span
            ></button
          >{/if}
        <Dialog.CloseTrigger
          class="h-9 cursor-pointer rounded-md border border-boulder-300 bg-boulder-100 px-3 text-xs font-semibold text-boulder-800 hover:bg-boulder-200"
          >{t("secrets.close")}</Dialog.CloseTrigger
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
