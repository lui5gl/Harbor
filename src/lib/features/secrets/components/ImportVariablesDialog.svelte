<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { FileUp, Info } from "@lucide/svelte";
  import { parseEnvironment } from "../application/environmentFormat";

  type ParsedVariable = { key: string; value: string };
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
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="flex w-full max-w-lg flex-col gap-3.5 rounded-xl border border-boulder-200 bg-white p-5 shadow-2xl outline-none"
      aria-describedby="import-dialog-desc"
    >
      <div class="flex gap-3">
        <div
          class="flex size-9.5 shrink-0 items-center justify-center rounded-lg border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        >
          <FileUp size={18} strokeWidth={2.2} />
        </div>
        <div>
          <Dialog.Title class="text-base font-bold text-boulder-950">Import variables</Dialog.Title
          ><Dialog.Description
            id="import-dialog-desc"
            class="mt-0.5 text-[13px] leading-relaxed text-boulder-600"
            >Paste environment variables from a .env file (KEY=VALUE format).</Dialog.Description
          >
        </div>
      </div>
      <div class="flex flex-col gap-1.5">
        <textarea
          class="w-full resize-y rounded-md border border-boulder-300 bg-white p-2.5 font-mono text-xs leading-normal text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-3 focus:ring-east-bay-500/15"
          bind:value={rawContent}
          placeholder="DATABASE_URL=postgres://user:pass@localhost:5432/db&#10;API_KEY=secret-token&#10;PORT=3000"
          rows={7}
          spellcheck="false"></textarea>
        <div class="flex min-h-5 items-center">
          {#if rawContent.trim().length > 0}
            <span class="inline-flex items-center gap-1 text-[11.5px] font-medium text-east-bay-800"
              ><Info size={13} strokeWidth={2} /><span
                >Found <strong>{parseResult.variables.length}</strong>
                {parseResult.variables.length === 1 ? "variable" : "variables"} ({conflicts.length} existing
                keys will be overwritten)</span
              ></span
            >
          {:else}<span class="text-[11.5px] text-boulder-500"
              >Lines with # comments and export keywords are supported.</span
            >{/if}
        </div>
      </div>
      {#if parseResult.errors.length}<p
          class="rounded-md border border-red-200 bg-red-50 p-2 text-xs text-red-700"
          role="alert"
        >
          {parseResult.errors.join("; ")}
        </p>{/if}
      {#if conflicts.length}<p class="text-xs text-amber-800">
          Conflicting keys: {conflicts.map((variable) => variable.key).join(", ")}
        </p>{/if}
      <label class="flex cursor-pointer items-center gap-2 text-xs text-boulder-700"
        ><input
          class="size-4 accent-east-bay-700"
          type="checkbox"
          bind:checked={replaceExisting}
        /><span>Replace all existing variables in this profile</span></label
      >
      {#if destructive}<label
          class="flex cursor-pointer items-center gap-2 text-xs text-boulder-700"
          ><input class="size-4 accent-red-600" type="checkbox" bind:checked={confirmed} /><span
            >Confirm replacement of existing variables</span
          ></label
        >{/if}
      <div class="flex items-center justify-end gap-2">
        <Dialog.CloseTrigger
          class="h-8.5 cursor-pointer rounded-md bg-boulder-100 px-3 text-xs font-semibold text-boulder-800 hover:bg-boulder-200"
          >Cancel</Dialog.CloseTrigger
        >
        <button
          class="inline-flex h-8.5 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-semibold text-white hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
          type="button"
          disabled={!canImport}
          onclick={handleImport}
          ><FileUp size={15} strokeWidth={2.2} aria-hidden="true" /><span
            >Import {parseResult.variables.length > 0
              ? `(${parseResult.variables.length})`
              : ""}</span
          ></button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
