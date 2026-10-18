<script lang="ts">
  import { Check, Copy, Eye, EyeOff, Trash2, Search } from "@lucide/svelte";
  import { Button, ScrollArea } from "bits-ui";
  import { onMount } from "svelte";
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
  let query = $state("");
  let clipboardError = $state("");
  let revealTimer: number | undefined;
  let secrets = $derived(
    environment.secrets.filter(
      (secret) =>
        (secret.key.trim() || secret.value.trim()) &&
        secret.key.toLowerCase().includes(query.trim().toLowerCase()),
    ),
  );
  let allRevealed = $derived(
    environment.secrets.length > 0 &&
      environment.secrets.every((secret) => revealed.includes(secret.id)),
  );

  function toggle(id: number) {
    revealed = revealed.includes(id) ? revealed.filter((item) => item !== id) : [...revealed, id];
    resetRevealTimer();
  }

  function toggleAll() {
    revealed = allRevealed ? [] : environment.secrets.map((secret) => secret.id);
    resetRevealTimer();
  }

  function resetRevealTimer() {
    window.clearTimeout(revealTimer);
    revealTimer = window.setTimeout(() => (revealed = []), 30_000);
  }

  $effect(() => {
    environment.id;
    revealed = [];
    query = "";
    draftKey = "";
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
      clipboardError = "Unable to copy the secret. Check clipboard permissions.";
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
  }

  function updateDraftKey(value: string) {
    if (!value.trim()) {
      draftKey = "";
      return;
    }
    const id = onAddVariable();
    if (id === null) return;
    onUpdateVariable(id, "key", value);
    draftKey = "";
  }
</script>

<label class="variable-search"
  ><Search size={15} /><input
    type="search"
    aria-label="Search variable names"
    placeholder="Search variable names"
    bind:value={query}
  /></label
>
{#if clipboardError}<p role="alert">{clipboardError}</p>{/if}
<ScrollArea.Root class="variables-scroll" type="auto">
  <ScrollArea.Viewport class="variables-table">
    <div class="table-header">
      <span>Key</span>
      <span>
        Value
        <Button.Root type="button" onclick={toggleAll}>
          {#if allRevealed}<EyeOff size={12} />Hide all{:else}<Eye size={12} />Show all{/if}
        </Button.Root>
      </span>
      <span aria-hidden="true"></span>
    </div>
    {#each secrets as secret (secret.id)}
      {@const shown = revealed.includes(secret.id)}
      <div class="variable-row">
        <input
          class="input"
          {disabled}
          aria-label={`Variable name ${secret.key}`}
          spellcheck="false"
          placeholder="VARIABLE_NAME"
          value={secret.key}
          oninput={(event) => onUpdateVariable(secret.id, "key", event.currentTarget.value)}
          onpaste={(event) => splitAssignment(secret, event)}
        />
        <div class="value-input">
          <input
            class="input"
            {disabled}
            aria-label={`Value for ${secret.key}`}
            autocomplete="off"
            spellcheck="false"
            type={shown ? "text" : "password"}
            placeholder="Value"
            value={secret.value}
            oninput={(event) => onUpdateVariable(secret.id, "value", event.currentTarget.value)}
          />
          <Button.Root
            type="button"
            onclick={() => toggle(secret.id)}
            aria-label="Toggle visibility"
            title="Toggle visibility"
          >
            {#if shown}<EyeOff size={14} />{:else}<Eye size={14} />{/if}
          </Button.Root>
        </div>
        <div class="row-actions">
          <Button.Root
            type="button"
            onclick={() => void copyValue(secret)}
            aria-label="Copy value"
            title="Copy value"
          >
            {#if copied === secret.id}<Check size={14} />{:else}<Copy size={14} />{/if}
          </Button.Root>
          <Button.Root
            class="danger-icon"
            type="button"
            {disabled}
            onclick={() => onRequestDeleteVariable(secret.id)}
            aria-label="Delete variable"
            title="Delete variable"><Trash2 size={14} /></Button.Root
          >
        </div>
      </div>
    {/each}
    <div class="variable-row draft-row">
      <input
        class="input"
        {disabled}
        aria-label="New variable name"
        placeholder="VARIABLE_NAME"
        value={draftKey}
        oninput={(event) => updateDraftKey(event.currentTarget.value)}
        onpaste={splitDraftAssignment}
      />
      <div class="value-input"><input class="input" placeholder="Value" disabled /></div>
      <div class="row-actions"></div>
    </div>
  </ScrollArea.Viewport>
  <ScrollArea.Scrollbar class="scrollbar" orientation="vertical">
    <ScrollArea.Thumb class="scroll-thumb" />
  </ScrollArea.Scrollbar>
</ScrollArea.Root>

<style>
  .variable-search {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 0;
    color: var(--color-boulder-500);
  }
  .variable-search input {
    width: 100%;
    min-width: 0;
    font: inherit;
    font-size: 12px;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    padding: 7px 9px;
  }
  :global(.variables-scroll) {
    flex: 1;
    min-height: 0;
  }
  :global(.variables-table) {
    height: 100%;
    width: 100%;
  }
  .table-header,
  .variable-row {
    display: grid;
    gap: 10px;
    grid-template-columns: minmax(120px, 0.8fr) minmax(180px, 1.5fr) 62px;
  }
  .table-header {
    color: var(--color-boulder-500);
    font-size: 10px;
    font-weight: 700;
    padding: 10px 2px 7px;
    text-transform: uppercase;
  }
  .table-header span {
    align-items: center;
    display: flex;
    gap: 6px;
  }
  .table-header :global(button) {
    color: var(--color-east-bay-700);
    font-size: 10px;
    font-weight: 650;
    text-transform: none;
  }
  .variable-row {
    border-top: 1px solid var(--color-boulder-100);
    padding: 7px 2px;
  }
  .input {
    background: #fff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 12px;
    height: 32px;
    min-width: 0;
    outline: none;
    padding: 0 8px;
    width: 100%;
  }
  .input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 15%);
  }
  .value-input {
    align-items: center;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    display: flex;
    overflow: hidden;
  }
  .value-input .input {
    border: 0;
  }
  .value-input:focus-within {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 15%);
  }
  .row-actions {
    align-items: center;
    display: flex;
    gap: 2px;
    justify-content: flex-end;
  }
  .value-input :global(button),
  .row-actions :global(button),
  .table-header :global(button) {
    align-items: center;
    background: transparent;
    border: 0;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    padding: 3px;
  }
  .row-actions :global(button) {
    border-radius: 4px;
  }
  .row-actions :global(button:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
  }
  .row-actions :global(.danger-icon:hover) {
    background: #fef2f2;
    color: #b91c1c;
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
  @media (max-width: 700px) {
    .table-header {
      display: none;
    }
    .variable-row {
      grid-template-columns: 1fr 38px;
    }
    .variable-row > :first-child {
      grid-column: 1 / -1;
    }
    .value-input {
      grid-column: 1;
      grid-row: 2;
    }
    .row-actions {
      grid-column: 2;
      grid-row: 2;
    }
  }
</style>
