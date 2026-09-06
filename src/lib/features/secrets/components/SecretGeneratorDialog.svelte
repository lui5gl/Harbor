<script lang="ts">
  import { Check, Copy, Dices, KeyRound, RefreshCw, ShieldCheck } from "@lucide/svelte";
  import { Button, Dialog } from "bits-ui";
  import { generateSecret, type GeneratorType } from "../application/secretGenerator";

  type SecretGeneratorDialogProps = {
    open: boolean;
    initialKey?: string;
    onApply: (key: string, value: string) => void;
  };

  let { open = $bindable(), initialKey = "", onApply }: SecretGeneratorDialogProps = $props();

  let targetKey = $state("");
  let generatorType = $state<GeneratorType>("hex256");
  let passwordLength = $state(32);
  let generatedValue = $state("");
  let isCopied = $state(false);

  $effect(() => {
    if (open) {
      targetKey = initialKey;
      regenerate();
      isCopied = false;
    }
  });

  function regenerate() {
    isCopied = false;
    generatedValue = generateSecret(generatorType, passwordLength);
  }

  async function copyValue() {
    try {
      await navigator.clipboard.writeText(generatedValue);
      isCopied = true;
      window.setTimeout(() => {
        isCopied = false;
      }, 1500);
    } catch {
      isCopied = false;
    }
  }

  function handleApply() {
    onApply(targetKey.trim(), generatedValue);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="generator-dialog" aria-describedby="generator-description">
      <div class="dialog-header">
        <div class="dialog-header-icon">
          <KeyRound size={20} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="dialog-title">Generate secure secret</Dialog.Title>
          <Dialog.Description id="generator-description" class="dialog-description">
            Create cryptographically strong secrets, tokens, or encryption keys.
          </Dialog.Description>
        </div>
      </div>

      <div class="field-group">
        <label class="field-label" for="generator-target-key">Variable key (optional)</label>
        <input
          id="generator-target-key"
          class="text-input"
          placeholder="e.g. JWT_SECRET, ENCRYPTION_KEY"
          bind:value={targetKey}
        />
      </div>

      <div class="field-group">
        <span class="field-label">Secret format</span>
        <div class="type-selector">
          <button
            type="button"
            class={`type-chip${generatorType === "hex256" ? " active" : ""}`}
            onclick={() => {
              generatorType = "hex256";
              regenerate();
            }}
          >
            Hex 256-bit
          </button>
          <button
            type="button"
            class={`type-chip${generatorType === "hex128" ? " active" : ""}`}
            onclick={() => {
              generatorType = "hex128";
              regenerate();
            }}
          >
            Hex 128-bit
          </button>
          <button
            type="button"
            class={`type-chip${generatorType === "base64" ? " active" : ""}`}
            onclick={() => {
              generatorType = "base64";
              regenerate();
            }}
          >
            Base64 (32B)
          </button>
          <button
            type="button"
            class={`type-chip${generatorType === "token" ? " active" : ""}`}
            onclick={() => {
              generatorType = "token";
              regenerate();
            }}
          >
            Alphanumeric
          </button>
          <button
            type="button"
            class={`type-chip${generatorType === "password" ? " active" : ""}`}
            onclick={() => {
              generatorType = "password";
              regenerate();
            }}
          >
            Password
          </button>
          <button
            type="button"
            class={`type-chip${generatorType === "uuid" ? " active" : ""}`}
            onclick={() => {
              generatorType = "uuid";
              regenerate();
            }}
          >
            UUID v4
          </button>
        </div>
      </div>

      {#if generatorType === "password"}
        <div class="length-slider-row">
          <label for="password-length-slider" class="length-label">
            Length: <strong>{passwordLength} characters</strong>
          </label>
          <input
            id="password-length-slider"
            type="range"
            min="16"
            max="64"
            step="1"
            class="slider"
            bind:value={passwordLength}
            oninput={regenerate}
          />
        </div>
      {/if}

      <div class="preview-box">
        <div class="preview-header">
          <span class="preview-label">Generated value</span>
          <button
            type="button"
            class="regenerate-button"
            onclick={regenerate}
            aria-label="Regenerate"
          >
            <RefreshCw size={14} strokeWidth={2.2} />
            <span>Regenerate</span>
          </button>
        </div>
        <div class="preview-content">
          <code class="preview-code">{generatedValue}</code>
          <button type="button" class="copy-btn" onclick={copyValue} aria-label="Copy to clipboard">
            {#if isCopied}
              <Check size={15} strokeWidth={2.4} class="copied-icon" />
            {:else}
              <Copy size={15} strokeWidth={2} />
            {/if}
          </button>
        </div>
      </div>

      <div class="dialog-actions">
        <Dialog.Close class="secondary-button">Cancel</Dialog.Close>
        <Button.Root class="primary-button" type="button" onclick={handleApply}>
          <ShieldCheck size={16} strokeWidth={2.2} />
          <span>Insert into profile</span>
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.modal-backdrop) {
    background: rgb(11 11 11 / 45%);
    inset: 0;
    position: fixed;
    z-index: 100;
  }

  :global(.generator-dialog) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow: 0 20px 60px rgb(11 11 11 / 22%);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 16px;
    left: 50%;
    max-width: 500px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: calc(100vw - 32px);
    z-index: 101;
  }

  .dialog-header {
    align-items: flex-start;
    display: flex;
    gap: 12px;
  }

  .dialog-header-icon {
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
    color: var(--color-boulder-950);
    font-size: 16px;
    font-weight: 700;
    margin: 0;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.4;
    margin: 4px 0 0;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 600;
  }

  .text-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 13px;
    min-height: 36px;
    outline: none;
    padding: 0 10px;
    width: 100%;
  }

  .text-input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 16%);
  }

  .type-selector {
    display: grid;
    gap: 6px;
    grid-template-columns: repeat(3, 1fr);
  }

  .type-chip {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: flex;
    font-size: 11.5px;
    font-weight: 600;
    justify-content: center;
    min-height: 32px;
    padding: 0 8px;
    transition:
      background 0.12s ease,
      border-color 0.12s ease,
      color 0.12s ease;
  }

  .type-chip:hover {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-300);
  }

  .type-chip.active {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-500);
    color: var(--color-east-bay-900);
  }

  .length-slider-row {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    display: flex;
    justify-content: space-between;
    padding: 8px 12px;
    gap: 12px;
  }

  .length-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    white-space: nowrap;
  }

  .slider {
    accent-color: var(--color-east-bay-700);
    cursor: pointer;
    flex: 1;
  }

  .preview-box {
    background: var(--color-boulder-900);
    border: 1px solid var(--color-boulder-800);
    border-radius: 8px;
    padding: 12px;
  }

  .preview-header {
    align-items: center;
    display: flex;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .preview-label {
    color: var(--color-boulder-400);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .regenerate-button {
    align-items: center;
    background: transparent;
    border: 0;
    color: var(--color-boulder-300);
    cursor: pointer;
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 4px;
    padding: 0;
    transition: color 0.12s ease;
  }

  .regenerate-button:hover {
    color: #ffffff;
  }

  .preview-content {
    align-items: center;
    display: flex;
    gap: 8px;
    justify-content: space-between;
  }

  .preview-code {
    color: #4ade80;
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 12.5px;
    word-break: break-all;
    user-select: all;
  }

  .copy-btn {
    align-items: center;
    background: rgb(255 255 255 / 10%);
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-200);
    cursor: pointer;
    display: inline-flex;
    height: 30px;
    justify-content: center;
    width: 30px;
    flex-shrink: 0;
    transition:
      background 0.12s ease,
      color 0.12s ease;
  }

  .copy-btn:hover {
    background: rgb(255 255 255 / 20%);
    color: #ffffff;
  }

  :global(.copied-icon) {
    color: #4ade80;
  }

  .dialog-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 4px;
  }
</style>
