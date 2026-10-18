<script lang="ts">
  import { t } from "#lib/i18n";
  import {
    AlertCircle,
    Check,
    Copy,
    Eye,
    EyeOff,
    KeyRound,
    Lock,
    Plus,
    Search,
    Trash2,
    Wand2,
    X,
  } from "@lucide/svelte";
  import { Button, ScrollArea, Tooltip } from "bits-ui";
  import { onMount } from "svelte";
  import { generateSecret } from "../application/secretGenerator";
  import { validateSecretKey } from "../domain/validation";
  import type { Environment, Secret } from "../types";

  type Props = {
    disabled?: boolean;
    environment: Environment;
    onAddVariable: () => number | null;
    onUpdateVariable: (id: number, field: "key" | "value", value: string) => void;
    onRequestDeleteVariable: (id: number) => void;
  };

  let {
    disabled = false,
    environment,
    onAddVariable,
    onUpdateVariable,
    onRequestDeleteVariable,
  }: Props = $props();

  let revealed = $state<number[]>([]);
  let copied = $state<number | null>(null);
  let draftKey = $state("");
  let draftValue = $state("");
  let query = $state("");
  let clipboardError = $state("");
  let revealTimer: number | undefined;

  let totalCount = $derived(environment.secrets.length);
  let secrets = $derived(
    environment.secrets.filter(
      (secret) =>
        (secret.key.trim() || secret.value.trim()) &&
        secret.key.toLowerCase().includes(query.trim().toLowerCase()),
    ),
  );
  let allRevealed = $derived(
    secrets.length > 0 && secrets.every((secret) => revealed.includes(secret.id)),
  );

  function toggle(id: number) {
    revealed = revealed.includes(id) ? revealed.filter((item) => item !== id) : [...revealed, id];
    resetRevealTimer();
  }

  function toggleAll() {
    revealed = allRevealed ? [] : secrets.map((secret) => secret.id);
    resetRevealTimer();
  }

  function resetRevealTimer() {
    window.clearTimeout(revealTimer);
    revealTimer = window.setTimeout(() => (revealed = []), 30_000);
  }

  function generateInline(secretId: number) {
    if (disabled) return;
    const generated = generateSecret("token", 32);
    onUpdateVariable(secretId, "value", generated);
    if (!revealed.includes(secretId)) {
      revealed = [...revealed, secretId];
    }
    resetRevealTimer();
  }

  $effect(() => {
    environment.id;
    revealed = [];
    query = "";
    draftKey = "";
    draftValue = "";
  });

  onMount(() => {
    const hide = () => {
      revealed = [];
      copied = null;
    };
    window.addEventListener("blur", hide);
    document.addEventListener("visibilitychange", hide);
    return () => {
      window.removeEventListener("blur", hide);
      document.removeEventListener("visibilitychange", hide);
      window.clearTimeout(revealTimer);
    };
  });

  async function copyValue(secret: Secret) {
    try {
      await navigator.clipboard.writeText(secret.value);
      clipboardError = "";
      copied = secret.id;
      window.setTimeout(() => {
        if (copied === secret.id) copied = null;
      }, 1400);
    } catch {
      copied = null;
      clipboardError = "Unable to copy to clipboard. Check system permissions.";
    }
  }

  function splitAssignment(secret: Secret, event: ClipboardEvent) {
    const text = event.clipboardData?.getData("text") ?? "";
    const separator = text.indexOf("=");
    if (separator <= 0) return;
    const key = text.slice(0, separator).trim();
    if (!key) return;
    event.preventDefault();
    onUpdateVariable(secret.id, "key", key);
    onUpdateVariable(secret.id, "value", text.slice(separator + 1));
  }

  function splitDraftAssignment(event: ClipboardEvent) {
    const text = event.clipboardData?.getData("text") ?? "";
    const separator = text.indexOf("=");
    if (separator <= 0) return;
    const key = text.slice(0, separator).trim();
    if (!key) return;
    event.preventDefault();
    const id = onAddVariable();
    if (id === null) return;
    onUpdateVariable(id, "key", key);
    onUpdateVariable(id, "value", text.slice(separator + 1));
    draftKey = "";
    draftValue = "";
  }

  function handleDraftSubmit() {
    const key = draftKey.trim();
    if (!key) return;
    const id = onAddVariable();
    if (id === null) return;
    onUpdateVariable(id, "key", key);
    if (draftValue) {
      onUpdateVariable(id, "value", draftValue);
    }
    draftKey = "";
    draftValue = "";
  }

  function generateForDraft() {
    if (disabled) return;
    const id = onAddVariable();
    if (id === null) return;
    const key = draftKey.trim() || "SECRET_KEY";
    onUpdateVariable(id, "key", key);
    const generated = generateSecret("token", 32);
    onUpdateVariable(id, "value", generated);
    revealed = [...revealed, id];
    draftKey = "";
    draftValue = "";
    resetRevealTimer();
  }
</script>

<div class="table-container">
  {#if disabled}
    <div class="readonly-banner" role="status">
      <Lock size={13.5} class="lock-icon" />
      <span>{t("secrets.readOnlyActiveNotice")}</span>
    </div>
  {/if}

  <div class="toolbar-row">
    <div class="search-wrapper">
      <Search size={13.5} class="search-icon" />
      <input
        type="search"
        class="search-input"
        aria-label={t("secrets.filterVariables")}
        placeholder={t("secrets.filterVariables")}
        bind:value={query}
      />
      {#if query}
        <button
          type="button"
          class="clear-query-btn"
          aria-label={t("secrets.clearFilter")}
          onclick={() => (query = "")}
        >
          <X size={12} />
        </button>
      {/if}
    </div>

    <div class="toolbar-actions">
      {#if query && totalCount > 0}
        <span class="count-tag">
          {secrets.length}
          {t("secrets.of")}
          {totalCount}
        </span>
      {/if}

      {#if secrets.length > 0}
        <Button.Root class="ghost-action-btn" type="button" onclick={toggleAll}>
          {#if allRevealed}
            <EyeOff size={13} />
            <span>{t("secrets.hideAll")}</span>
          {:else}
            <Eye size={13} />
            <span>{t("secrets.showAll")}</span>
          {/if}
        </Button.Root>
      {/if}

      <Button.Root
        class="add-variable-btn"
        type="button"
        {disabled}
        onclick={() => onAddVariable()}
      >
        <Plus size={13.5} strokeWidth={2.4} />
        <span>{t("secrets.addVariable")}</span>
      </Button.Root>
    </div>
  </div>

  {#if clipboardError}
    <p class="clipboard-error" role="alert">{clipboardError}</p>
  {/if}

  <ScrollArea.Root class="variables-scroll" type="auto">
    <ScrollArea.Viewport class="variables-table">
      <div class="table-header">
        <span class="col-head">{t("secrets.key")}</span>
        <span class="col-head">{t("secrets.value")}</span>
        <span class="col-head col-actions-head">{t("secrets.variableActions")}</span>
      </div>

      {#if secrets.length === 0 && query}
        <div class="empty-filter-state">
          <p>{t("secrets.noMatchFor")} "<strong>{query}</strong>"</p>
          <Button.Root class="reset-filter-btn" type="button" onclick={() => (query = "")}>
            {t("secrets.clearFilter")}
          </Button.Root>
        </div>
      {:else if secrets.length === 0 && !draftKey}
        <div class="empty-env-state">
          <div class="empty-icon-ring">
            <KeyRound size={22} class="empty-icon" strokeWidth={1.8} />
          </div>
          <h4>{t("secrets.noVariablesTitle")}</h4>
          <p>{t("secrets.noVariablesDesc")}</p>
          <div class="empty-actions">
            <Button.Root
              class="primary-start-btn"
              type="button"
              {disabled}
              onclick={() => onAddVariable()}
            >
              <Plus size={13.5} strokeWidth={2.4} />
              <span>{t("secrets.addFirstVariable")}</span>
            </Button.Root>
            <Button.Root
              class="secondary-start-btn"
              type="button"
              {disabled}
              onclick={generateForDraft}
            >
              <Wand2 size={13.5} />
              <span>{t("secrets.generateKey")}</span>
            </Button.Root>
          </div>
        </div>
      {/if}

      {#each secrets as secret (secret.id)}
        {@const shown = revealed.includes(secret.id)}
        {@const keyError = secret.key.trim() ? validateSecretKey(secret.key) : null}
        <div class="variable-row">
          <div class="key-field-wrapper">
            <input
              class={`input key-input${keyError ? " has-error" : ""}`}
              {disabled}
              aria-label={`${t("secrets.key")} ${secret.key}`}
              spellcheck="false"
              placeholder="VARIABLE_NAME"
              value={secret.key}
              oninput={(event) => onUpdateVariable(secret.id, "key", event.currentTarget.value)}
              onpaste={(event) => splitAssignment(secret, event)}
            />
            {#if keyError}
              <Tooltip.Root>
                <Tooltip.Trigger class="error-tooltip-trigger" type="button" aria-label={keyError}>
                  <AlertCircle size={13} />
                </Tooltip.Trigger>
                <Tooltip.Portal>
                  <Tooltip.Content class="tooltip-content error-tooltip" sideOffset={6}>
                    {keyError}
                  </Tooltip.Content>
                </Tooltip.Portal>
              </Tooltip.Root>
            {/if}
          </div>

          <div class="value-input-wrapper">
            <input
              class="input value-input"
              {disabled}
              aria-label={`${t("secrets.value")} ${secret.key}`}
              autocomplete="off"
              spellcheck="false"
              type={shown ? "text" : "password"}
              placeholder="Valor..."
              value={secret.value}
              oninput={(event) => onUpdateVariable(secret.id, "value", event.currentTarget.value)}
            />

            <div class="value-inner-actions">
              <Tooltip.Root>
                <Tooltip.Trigger
                  class="row-icon-btn"
                  type="button"
                  onclick={() => toggle(secret.id)}
                  aria-label={shown ? "Ocultar" : "Mostrar"}
                >
                  {#if shown}<EyeOff size={13} />{:else}<Eye size={13} />{/if}
                </Tooltip.Trigger>
                <Tooltip.Portal>
                  <Tooltip.Content class="tooltip-content" sideOffset={6}>
                    {shown ? "Ocultar valor" : "Revelar valor (30s)"}
                  </Tooltip.Content>
                </Tooltip.Portal>
              </Tooltip.Root>

              {#if !disabled}
                <Tooltip.Root>
                  <Tooltip.Trigger
                    class="row-icon-btn"
                    type="button"
                    onclick={() => generateInline(secret.id)}
                    aria-label={t("secrets.generateToken")}
                  >
                    <Wand2 size={13} />
                  </Tooltip.Trigger>
                  <Tooltip.Portal>
                    <Tooltip.Content class="tooltip-content" sideOffset={6}>
                      {t("secrets.generateToken")}
                    </Tooltip.Content>
                  </Tooltip.Portal>
                </Tooltip.Root>
              {/if}
            </div>
          </div>

          <div class="row-actions">
            <Tooltip.Root>
              <Tooltip.Trigger
                class={`action-btn${copied === secret.id ? " is-copied" : ""}`}
                type="button"
                onclick={() => void copyValue(secret)}
                aria-label={t("secrets.copyValue")}
              >
                {#if copied === secret.id}
                  <Check size={13.5} strokeWidth={2.4} />
                {:else}
                  <Copy size={13.5} />
                {/if}
              </Tooltip.Trigger>
              <Tooltip.Portal>
                <Tooltip.Content class="tooltip-content" sideOffset={6}>
                  {copied === secret.id ? t("secrets.copiedFeedback") : t("secrets.copyValue")}
                </Tooltip.Content>
              </Tooltip.Portal>
            </Tooltip.Root>

            <Tooltip.Root>
              <Tooltip.Trigger
                class="action-btn danger-btn"
                type="button"
                {disabled}
                onclick={() => onRequestDeleteVariable(secret.id)}
                aria-label={t("secrets.deleteVariable")}
              >
                <Trash2 size={13.5} />
              </Tooltip.Trigger>
              <Tooltip.Portal>
                <Tooltip.Content class="tooltip-content" sideOffset={6}>
                  {t("secrets.deleteVariable")}
                </Tooltip.Content>
              </Tooltip.Portal>
            </Tooltip.Root>
          </div>
        </div>
      {/each}

      {#if !disabled}
        <div class="variable-row draft-row">
          <input
            class="input key-input draft-input"
            aria-label={t("secrets.addVariablePlaceholder")}
            placeholder={`+ ${t("secrets.addVariablePlaceholder")}`}
            bind:value={draftKey}
            onkeydown={(e) => {
              if (e.key === "Enter") handleDraftSubmit();
            }}
            onpaste={splitDraftAssignment}
          />
          <div class="value-input-wrapper draft-value-wrapper">
            <input
              class="input value-input draft-input"
              placeholder={t("secrets.valuePlaceholder")}
              bind:value={draftValue}
              onkeydown={(e) => {
                if (e.key === "Enter") handleDraftSubmit();
              }}
            />
            <div class="value-inner-actions">
              <Tooltip.Root>
                <Tooltip.Trigger
                  class="row-icon-btn"
                  type="button"
                  onclick={generateForDraft}
                  aria-label={t("secrets.generateToken")}
                >
                  <Wand2 size={13} />
                </Tooltip.Trigger>
                <Tooltip.Portal>
                  <Tooltip.Content class="tooltip-content" sideOffset={6}>
                    {t("secrets.generateToken")}
                  </Tooltip.Content>
                </Tooltip.Portal>
              </Tooltip.Root>
            </div>
          </div>
          <div class="row-actions">
            {#if draftKey.trim()}
              <Button.Root class="confirm-draft-btn" type="button" onclick={handleDraftSubmit}>
                <Check size={13} strokeWidth={2.4} />
              </Button.Root>
            {/if}
          </div>
        </div>
      {/if}
    </ScrollArea.Viewport>
    <ScrollArea.Scrollbar class="scrollbar" orientation="vertical">
      <ScrollArea.Thumb class="scroll-thumb" />
    </ScrollArea.Scrollbar>
  </ScrollArea.Root>
</div>

<style>
  .table-container {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    width: 100%;
  }

  .readonly-banner {
    display: flex;
    align-items: center;
    gap: 8px;
    background: #fffbeb;
    border: 1px solid #fef3c7;
    border-radius: 6px;
    color: #92400e;
    font-size: 11.5px;
    font-weight: 550;
    padding: 7px 11px;
    margin-bottom: 10px;
  }
  :global(.lock-icon) {
    color: #d97706;
    flex-shrink: 0;
  }

  .toolbar-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--color-boulder-100);
    margin-bottom: 4px;
  }

  .search-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    flex: 1;
    max-width: 320px;
  }
  :global(.search-icon) {
    position: absolute;
    left: 9px;
    color: var(--color-boulder-400);
    pointer-events: none;
  }
  .search-input {
    width: 100%;
    height: 30px;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    background: var(--color-boulder-50);
    color: var(--color-boulder-850);
    font: inherit;
    font-size: 12px;
    padding: 0 28px 0 30px;
    outline: none;
    transition: all 0.15s ease;
  }
  .search-input:focus {
    background: #fff;
    border-color: var(--color-east-bay-400);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 16%);
  }
  .clear-query-btn {
    position: absolute;
    right: 6px;
    background: transparent;
    border: 0;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: flex;
    align-items: center;
    padding: 2px;
  }
  .clear-query-btn:hover {
    color: var(--color-boulder-700);
  }

  .toolbar-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .count-tag {
    font-size: 11px;
    color: var(--color-boulder-500);
    font-weight: 500;
  }
  :global(.ghost-action-btn) {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    background: transparent;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-700);
    font: inherit;
    font-size: 11.5px;
    font-weight: 600;
    height: 30px;
    padding: 0 9px;
    cursor: pointer;
    transition: all 0.12s ease;
  }
  :global(.ghost-action-btn:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-900);
  }
  :global(.add-variable-btn) {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 6px;
    color: #fff;
    font: inherit;
    font-size: 11.5px;
    font-weight: 650;
    height: 30px;
    padding: 0 11px;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }
  :global(.add-variable-btn:hover) {
    opacity: 0.92;
  }
  :global(.add-variable-btn:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .clipboard-error {
    background: #fef2f2;
    border: 1px solid #fee2e2;
    border-radius: 6px;
    color: #b91c1c;
    font-size: 11px;
    padding: 6px 10px;
    margin: 4px 0 8px;
  }

  :global(.variables-scroll) {
    flex: 1;
    min-height: 0;
    width: 100%;
  }
  :global(.variables-table) {
    height: 100%;
    width: 100%;
  }

  .table-header {
    display: grid;
    grid-template-columns: minmax(180px, 1.2fr) minmax(240px, 2fr) 68px;
    gap: 10px;
    padding: 8px 4px 6px;
    border-bottom: 1px solid var(--color-boulder-150);
  }
  .col-head {
    color: var(--color-boulder-500);
    font-size: 10px;
    font-weight: 750;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  .col-actions-head {
    text-align: right;
    padding-right: 4px;
  }

  .variable-row {
    display: grid;
    grid-template-columns: minmax(180px, 1.2fr) minmax(240px, 2fr) 68px;
    gap: 10px;
    padding: 5px 2px;
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
  }
  .variable-row:hover {
    background: var(--color-boulder-50);
    border-radius: 6px;
  }

  .key-field-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }
  .input {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font-family: var(--font-mono);
    font-size: 12px;
    height: 32px;
    min-width: 0;
    outline: none;
    padding: 0 9px;
    width: 100%;
    transition: all 0.12s ease;
  }
  .input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 16%);
  }
  .input:disabled {
    background: var(--color-boulder-50);
    color: var(--color-boulder-600);
    cursor: not-allowed;
  }
  .key-input {
    font-weight: 600;
  }
  .key-input.has-error {
    border-color: #ef4444;
    padding-right: 26px;
  }
  .error-tooltip-trigger {
    position: absolute;
    right: 6px;
    background: transparent;
    border: 0;
    color: #ef4444;
    display: flex;
    align-items: center;
    cursor: pointer;
    padding: 0;
  }

  .value-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }
  .value-input {
    padding-right: 58px;
    font-family: var(--font-mono);
  }
  .value-inner-actions {
    position: absolute;
    right: 4px;
    display: flex;
    align-items: center;
    gap: 2px;
  }
  :global(.row-icon-btn) {
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 24px;
    width: 24px;
    padding: 0;
    transition: all 0.12s ease;
  }
  :global(.row-icon-btn:hover) {
    background: var(--color-boulder-150);
    color: var(--color-boulder-800);
  }

  .row-actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 3px;
  }
  :global(.action-btn) {
    background: transparent;
    border: 1px solid transparent;
    border-radius: 5px;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 28px;
    width: 28px;
    padding: 0;
    transition: all 0.12s ease;
  }
  :global(.action-btn:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
    border-color: var(--color-boulder-200);
  }
  :global(.action-btn.is-copied) {
    background: #dcfce7;
    color: #15803d;
    border-color: #bbf7d0;
  }
  :global(.action-btn.danger-btn:hover) {
    background: #fef2f2;
    color: #b91c1c;
    border-color: #fecaca;
  }
  :global(.action-btn:disabled) {
    opacity: 0.35;
    cursor: not-allowed;
  }

  .draft-row {
    border-top: 1px dashed var(--color-boulder-200);
    border-bottom: 0;
    margin-top: 4px;
    background: rgb(250 250 250 / 60%);
    border-radius: 6px;
  }
  .draft-input {
    border-style: dashed;
    background: #fff;
  }
  :global(.confirm-draft-btn) {
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 5px;
    color: #fff;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    height: 26px;
    width: 26px;
    padding: 0;
  }

  .empty-env-state,
  .empty-filter-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 16px;
    text-align: center;
    color: var(--color-boulder-500);
  }
  :global(.empty-icon) {
    color: var(--color-east-bay-400);
    margin-bottom: 10px;
  }
  .empty-env-state h4 {
    margin: 0;
    color: var(--color-boulder-900);
    font-size: 14px;
    font-weight: 650;
  }
  .empty-env-state p {
    margin: 6px 0 16px;
    font-size: 12px;
    color: var(--color-boulder-500);
    max-width: 380px;
  }
  .empty-actions {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    justify-content: center;
  }
  :global(.primary-start-btn) {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: var(--color-east-bay-900);
    color: #fff;
    border: 0;
    border-radius: 6px;
    padding: 7px 12px;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    cursor: pointer;
  }
  :global(.secondary-start-btn) {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: #fff;
    color: var(--color-boulder-800);
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    padding: 7px 12px;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    cursor: pointer;
  }
  :global(.reset-filter-btn) {
    background: var(--color-boulder-100);
    border: 1px solid var(--color-boulder-200);
    border-radius: 5px;
    color: var(--color-boulder-800);
    font: inherit;
    font-size: 11px;
    font-weight: 600;
    padding: 5px 10px;
    cursor: pointer;
    margin-top: 8px;
  }

  :global(.scrollbar) {
    display: flex;
    padding: 2px;
    width: 6px;
  }
  :global(.scroll-thumb) {
    background: var(--color-boulder-300);
    border-radius: 99px;
    flex: 1;
  }
  :global(.tooltip-content) {
    background: var(--color-boulder-950);
    border-radius: 5px;
    color: #fff;
    font-size: 11px;
    font-weight: 550;
    padding: 4px 8px;
    z-index: 150;
  }
  :global(.tooltip-content.error-tooltip) {
    background: #b91c1c;
  }
</style>
