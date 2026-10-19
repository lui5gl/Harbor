<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Check, Copy, KeyRound, RefreshCw, ShieldCheck } from "@lucide/svelte";
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

  const generatorTypes: { value: GeneratorType; label: string }[] = [
    { value: "hex256", label: "Hex 256-bit" },
    { value: "hex128", label: "Hex 128-bit" },
    { value: "base64", label: "Base64 (32B)" },
    { value: "token", label: "Alphanumeric" },
    { value: "password", label: "Password" },
    { value: "uuid", label: "UUID v4" },
  ];

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
      window.setTimeout(() => (isCopied = false), 1500);
    } catch {
      isCopied = false;
    }
  }

  function selectGeneratorType(type: GeneratorType) {
    generatorType = type;
    regenerate();
  }

  function handleApply() {
    onApply(targetKey.trim(), generatedValue);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="flex max-h-[calc(100dvh-2rem)] w-full max-w-lg flex-col gap-4 overflow-y-auto rounded-xl border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
      aria-describedby="generator-description"
    >
      <div class="flex items-start gap-3">
        <div
          class="flex size-9.5 shrink-0 items-center justify-center rounded-lg border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        >
          <KeyRound size={20} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="text-base font-bold text-boulder-950"
            >Generate secure secret</Dialog.Title
          ><Dialog.Description
            id="generator-description"
            class="mt-1 text-[13px] leading-snug text-boulder-600"
            >Create cryptographically strong secrets, tokens, or encryption keys.</Dialog.Description
          >
        </div>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-xs font-semibold text-boulder-700" for="generator-target-key"
          >Variable key (optional)</label
        >
        <input
          id="generator-target-key"
          class="h-9 w-full rounded-md border border-boulder-300 bg-white px-2.5 text-[13px] text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-3 focus:ring-east-bay-500/15"
          placeholder="e.g. JWT_SECRET, ENCRYPTION_KEY"
          bind:value={targetKey}
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <span class="text-xs font-semibold text-boulder-700">Secret format</span>
        <div class="grid grid-cols-3 gap-1.5">
          {#each generatorTypes as type (type.value)}
            <button
              type="button"
              class={`flex min-h-8 cursor-pointer items-center justify-center rounded-md border px-2 text-[11.5px] font-semibold transition-colors ${generatorType === type.value ? "border-east-bay-500 bg-east-bay-50 text-east-bay-900" : "border-boulder-200 bg-boulder-50 text-boulder-700 hover:border-boulder-300 hover:bg-boulder-100"}`}
              onclick={() => selectGeneratorType(type.value)}>{type.label}</button
            >
          {/each}
        </div>
      </div>

      {#if generatorType === "password"}
        <div class="flex flex-col gap-2 rounded-md border border-boulder-200 bg-boulder-50 p-3">
          <label for="password-length-slider" class="text-xs text-boulder-700"
            >Length: <strong>{passwordLength} characters</strong></label
          >
          <input
            id="password-length-slider"
            type="range"
            min="16"
            max="64"
            step="1"
            class="w-full accent-east-bay-700"
            bind:value={passwordLength}
            oninput={regenerate}
          />
        </div>
      {/if}

      <div class="rounded-lg border border-boulder-200 bg-boulder-50 p-3">
        <div class="mb-2 flex items-center justify-between">
          <span class="text-[11px] font-semibold uppercase text-boulder-500">Generated value</span>
          <button
            type="button"
            class="inline-flex cursor-pointer items-center gap-1 text-[11.5px] font-semibold text-east-bay-700 hover:text-east-bay-950"
            onclick={regenerate}
            aria-label="Regenerate"
            ><RefreshCw size={14} strokeWidth={2.2} /><span>Regenerate</span></button
          >
        </div>
        <div class="flex items-center gap-2 rounded-md border border-boulder-200 bg-white p-2.5">
          <code class="min-w-0 flex-1 break-all font-mono text-xs text-boulder-800"
            >{generatedValue}</code
          >
          <button
            type="button"
            class={`inline-flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md transition-colors ${isCopied ? "bg-emerald-50 text-emerald-600" : "text-boulder-500 hover:bg-boulder-100 hover:text-boulder-900"}`}
            onclick={copyValue}
            aria-label="Copy to clipboard"
            >{#if isCopied}<Check size={15} strokeWidth={2.4} />{:else}<Copy
                size={15}
                strokeWidth={2}
              />{/if}</button
          >
        </div>
      </div>

      <div class="flex items-center justify-end gap-2">
        <Dialog.CloseTrigger
          class="h-9 cursor-pointer rounded-md bg-boulder-100 px-3 text-sm font-semibold text-boulder-800 hover:bg-boulder-200"
          >Cancel</Dialog.CloseTrigger
        >
        <button
          class="inline-flex h-9 cursor-pointer items-center gap-2 rounded-md bg-east-bay-900 px-3.5 text-sm font-semibold text-white hover:bg-east-bay-950"
          type="button"
          onclick={handleApply}
          ><ShieldCheck size={16} strokeWidth={2.2} /><span>Insert into profile</span></button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
