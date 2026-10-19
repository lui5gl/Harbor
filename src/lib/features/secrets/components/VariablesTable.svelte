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
    onUpdateVariable(secretId, "value", generateSecret("token", 32));
    if (!revealed.includes(secretId)) revealed = [...revealed, secretId];
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
    if (draftValue) onUpdateVariable(id, "value", draftValue);
    draftKey = "";
    draftValue = "";
  }

  function generateForDraft() {
    if (disabled) return;
    const id = onAddVariable();
    if (id === null) return;
    onUpdateVariable(id, "key", draftKey.trim() || "SECRET_KEY");
    onUpdateVariable(id, "value", generateSecret("token", 32));
    revealed = [...revealed, id];
    draftKey = "";
    draftValue = "";
    resetRevealTimer();
  }

  const inputClass =
    "h-8 min-w-0 w-full rounded-md border border-boulder-200 bg-white px-2.5 font-mono text-xs text-boulder-900 outline-none transition focus:border-east-bay-500 focus:ring-2 focus:ring-east-bay-500/15 disabled:cursor-not-allowed disabled:bg-boulder-50 disabled:text-boulder-600";
  const iconButtonClass =
    "inline-flex size-7 cursor-pointer items-center justify-center rounded-md border border-transparent text-boulder-500 transition-colors hover:border-boulder-200 hover:bg-boulder-100 hover:text-boulder-800 disabled:cursor-not-allowed disabled:opacity-40";
</script>

<div class="flex min-h-0 w-full flex-1 flex-col">
  {#if disabled}<div
      class="mb-2.5 flex items-center gap-2 rounded-md border border-amber-200 bg-amber-50 px-3 py-2 text-[11.5px] font-medium text-amber-800"
      role="status"
    >
      <Lock size={14} class="shrink-0 text-amber-600" /><span
        >{t("secrets.readOnlyActiveNotice")}</span
      >
    </div>{/if}

  <div class="mb-1 flex items-center justify-between gap-2.5 border-b border-boulder-100 pb-2.5">
    <div class="relative flex max-w-80 flex-1 items-center">
      <Search size={14} class="pointer-events-none absolute left-2.5 text-boulder-400" />
      <input
        type="search"
        class="h-7.5 w-full rounded-md border border-boulder-200 bg-boulder-50 pr-7 pl-7.5 text-xs text-boulder-800 outline-none focus:border-east-bay-400 focus:bg-white focus:ring-2 focus:ring-east-bay-400/15"
        aria-label={t("secrets.filterVariables")}
        placeholder={t("secrets.filterVariables")}
        bind:value={query}
      />
      {#if query}<button
          type="button"
          class="absolute right-1.5 inline-flex size-5 cursor-pointer items-center justify-center text-boulder-400 hover:text-boulder-700"
          aria-label={t("secrets.clearFilter")}
          onclick={() => (query = "")}><X size={12} /></button
        >{/if}
    </div>
    <div class="flex items-center gap-2">
      {#if query && totalCount > 0}<span class="text-[11px] font-medium text-boulder-500"
          >{secrets.length} {t("secrets.of")} {totalCount}</span
        >{/if}
      {#if secrets.length > 0}<button
          class="inline-flex h-7.5 cursor-pointer items-center gap-1.5 rounded-md border border-boulder-200 bg-white px-2.5 text-[11.5px] font-semibold text-boulder-700 hover:bg-boulder-100"
          type="button"
          onclick={toggleAll}
          >{#if allRevealed}<EyeOff size={13} /><span>{t("secrets.hideAll")}</span>{:else}<Eye
              size={13}
            /><span>{t("secrets.showAll")}</span>{/if}</button
        >{/if}
      <button
        class="inline-flex h-7.5 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-2.5 text-[11.5px] font-semibold text-white hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
        type="button"
        {disabled}
        onclick={() => onAddVariable()}
        ><Plus size={14} strokeWidth={2.4} /><span>{t("secrets.addVariable")}</span></button
      >
    </div>
  </div>

  {#if clipboardError}<p
      class="my-1 rounded-md border border-red-100 bg-red-50 px-2.5 py-1.5 text-[11px] text-red-700"
      role="alert"
    >
      {clipboardError}
    </p>{/if}

  <div class="min-h-0 flex-1 overflow-y-auto">
    <div
      class="grid grid-cols-[minmax(180px,1.2fr)_minmax(240px,2fr)_68px] gap-2.5 border-b border-boulder-200 px-1 py-2 text-[10px] font-bold uppercase text-boulder-500"
    >
      <span>{t("secrets.key")}</span><span>{t("secrets.value")}</span><span class="pr-1 text-right"
        >{t("secrets.variableActions")}</span
      >
    </div>

    {#if secrets.length === 0 && query}
      <div class="flex flex-col items-center gap-2 py-12 text-center text-xs text-boulder-500">
        <p>{t("secrets.noMatchFor")} "<strong>{query}</strong>"</p>
        <button
          class="cursor-pointer font-semibold text-east-bay-700 hover:text-east-bay-950"
          type="button"
          onclick={() => (query = "")}>{t("secrets.clearFilter")}</button
        >
      </div>
    {:else if secrets.length === 0 && !draftKey}
      <div class="flex flex-col items-center py-12 text-center">
        <div
          class="mb-3 flex size-12 items-center justify-center rounded-full border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        >
          <KeyRound size={22} strokeWidth={1.8} />
        </div>
        <h4 class="text-sm font-semibold text-boulder-950">{t("secrets.noVariablesTitle")}</h4>
        <p class="mt-1 max-w-sm text-xs text-boulder-500">{t("secrets.noVariablesDesc")}</p>
        <div class="mt-4 flex gap-2">
          <button
            class="inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-semibold text-white disabled:opacity-50"
            type="button"
            {disabled}
            onclick={() => onAddVariable()}
            ><Plus size={14} /><span>{t("secrets.addFirstVariable")}</span></button
          ><button
            class="inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-md border border-boulder-300 bg-white px-3 text-xs font-semibold text-boulder-700 disabled:opacity-50"
            type="button"
            {disabled}
            onclick={generateForDraft}
            ><Wand2 size={14} /><span>{t("secrets.generateKey")}</span></button
          >
        </div>
      </div>
    {/if}

    {#each secrets as secret (secret.id)}
      {@const shown = revealed.includes(secret.id)}
      {@const keyError = secret.key.trim() ? validateSecretKey(secret.key) : null}
      <div
        class="grid grid-cols-[minmax(180px,1.2fr)_minmax(240px,2fr)_68px] items-center gap-2.5 border-b border-boulder-100 px-0.5 py-1.5 hover:rounded-md hover:bg-boulder-50"
      >
        <div class="relative flex items-center">
          <input
            class={`${inputClass} font-semibold ${keyError ? "border-red-500 pr-7" : ""}`}
            {disabled}
            aria-label={`${t("secrets.key")} ${secret.key}`}
            spellcheck="false"
            placeholder="VARIABLE_NAME"
            value={secret.key}
            oninput={(event) => onUpdateVariable(secret.id, "key", event.currentTarget.value)}
            onpaste={(event) => splitAssignment(secret, event)}
          />
          {#if keyError}<span class="absolute right-1.5 inline-flex text-red-500" title={keyError}
              ><AlertCircle size={13} /></span
            >{/if}
        </div>
        <div class="relative flex items-center">
          <input
            class={`${inputClass} pr-15`}
            {disabled}
            aria-label={`${t("secrets.value")} ${secret.key}`}
            autocomplete="off"
            spellcheck="false"
            type={shown ? "text" : "password"}
            placeholder="Valor..."
            value={secret.value}
            oninput={(event) => onUpdateVariable(secret.id, "value", event.currentTarget.value)}
          />
          <div class="absolute right-1 flex items-center gap-0.5">
            <button
              class="inline-flex size-6 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-boulder-100 hover:text-boulder-800"
              type="button"
              onclick={() => toggle(secret.id)}
              title={shown ? "Ocultar valor" : "Revelar valor (30s)"}
              aria-label={shown ? "Ocultar" : "Mostrar"}
              >{#if shown}<EyeOff size={13} />{:else}<Eye size={13} />{/if}</button
            >
            {#if !disabled}<button
                class="inline-flex size-6 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-boulder-100 hover:text-boulder-800"
                type="button"
                onclick={() => generateInline(secret.id)}
                title={t("secrets.generateToken")}
                aria-label={t("secrets.generateToken")}><Wand2 size={13} /></button
              >{/if}
          </div>
        </div>
        <div class="flex items-center justify-end gap-0.5">
          <button
            class={`${iconButtonClass} ${copied === secret.id ? "border-emerald-200 bg-emerald-50 text-emerald-600" : ""}`}
            type="button"
            onclick={() => void copyValue(secret)}
            title={copied === secret.id ? t("secrets.copiedFeedback") : t("secrets.copyValue")}
            aria-label={t("secrets.copyValue")}
            >{#if copied === secret.id}<Check size={14} strokeWidth={2.4} />{:else}<Copy
                size={14}
              />{/if}</button
          >
          <button
            class={`${iconButtonClass} hover:border-red-100 hover:bg-red-50 hover:text-red-600`}
            type="button"
            {disabled}
            onclick={() => onRequestDeleteVariable(secret.id)}
            title={t("secrets.deleteVariable")}
            aria-label={t("secrets.deleteVariable")}><Trash2 size={14} /></button
          >
        </div>
      </div>
    {/each}

    {#if !disabled}
      <div
        class="grid grid-cols-[minmax(180px,1.2fr)_minmax(240px,2fr)_68px] items-center gap-2.5 border-b border-boulder-100 px-0.5 py-1.5"
      >
        <input
          class={`${inputClass} border-dashed`}
          aria-label={t("secrets.addVariablePlaceholder")}
          placeholder={`+ ${t("secrets.addVariablePlaceholder")}`}
          bind:value={draftKey}
          onkeydown={(event) => {
            if (event.key === "Enter") handleDraftSubmit();
          }}
          onpaste={splitDraftAssignment}
        />
        <div class="relative flex items-center">
          <input
            class={`${inputClass} border-dashed pr-8`}
            placeholder={t("secrets.valuePlaceholder")}
            bind:value={draftValue}
            onkeydown={(event) => {
              if (event.key === "Enter") handleDraftSubmit();
            }}
          /><button
            class="absolute right-1 inline-flex size-6 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-boulder-100 hover:text-boulder-800"
            type="button"
            onclick={generateForDraft}
            title={t("secrets.generateToken")}
            aria-label={t("secrets.generateToken")}><Wand2 size={13} /></button
          >
        </div>
        <div class="flex justify-end">
          {#if draftKey.trim()}<button
              class="inline-flex size-7 cursor-pointer items-center justify-center rounded-md bg-east-bay-900 text-white hover:bg-east-bay-950"
              type="button"
              onclick={handleDraftSubmit}
              aria-label={t("secrets.addVariable")}><Check size={13} strokeWidth={2.4} /></button
            >{/if}
        </div>
      </div>
    {/if}
  </div>
</div>
