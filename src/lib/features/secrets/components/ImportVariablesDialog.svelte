<script lang="ts">
  import { FileUp, Info } from "@lucide/svelte";
  import { Button, Dialog } from "bits-ui";
  import { parseEnvironment } from "../application/environmentFormat";

  type ParsedVariable = {
    key: string;
    value: string;
  };

  type ImportVariablesDialogProps = {
    open: boolean;
    existingKeys?: string[];
    onImport: (variables: ParsedVariable[], replaceAll: boolean) => void;
  };

  let { open = $bindable(), onImport, existingKeys = [] }: ImportVariablesDialogProps = $props();

  let rawContent = $state("");
  let replaceExisting = $state(false);

  let confirmed = $state(false);
  let parseResult = $derived.by(() => {
    try {
      return parseEnvironment(rawContent);
    } catch (error) {
      return { variables: [], errors: [String(error)], duplicates: 0 };
    }
  });
  let conflicts = $derived(
    parseResult.variables.filter((variable) =>
      existingKeys.some((key) => key.toUpperCase() === variable.key.toUpperCase()),
    ),
  );
  let destructive = $derived(replaceExisting || conflicts.length > 0);
  let canImport = $derived(
    parseResult.variables.length > 0 &&
      parseResult.errors.length === 0 &&
      (!destructive || confirmed),
  );

  $effect(() => {
    rawContent;
    replaceExisting;
    confirmed = false;
  });

  $effect(() => {
    if (!open) {
      rawContent = "";
      replaceExisting = false;
    }
  });

  function handleImport() {
    if (!canImport) return;
    onImport(parseResult.variables, replaceExisting);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="import-dialog" aria-describedby="import-dialog-desc">
      <div class="dialog-header">
        <div class="dialog-icon-box">
          <FileUp size={18} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="dialog-title">Import variables</Dialog.Title>
          <Dialog.Description id="import-dialog-desc" class="dialog-description">
            Paste environment variables from a .env file (KEY=VALUE format).
          </Dialog.Description>
        </div>
      </div>

      <div class="textarea-section">
        <textarea
          class="env-textarea"
          bind:value={rawContent}
          placeholder="DATABASE_URL=postgres://user:pass@localhost:5432/db&#10;API_KEY=secret-token&#10;PORT=3000"
          rows={7}
          spellcheck="false"></textarea>

        <div class="parse-summary">
          {#if rawContent.trim().length > 0}
            <span class="summary-text">
              <Info size={13} strokeWidth={2} />
              <span>
                Found <strong>{parseResult.variables.length}</strong>
                {parseResult.variables.length === 1 ? "variable" : "variables"}
                ({conflicts.length} existing keys will be overwritten)
              </span>
            </span>
          {:else}
            <span class="summary-text muted"
              >Lines with # comments and export keywords are supported.</span
            >
          {/if}
        </div>
      </div>

      {#if parseResult.errors.length}<p role="alert">{parseResult.errors.join("; ")}</p>{/if}
      {#if conflicts.length}<p>
          Conflicting keys: {conflicts.map((variable) => variable.key).join(", ")}
        </p>{/if}
      <label class="replace-option">
        <input type="checkbox" bind:checked={replaceExisting} />
        <span>Replace all existing variables in this profile</span>
      </label>
      {#if destructive}<label class="replace-option"
          ><input type="checkbox" bind:checked={confirmed} /><span
            >Confirm replacement of existing variables</span
          ></label
        >{/if}

      <div class="dialog-actions">
        <Dialog.Close class="secondary-button">Cancel</Dialog.Close>
        <Button.Root
          class="primary-button"
          type="button"
          disabled={!canImport}
          onclick={handleImport}
        >
          <FileUp size={15} strokeWidth={2.2} aria-hidden="true" />
          <span
            >Import {parseResult.variables.length > 0
              ? `(${parseResult.variables.length})`
              : ""}</span
          >
        </Button.Root>
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

  :global(.import-dialog) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow:
      0 20px 25px -5px rgb(0 0 0 / 15%),
      0 8px 10px -6px rgb(0 0 0 / 10%);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 14px;
    left: 50%;
    max-width: 480px;
    padding: 20px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(calc(100% - 32px), 480px);
    z-index: 101;
  }

  .dialog-header {
    display: flex;
    gap: 12px;
  }

  .dialog-icon-box {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    border-radius: 8px;
    color: var(--color-east-bay-700);
    display: flex;
    flex-shrink: 0;
    height: 38px;
    justify-content: center;
    width: 38px;
  }

  :global(.dialog-title) {
    color: var(--color-boulder-950);
    font-size: 16px;
    font-weight: 700;
    letter-spacing: -0.01em;
    margin: 0;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.45;
    margin: 2px 0 0;
  }

  .textarea-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .env-textarea {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font-family: ui-monospace, monospace;
    font-size: 12.5px;
    line-height: 1.5;
    outline: none;
    padding: 10px;
    resize: vertical;
    transition:
      border-color 0.15s ease,
      box-shadow 0.15s ease;
    width: 100%;
  }

  .env-textarea:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 16%);
  }

  .parse-summary {
    align-items: center;
    display: flex;
    min-height: 20px;
  }

  .summary-text {
    align-items: center;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 500;
    gap: 5px;
  }

  .summary-text.muted {
    color: var(--color-boulder-500);
  }

  .replace-option {
    align-items: center;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: flex;
    font-size: 12.5px;
    gap: 8px;
    user-select: none;
  }

  .dialog-actions {
    align-items: center;
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 4px;
  }

  :global(.secondary-button),
  :global(.primary-button) {
    align-items: center;
    border-radius: 6px;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 13px;
    font-weight: 600;
    gap: 6px;
    height: 34px;
    justify-content: center;
    padding: 0 14px;
    transition:
      background 0.12s ease,
      opacity 0.12s ease;
  }

  :global(.secondary-button) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    color: var(--color-boulder-700);
  }

  :global(.secondary-button:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-400);
    color: var(--color-boulder-900);
  }

  :global(.primary-button) {
    background: var(--color-east-bay-900);
    border: 1px solid var(--color-east-bay-900);
    color: #ffffff;
  }

  :global(.primary-button:hover:not(:disabled)) {
    background: var(--color-east-bay-950);
  }

  :global(.primary-button:disabled) {
    cursor: not-allowed;
    opacity: 0.5;
  }
</style>
