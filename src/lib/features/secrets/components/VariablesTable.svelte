<script lang="ts">
  import { t } from "#lib/i18n";
  import { Menu } from "@ark-ui/svelte/menu";
  import {
    ArrowDown,
    ArrowUp,
    ArrowUpDown,
    AlertCircle,
    Check,
    Copy,
    Eye,
    EyeOff,
    KeyRound,
    Lock,
    MoreHorizontal,
    Plus,
    Search,
    Trash2,
    Wand2,
    X,
  } from "@lucide/svelte";
  import { onMount, tick } from "svelte";
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
  let pendingVariableId = $state<number | null>(null);
  let query = $state("");
  let sortField = $state<"key" | "value">("key");
  let sortDirection = $state<"asc" | "desc">("asc");
  let clipboardError = $state("");
  let previousEnvironmentId: typeof environment.id | undefined;
  let revealTimer: number | undefined;
  let totalCount = $derived(environment.secrets.length);
  let filteredSecrets = $derived(
    environment.secrets.filter(
      (secret) =>
        (secret.key.trim() || secret.value.trim() || secret.id === pendingVariableId) &&
        secret.key.toLowerCase().includes(query.trim().toLowerCase()),
    ),
  );
  let secrets = $derived(
    [...filteredSecrets].sort((first, second) => {
      const comparison = first[sortField].localeCompare(second[sortField], undefined, {
        numeric: true,
        sensitivity: "base",
      });
      return sortDirection === "asc" ? comparison : -comparison;
    }),
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
  function toggleSort(field: "key" | "value") {
    if (sortField === field) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
      return;
    }
    sortField = field;
    sortDirection = "asc";
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

  function duplicateVariable(secret: Secret) {
    if (disabled) return;
    const baseKey = secret.key.trim() || "VARIABLE";
    let key = `${baseKey}_COPY`;
    let suffix = 2;
    while (
      environment.secrets.some((item) => item.key.trim().toUpperCase() === key.toUpperCase())
    ) {
      key = `${baseKey}_COPY_${suffix++}`;
    }
    const id = onAddVariable();
    if (id === null) return;
    onUpdateVariable(id, "key", key);
    onUpdateVariable(id, "value", secret.value);
  }

  function clearValue(secretId: number) {
    if (disabled) return;
    onUpdateVariable(secretId, "value", "");
    revealed = revealed.filter((id) => id !== secretId);
  }

  $effect(() => {
    const currentEnvironmentId = environment.id;
    if (currentEnvironmentId !== previousEnvironmentId) {
      previousEnvironmentId = currentEnvironmentId;
      revealed = [];
      query = "";
      pendingVariableId = null;
    }
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

  function addVariable() {
    if (disabled || pendingVariableId !== null) return;
    query = "";
    pendingVariableId = onAddVariable();
  }

  async function addNextVariable() {
    if (disabled) return;
    if (pendingVariableId === null) addVariable();
    await tick();
    document.querySelector<HTMLInputElement>("[data-pending-variable='true']")?.focus();
  }

  function updateKey(id: number, value: string) {
    onUpdateVariable(id, "key", value);
    if (id === pendingVariableId && value.trim()) pendingVariableId = null;
  }

  const inputClass =
    "h-8 min-w-0 w-full rounded border border-transparent bg-transparent px-2.5 font-mono text-xs text-boulder-900 outline-none transition focus:border-east-bay-500 focus:bg-white focus:ring-2 focus:ring-east-bay-500/15 disabled:cursor-not-allowed disabled:text-boulder-600";
  const menuContentClass =
    "z-100 min-w-40 rounded-md border border-boulder-200 bg-white p-1.5 shadow-lg outline-none";
  const menuItemClass =
    "flex h-8 cursor-pointer items-center gap-2 rounded px-2 text-xs font-medium text-boulder-700 outline-none transition-colors data-[highlighted]:bg-boulder-100 data-[highlighted]:text-boulder-950 data-[disabled]:cursor-not-allowed data-[disabled]:opacity-40";
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
    <div class="relative flex min-w-0 flex-1 items-center">
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
        onclick={addVariable}
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

  <div class="flex min-h-0 flex-1 flex-col overflow-y-auto rounded-md border border-boulder-200">
    <div
      class="grid shrink-0 grid-cols-[40px_minmax(180px,1.2fr)_minmax(200px,2fr)_104px] gap-0 border-b border-boulder-200 bg-boulder-50 text-[11px] font-semibold text-boulder-600"
    >
      <span
        class="flex items-center justify-center border-r border-boulder-200 px-1 py-2"
        aria-label="#">#</span
      >
      <span class="border-r border-boulder-200 px-3 py-1.5">
        <button
          class="flex w-full cursor-pointer items-center gap-1.5 text-left hover:text-boulder-900"
          type="button"
          onclick={() => toggleSort("key")}
          aria-label={`${t("secrets.sortBy")} ${t("secrets.key")}`}
          title={`${t("secrets.sortBy")} ${t("secrets.key")}`}
          >{t(
            "secrets.key",
          )}{#if sortField === "key"}{#if sortDirection === "asc"}<ArrowUp
                size={12}
              />{:else}<ArrowDown size={12} />{/if}{:else}<ArrowUpDown
              size={12}
              class="opacity-40"
            />{/if}</button
        >
      </span>
      <span class="border-r border-boulder-200 px-3 py-1.5">
        <button
          class="flex w-full cursor-pointer items-center gap-1.5 text-left hover:text-boulder-900"
          type="button"
          onclick={() => toggleSort("value")}
          aria-label={`${t("secrets.sortBy")} ${t("secrets.value")}`}
          title={`${t("secrets.sortBy")} ${t("secrets.value")}`}
          >{t(
            "secrets.value",
          )}{#if sortField === "value"}{#if sortDirection === "asc"}<ArrowUp
                size={12}
              />{:else}<ArrowDown size={12} />{/if}{:else}<ArrowUpDown
              size={12}
              class="opacity-40"
            />{/if}</button
        >
      </span>
      <span class="flex items-center justify-start px-2 py-2">{t("secrets.variableActions")}</span>
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
    {:else if secrets.length === 0}
      <div class="flex flex-col items-center py-12 text-center">
        <div
          class="mb-3 flex size-12 items-center justify-center rounded-full border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        >
          <KeyRound size={22} strokeWidth={1.8} />
        </div>
        <h4 class="text-sm font-semibold text-boulder-950">{t("secrets.noVariablesTitle")}</h4>
        <p class="mt-1 max-w-sm text-xs text-boulder-500">{t("secrets.noVariablesDesc")}</p>
      </div>
    {/if}

    {#each secrets as secret, index (secret.id)}
      {@const shown = revealed.includes(secret.id)}
      {@const keyError = secret.key.trim() ? validateSecretKey(secret.key) : null}
      <div
        class="grid shrink-0 grid-cols-[40px_minmax(180px,1.2fr)_minmax(200px,2fr)_104px] items-center gap-0 border-b border-boulder-100 transition-colors hover:bg-boulder-50"
      >
        <div
          class="flex h-full items-center justify-center border-r border-boulder-100 px-1 text-[11px] tabular-nums text-boulder-500"
        >
          {index + 1}
        </div>
        <div class="relative flex items-center border-r border-boulder-100 px-1.5 py-1">
          <input
            class={`${inputClass} font-semibold ${keyError ? "border-red-500 pr-7" : ""}`}
            {disabled}
            data-pending-variable={secret.id === pendingVariableId ? "true" : undefined}
            aria-label={`${t("secrets.key")} ${secret.key}`}
            spellcheck="false"
            placeholder="VARIABLE_NAME"
            value={secret.key}
            oninput={(event) => updateKey(secret.id, event.currentTarget.value)}
            onpaste={(event) => splitAssignment(secret, event)}
          />
          {#if keyError}<span class="absolute right-1.5 inline-flex text-red-500" title={keyError}
              ><AlertCircle size={13} /></span
            >{/if}
        </div>
        <div class="relative flex items-center border-r border-boulder-100 px-1.5 py-1">
          <input
            class={inputClass}
            {disabled}
            aria-label={`${t("secrets.value")} ${secret.key}`}
            autocomplete="off"
            spellcheck="false"
            type={shown ? "text" : "password"}
            placeholder="Valor..."
            value={secret.value}
            oninput={(event) => onUpdateVariable(secret.id, "value", event.currentTarget.value)}
            onkeydown={(event) => {
              if (event.key === "Enter") {
                event.preventDefault();
                void addNextVariable();
              }
            }}
          />
        </div>
        <div class="flex items-center justify-start gap-0.5 px-1.5 py-1">
          <button
            class="inline-flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-boulder-500 transition-colors hover:bg-boulder-100 hover:text-boulder-800"
            type="button"
            onclick={() => void copyValue(secret)}
            title={copied === secret.id ? t("secrets.copiedFeedback") : t("secrets.copyValue")}
            aria-label={t("secrets.copyValue")}
            >{#if copied === secret.id}<Check size={15} />{:else}<Copy size={15} />{/if}</button
          >
          <button
            class="inline-flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md text-boulder-500 transition-colors hover:bg-boulder-100 hover:text-boulder-800"
            type="button"
            onclick={() => toggle(secret.id)}
            title={shown ? "Ocultar valor" : "Revelar valor (30s)"}
            aria-label={shown ? "Ocultar" : "Mostrar"}
            >{#if shown}<EyeOff size={15} />{:else}<Eye size={15} />{/if}</button
          >
          <Menu.Root positioning={{ placement: "bottom-end", gutter: 4 }}>
            <Menu.Trigger
              class="inline-flex size-7 cursor-pointer items-center justify-center rounded-md text-boulder-500 transition-colors hover:bg-boulder-100 hover:text-boulder-800 data-[state=open]:bg-boulder-100 data-[state=open]:text-boulder-800"
              aria-label={t("secrets.variableActions")}
              title={t("secrets.variableActions")}><MoreHorizontal size={15} /></Menu.Trigger
            >
            <Menu.Positioner>
              <Menu.Content class={menuContentClass}>
                <Menu.Item
                  value="generate"
                  class={menuItemClass}
                  {disabled}
                  onSelect={() => generateInline(secret.id)}
                  ><Wand2 size={14} /><span>{t("secrets.generateToken")}</span></Menu.Item
                >
                <Menu.Item
                  value="duplicate"
                  class={menuItemClass}
                  {disabled}
                  onSelect={() => duplicateVariable(secret)}
                  ><Copy size={14} /><span>{t("secrets.duplicateVariable")}</span></Menu.Item
                >
                <Menu.Item
                  value="clear"
                  class={menuItemClass}
                  {disabled}
                  onSelect={() => clearValue(secret.id)}
                  ><X size={14} /><span>{t("secrets.clearValue")}</span></Menu.Item
                >
                <Menu.Separator class="mx-2 my-0.5 h-px border-0 bg-boulder-100" />
                <Menu.Item
                  value="delete"
                  class={`${menuItemClass} text-red-700 data-highlighted:bg-red-50 data-highlighted:text-red-700`}
                  {disabled}
                  onSelect={() => onRequestDeleteVariable(secret.id)}
                  ><Trash2 size={14} /><span>{t("secrets.deleteVariable")}</span></Menu.Item
                >
              </Menu.Content>
            </Menu.Positioner>
          </Menu.Root>
        </div>
      </div>
    {/each}

    <div
      aria-hidden="true"
      class="grid min-h-12 flex-1 grid-cols-[40px_minmax(180px,1.2fr)_minmax(200px,2fr)_104px]"
    >
      <div class="border-r border-boulder-100"></div>
      <div class="border-r border-boulder-100"></div>
      <div class="border-r border-boulder-100"></div>
      <div></div>
    </div>
  </div>
</div>
