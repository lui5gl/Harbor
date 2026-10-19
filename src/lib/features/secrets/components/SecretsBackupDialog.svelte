<script lang="ts">
  import { t } from "#lib/i18n";
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Download, Upload } from "@lucide/svelte";
  import { decryptBackup, encryptBackup } from "../application/secretsBackup";
  import type { SecretsConfiguration } from "../types";

  let {
    open = $bindable(),
    mode,
    configuration,
    onRestore,
  }: {
    open: boolean;
    mode: "export" | "restore";
    configuration: () => SecretsConfiguration;
    onRestore: (configuration: SecretsConfiguration) => Promise<boolean>;
  } = $props();
  let password = $state("");
  let confirmation = $state("");
  let file = $state<File | null>(null);
  let confirmed = $state(false);
  let busy = $state(false);
  let error = $state("");

  $effect(() => {
    if (!open) {
      password = "";
      confirmation = "";
      file = null;
      confirmed = false;
      error = "";
    }
  });

  function handleOpenChange(details: { open: boolean }) {
    if (busy && !details.open) {
      open = true;
    }
  }

  async function submit() {
    if (busy) return;
    busy = true;
    error = "";
    try {
      if (mode === "export") {
        if (password !== confirmation) throw new Error("Passphrases do not match");
        const contents = await encryptBackup(configuration(), password);
        const url = URL.createObjectURL(new Blob([contents], { type: "application/json" }));
        const anchor = document.createElement("a");
        anchor.href = url;
        anchor.download = `harbor-secrets-${new Date().toISOString().slice(0, 10)}.harbor`;
        anchor.click();
        window.setTimeout(() => URL.revokeObjectURL(url), 1000);
      } else {
        if (!file || !confirmed) throw new Error("Select a backup and confirm replacement");
        if (file.size > 8_388_608) throw new Error("Backup file is too large");
        const restored = await decryptBackup(await file.text(), password);
        if (!(await onRestore(restored)))
          throw new Error(
            "Unable to save the restored backup. Check the Secrets error before continuing.",
          );
      }
      open = false;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    } finally {
      busy = false;
    }
  }
</script>

<Dialog.Root bind:open onOpenChange={handleOpenChange}>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="flex max-h-[calc(100dvh-2rem)] w-full max-w-md flex-col gap-3.5 overflow-y-auto rounded-lg border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
    >
      <Dialog.Title class="text-lg font-bold text-boulder-950"
        >{mode === "export"
          ? t("secrets.backupExportTitle")
          : t("secrets.backupRestoreTitle")}</Dialog.Title
      >
      <Dialog.Description class="text-[13px] leading-relaxed text-boulder-600"
        >{mode === "export"
          ? t("secrets.backupExportDesc")
          : t("secrets.backupRestoreDesc")}</Dialog.Description
      >
      {#if mode === "restore"}
        <label class="flex flex-col gap-1.5 text-xs font-semibold text-boulder-700"
          >{t("secrets.backupFile")}<input
            class="w-full rounded-md border border-boulder-300 bg-white p-2 text-xs font-normal text-boulder-800 file:mr-3 file:rounded file:border-0 file:bg-boulder-100 file:px-2 file:py-1 file:text-xs file:font-semibold file:text-boulder-700"
            type="file"
            accept=".harbor,application/json"
            disabled={busy}
            onchange={(event) => (file = event.currentTarget.files?.[0] ?? null)}
          /></label
        >
      {/if}
      <label class="flex flex-col gap-1.5 text-xs font-semibold text-boulder-700"
        >{t("secrets.passphrase")}<input
          class="h-9 w-full rounded-md border border-boulder-300 px-2.5 text-[13px] font-normal text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-3 focus:ring-east-bay-500/15"
          type="password"
          autocomplete="off"
          bind:value={password}
          disabled={busy}
        /></label
      >
      {#if mode === "export"}<label
          class="flex flex-col gap-1.5 text-xs font-semibold text-boulder-700"
          >{t("secrets.confirmPassphrase")}<input
            class="h-9 w-full rounded-md border border-boulder-300 px-2.5 text-[13px] font-normal text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-3 focus:ring-east-bay-500/15"
            type="password"
            autocomplete="off"
            bind:value={confirmation}
            disabled={busy}
          /></label
        >{/if}
      {#if mode === "restore"}<label
          class="flex cursor-pointer items-center gap-2 text-xs text-boulder-700"
          ><input
            class="size-4 accent-red-600"
            type="checkbox"
            bind:checked={confirmed}
            disabled={busy}
          />{t("secrets.replaceConfirm")}</label
        >{/if}
      {#if error}<p
          class="rounded-md border border-red-200 bg-red-50 p-2 text-xs text-red-700"
          role="alert"
        >
          {error}
        </p>{/if}
      <div class="flex justify-end gap-2">
        <button
          class="h-9 cursor-pointer rounded-md border border-boulder-300 bg-white px-3 text-xs font-semibold text-boulder-700 hover:bg-boulder-100 disabled:cursor-not-allowed disabled:opacity-50"
          type="button"
          disabled={busy}
          onclick={() => (open = false)}>{t("secrets.cancel")}</button
        >
        <button
          class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-semibold text-white hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
          type="button"
          disabled={busy ||
            !password ||
            (mode === "export"
              ? password.length < 12 || password !== confirmation
              : !file || !confirmed)}
          onclick={() => void submit()}
          >{#if mode === "export"}<Download size={14} />{:else}<Upload size={14} />{/if}<span
            >{busy
              ? t("secrets.processing")
              : mode === "export"
                ? t("secrets.downloadBackup")
                : t("secrets.restoreBackupBtn")}</span
          ></button
        >
      </div>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
