<script lang="ts">
  import { FileUp, Info } from "@lucide/svelte";
  import { Button, Dialog } from "bits-ui";

  type ParsedVariable = {
    key: string;
    value: string;
  };

  type ImportVariablesDialogProps = {
    open: boolean;
    onImport: (variables: ParsedVariable[], replaceAll: boolean) => void;
  };

  let { open = $bindable(), onImport }: ImportVariablesDialogProps = $props();

  let rawContent = $state("");
  let replaceExisting = $state(false);

  function parseEnvContent(text: string): { valid: ParsedVariable[]; ignoredCount: number } {
    const lines = text.split(/\r?\n/);
    const valid: ParsedVariable[] = [];
    const seenKeys = new Set<string>();
    let ignoredCount = 0;

    for (const rawLine of lines) {
      let line = rawLine.trim();
      if (!line || line.startsWith("#")) {
        continue;
      }

      if (line.startsWith("export ")) {
        line = line.slice(7).trim();
      }

      const equalsIndex = line.indexOf("=");
      if (equalsIndex <= 0) {
        ignoredCount++;
        continue;
      }

      let key = line.slice(0, equalsIndex).trim();
      let value = line.slice(equalsIndex + 1).trim();

      // Strip matching quotes around value
      if (
        (value.startsWith('"') && value.endsWith('"') && value.length >= 2) ||
        (value.startsWith("'") && value.endsWith("'") && value.length >= 2)
      ) {
        value = value.slice(1, -1);
      }

      // Check key validity
      if (!/^[a-zA-Z0-9_]+$/.test(key) || key.length > 256) {
        ignoredCount++;
        continue;
      }

      // Avoid duplicates within the pasted block (keep last or first)
      if (seenKeys.has(key.toUpperCase())) {
        // Update existing in parsed list
        const existing = valid.find((v) => v.key.toUpperCase() === key.toUpperCase());
        if (existing) existing.value = value;
      } else {
        seenKeys.add(key.toUpperCase());
        valid.push({ key, value });
      }
    }

    return { valid, ignoredCount };
  }

  let parseResult = $derived(parseEnvContent(rawContent));
  let canImport = $derived(parseResult.valid.length > 0);

  $effect(() => {
    if (!open) {
      rawContent = "";
      replaceExisting = false;
    }
  });

  function handleImport() {
    if (!canImport) return;
    onImport(parseResult.valid, replaceExisting);
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
          spellcheck="false"
        ></textarea>

        <div class="parse-summary">
          {#if rawContent.trim().length > 0}
            <span class="summary-text">
              <Info size={13} strokeWidth={2} />
              <span>
                Found <strong>{parseResult.valid.length}</strong> {parseResult.valid.length === 1 ? "variable" : "variables"}
                {#if parseResult.ignoredCount > 0}
                  ({parseResult.ignoredCount} ignored)
                {/if}
              </span>
            </span>
          {:else}
            <span class="summary-text muted">Lines with # comments and export keywords are supported.</span>
          {/if}
        </div>
      </div>

      <label class="replace-option">
        <input type="checkbox" bind:checked={replaceExisting} />
        <span>Replace all existing variables in this profile</span>
      </label>

      <div class="dialog-actions">
        <Dialog.Close class="secondary-button">Cancel</Dialog.Close>
        <Button.Root
          class="primary-button"
          type="button"
          disabled={!canImport}
          onclick={handleImport}
        >
          <FileUp size={15} strokeWidth={2.2} aria-hidden="true" />
          <span>Import {parseResult.valid.length > 0 ? `(${parseResult.valid.length})` : ""}</span>
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
    transition: border-color 0.15s ease, box-shadow 0.15s ease;
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
    transition: background 0.12s ease, opacity 0.12s ease;
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
