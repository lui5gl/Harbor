<script lang="ts">
  import { t } from "#lib/i18n";
  import { Check, Copy, Eye, EyeOff, GitCompare, Info } from "@lucide/svelte";
  import { Button, Dialog, ScrollArea, Select } from "bits-ui";
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
    availableEnvironments.find((e) => String(e.id) === targetIdStr) ?? availableEnvironments[0],
  );

  $effect(() => {
    if (open && availableEnvironments.length > 0 && !targetIdStr) {
      targetIdStr = String(availableEnvironments[0].id);
    }
  });

  type ComparisonRow = {
    key: string;
    currentValue?: string;
    targetValue?: string;
    status: "only-current" | "only-target" | "match" | "different";
  };

  let rows = $derived.by<ComparisonRow[]>(() => {
    if (!targetEnvironment) return [];
    const currentMap = new Map(currentEnvironment.secrets.map((s) => [s.key.toUpperCase(), s]));
    const targetMap = new Map(targetEnvironment.secrets.map((s) => [s.key.toUpperCase(), s]));

    const allKeys = Array.from(new Set([...currentMap.keys(), ...targetMap.keys()])).sort();

    return allKeys.map((k) => {
      const current = currentMap.get(k);
      const target = targetMap.get(k);
      const key = current?.key ?? target?.key ?? k;

      if (current && !target) {
        return { key, currentValue: current.value, status: "only-current" };
      }
      if (!current && target) {
        return { key, targetValue: target.value, status: "only-target" };
      }
      const match = current?.value === target?.value;
      return {
        key,
        currentValue: current?.value,
        targetValue: target?.value,
        status: match ? "match" : "different",
      };
    });
  });

  let missingInCurrent = $derived(
    rows
      .filter((r) => r.status === "only-target" && r.targetValue !== undefined)
      .map((r) => ({
        key: r.key,
        value: r.targetValue!,
      })),
  );

  function importMissing() {
    if (missingInCurrent.length === 0) return;
    onImportMissing(missingInCurrent);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="compare-dialog" aria-describedby="compare-description">
      <div class="dialog-header">
        <div class="dialog-icon-box"><GitCompare size={20} strokeWidth={2.2} /></div>
        <div class="header-text">
          <Dialog.Title class="dialog-title">{t("secrets.compareEnvironmentsTitle")}</Dialog.Title>
          <Dialog.Description id="compare-description" class="dialog-description">
            {t("secrets.comparingAgainst")} <strong>{currentEnvironment.name}</strong>
            {t("secrets.against")}
            {targetEnvironment?.name ?? ""}.
          </Dialog.Description>
        </div>
      </div>

      <div class="target-selector">
        <span class="selector-label">{t("secrets.compareWith")}</span>
        <select class="environment-dropdown" bind:value={targetIdStr}>
          {#each availableEnvironments as env (env.id)}
            <option value={String(env.id)}
              >{env.name} ({env.secrets.length} {t("secrets.variables")})</option
            >
          {/each}
        </select>
        <Button.Root
          class="toggle-values-btn"
          type="button"
          onclick={() => (showValues = !showValues)}
        >
          {#if showValues}<EyeOff size={13} />{t("secrets.hideValues")}{:else}<Eye size={13} />{t(
              "secrets.revealValues",
            )}{/if}
        </Button.Root>
      </div>

      <div class="stats-bar">
        <span class="badge diff-current"
          >{rows.filter((r) => r.status === "only-current").length}
          {t("secrets.onlyIn")}
          {currentEnvironment.name}</span
        >
        <span class="badge diff-target"
          >{rows.filter((r) => r.status === "only-target").length}
          {t("secrets.onlyIn")}
          {targetEnvironment?.name ?? "target"}</span
        >
        <span class="badge diff-mismatch"
          >{rows.filter((r) => r.status === "different").length} {t("secrets.valuesDiffer")}</span
        >
        <span class="badge diff-match"
          >{rows.filter((r) => r.status === "match").length} {t("secrets.identical")}</span
        >
      </div>

      <ScrollArea.Root class="diff-scroll-area" type="auto">
        <ScrollArea.Viewport class="diff-viewport">
          <div class="diff-table">
            <div class="diff-row header-row">
              <span class="col-key">{t("secrets.key")}</span>
              <span class="col-status">Estado</span>
              <span class="col-val">{currentEnvironment.name}</span>
              <span class="col-val">{targetEnvironment?.name ?? "Target"}</span>
            </div>
            {#each rows as row (row.key)}
              <div class={`diff-row status-${row.status}`}>
                <span class="col-key">{row.key}</span>
                <span class="col-status">
                  {#if row.status === "only-current"}
                    <span class="tag tag-current">{t("secrets.statusOnlyHere")}</span>
                  {:else if row.status === "only-target"}
                    <span class="tag tag-target">{t("secrets.statusMissing")}</span>
                  {:else if row.status === "different"}
                    <span class="tag tag-different">{t("secrets.statusDifferent")}</span>
                  {:else}
                    <span class="tag tag-match">{t("secrets.statusMatch")}</span>
                  {/if}
                </span>
                <span class="col-val mono">
                  {#if row.currentValue !== undefined}
                    {showValues ? row.currentValue || "<vacío>" : "••••••••"}
                  {:else}
                    <em class="not-set">—</em>
                  {/if}
                </span>
                <span class="col-val mono">
                  {#if row.targetValue !== undefined}
                    {showValues ? row.targetValue || "<vacío>" : "••••••••"}
                  {:else}
                    <em class="not-set">—</em>
                  {/if}
                </span>
              </div>
            {/each}
          </div>
        </ScrollArea.Viewport>
        <ScrollArea.Scrollbar class="diff-scrollbar" orientation="vertical">
          <ScrollArea.Thumb class="diff-thumb" />
        </ScrollArea.Scrollbar>
      </ScrollArea.Root>

      <div class="dialog-actions">
        {#if missingInCurrent.length > 0}
          <Button.Root
            class="primary-button import-missing-btn"
            type="button"
            onclick={importMissing}
          >
            <Copy size={14} />
            <span>{t("secrets.copyMissingVars")} ({missingInCurrent.length})</span>
          </Button.Root>
        {/if}
        <Dialog.Close class="secondary-button">{t("secrets.close")}</Dialog.Close>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.modal-backdrop) {
    background: rgb(11 11 11 / 45%);
    backdrop-filter: blur(2px);
    inset: 0;
    position: fixed;
    z-index: 100;
  }
  :global(.compare-dialog) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow: 0 20px 60px rgb(11 11 11 / 20%);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 12px;
    left: 50%;
    max-height: calc(100vh - 48px);
    max-width: 780px;
    padding: 22px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(calc(100vw - 32px), 780px);
    z-index: 101;
  }
  .dialog-header {
    display: flex;
    gap: 12px;
    align-items: center;
  }
  .dialog-icon-box {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    border-radius: 8px;
    color: var(--color-east-bay-700);
    display: flex;
    height: 38px;
    justify-content: center;
    width: 38px;
    flex-shrink: 0;
  }
  :global(.dialog-title) {
    font-size: 17px;
    font-weight: 700;
    color: var(--color-boulder-950);
    margin: 0;
  }
  :global(.dialog-description) {
    font-size: 13px;
    color: var(--color-boulder-600);
    margin: 2px 0 0;
  }
  .target-selector {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: wrap;
    font-size: 12px;
  }
  .selector-label {
    font-weight: 650;
    color: var(--color-boulder-700);
  }
  .environment-dropdown {
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    padding: 6px 10px;
    font: inherit;
    font-size: 12px;
    background: white;
    color: var(--color-boulder-900);
    outline: none;
  }
  :global(.toggle-values-btn) {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    background: white;
    padding: 5px 9px;
    font: inherit;
    font-size: 11px;
    cursor: pointer;
    margin-left: auto;
    color: var(--color-boulder-700);
  }
  .stats-bar {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    font-size: 11px;
  }
  .badge {
    padding: 3px 8px;
    border-radius: 99px;
    font-weight: 600;
  }
  .diff-current {
    background: #eff6ff;
    color: #1e40af;
  }
  .diff-target {
    background: #fef3c7;
    color: #92400e;
  }
  .diff-mismatch {
    background: #fee2e2;
    color: #991b1b;
  }
  .diff-match {
    background: #f0fdf4;
    color: #166534;
  }

  :global(.diff-scroll-area) {
    flex: 1;
    min-height: 240px;
    max-height: 380px;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    overflow: hidden;
  }
  :global(.diff-viewport) {
    height: 100%;
    width: 100%;
  }
  .diff-table {
    display: flex;
    flex-direction: column;
    width: 100%;
  }
  .diff-row {
    display: grid;
    grid-template-columns: 180px 100px 1fr 1fr;
    gap: 10px;
    padding: 8px 12px;
    font-size: 12px;
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
  }
  .header-row {
    background: var(--color-boulder-50);
    font-weight: 700;
    color: var(--color-boulder-700);
    font-size: 11px;
    text-transform: uppercase;
  }
  .col-key {
    font-weight: 600;
    word-break: break-all;
  }
  .col-val.mono {
    font-family: ui-monospace, monospace;
    font-size: 11px;
    word-break: break-all;
  }
  .not-set {
    color: var(--color-boulder-400);
  }
  .tag {
    display: inline-block;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10.5px;
    font-weight: 650;
  }
  .tag-current {
    background: #dbeafe;
    color: #1e40af;
  }
  .tag-target {
    background: #fef3c7;
    color: #b45309;
  }
  .tag-different {
    background: #fee2e2;
    color: #dc2626;
  }
  .tag-match {
    background: #dcfce7;
    color: #15803d;
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 6px;
  }
  :global(.primary-button),
  :global(.secondary-button) {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    border-radius: 6px;
    padding: 8px 12px;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    cursor: pointer;
  }
  :global(.primary-button) {
    background: var(--color-east-bay-900);
    color: white;
    border: none;
  }
  :global(.secondary-button) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
    border: 1px solid var(--color-boulder-300);
  }
  :global(.diff-scrollbar) {
    display: flex;
    padding: 2px;
    width: 6px;
  }
  :global(.diff-thumb) {
    background: var(--color-boulder-300);
    border-radius: 99px;
    flex: 1;
  }
</style>
