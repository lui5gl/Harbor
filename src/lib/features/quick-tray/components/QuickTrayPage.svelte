<script lang="ts">
  import { t } from "#lib/i18n";
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { Anchor, Boxes, KeyRound, Maximize2, X } from "@lucide/svelte";
  import { onMount } from "svelte";
  import type { Secret } from "#lib/features/secrets/types";
  import { createSecretsStore } from "#lib/features/secrets/application/secretsStore.svelte";
  import { createQuickTrayServices } from "#lib/features/quick-tray/application/quickTrayServices.svelte";
  import QuickTrayConfirmationDialog from "#lib/features/quick-tray/components/QuickTrayConfirmationDialog.svelte";
  import QuickTrayEnvironmentDialog from "#lib/features/quick-tray/components/QuickTrayEnvironmentDialog.svelte";
  import QuickTraySecretsPanel from "#lib/features/quick-tray/components/QuickTraySecretsPanel.svelte";
  import QuickTrayServicesPanel from "#lib/features/quick-tray/components/QuickTrayServicesPanel.svelte";
  import QuickTrayVariableDialog from "#lib/features/quick-tray/components/QuickTrayVariableDialog.svelte";

  const isNativeApp = isTauri();
  const secrets = createSecretsStore();
  const services = createQuickTrayServices((message) => flashStatus(message));
  let currentTab = $state<"secrets" | "services">("secrets");
  let revealedSecretIds = $state<number[]>([]);
  let copiedId = $state<string | null>(null);
  let isActivating = $state(false);
  let statusMessage = $state("");
  let clipboardErrorMessage = $state("");
  let isVariableDialogOpen = $state(false);
  let editingSecretId = $state<number | null>(null);
  let formKey = $state("");
  let formValue = $state("");
  let formError = $state("");
  let isEnvironmentDialogOpen = $state(false);
  let newEnvironmentName = $state("");
  let isActivationDialogOpen = $state(false);
  let isDeleteVariableDialogOpen = $state(false);
  let pendingDeleteSecretId = $state<number | null>(null);

  onMount(() => {
    secrets.start();
    void secrets.load();
    services.start();
    void services.load();

    const handleFocus = () => {
      void secrets.load();
      void services.load();
    };
    window.addEventListener("focus", handleFocus);

    let unlisten: (() => void) | undefined;
    if (isNativeApp) {
      listen("secrets-updated", () => {
        void secrets.load();
      }).then((fn) => {
        unlisten = fn;
      });
    }

    return () => {
      window.removeEventListener("focus", handleFocus);
      if (unlisten) unlisten();
      secrets.dispose();
      services.dispose();
    };
  });

  function handleActivationRequest() {
    const environmentId = secrets.selectedEnvironmentId;
    if (environmentId === null) return;
    const environment = secrets.selectedEnvironment;
    if (!environment) return;
    if (environment.id !== secrets.activeEnvironmentId) {
      isActivationDialogOpen = true;
      return;
    }
    void executeActivation(environmentId);
  }

  async function executeActivation(environmentId: number) {
    isActivating = true;
    isActivationDialogOpen = false;
    const activated = await secrets.activateEnvironment(environmentId);
    if (activated) flashStatus("Entorno activado en el sistema");
    isActivating = false;
  }

  function flashStatus(message: string) {
    statusMessage = message;
    window.setTimeout(() => {
      if (statusMessage === message) statusMessage = "";
    }, 2500);
  }

  async function copyText(text: string, identifier: string) {
    try {
      await navigator.clipboard.writeText(text);
      clipboardErrorMessage = "";
      copiedId = identifier;
      window.setTimeout(() => {
        if (copiedId === identifier) copiedId = null;
      }, 1500);
    } catch (error) {
      clipboardErrorMessage = error instanceof Error ? error.message : String(error);
    }
  }

  function openAddVariableDialog() {
    formKey = "";
    formValue = "";
    formError = "";
    editingSecretId = null;
    isVariableDialogOpen = true;
  }

  function openEditVariableDialog(secret: Secret) {
    formKey = secret.key;
    formValue = secret.value;
    formError = "";
    editingSecretId = secret.id;
    isVariableDialogOpen = true;
  }

  async function saveVariable() {
    const key = formKey.trim();
    if (!key || !secrets.selectedEnvironment) {
      formError = "El nombre de la variable es obligatorio";
      return;
    }
    if (editingSecretId !== null) {
      secrets.updateVariable(editingSecretId, "key", key);
      secrets.updateVariable(editingSecretId, "value", formValue);
    } else {
      secrets.addCustomVariable(key, formValue);
    }
    isVariableDialogOpen = false;
  }

  function requestDeleteVariable(secretId: number) {
    pendingDeleteSecretId = secretId;
    isDeleteVariableDialogOpen = true;
  }

  async function confirmDeleteVariable() {
    if (pendingDeleteSecretId === null) return;
    const secretId = pendingDeleteSecretId;
    pendingDeleteSecretId = null;
    isDeleteVariableDialogOpen = false;
    secrets.deleteVariable(secretId);
  }

  function openCreateEnvironmentDialog() {
    newEnvironmentName = "";
    isEnvironmentDialogOpen = true;
  }

  async function saveNewEnvironment() {
    const name = newEnvironmentName.trim();
    if (!name) return;
    if (!secrets.prepareEnvironmentCreation(secrets.selectedProjectId)) return;
    secrets.createEnvironment(name);
    isEnvironmentDialogOpen = false;
  }

  async function openMainWindow() {
    if (isNativeApp) await invoke("show_main_window");
  }

  async function closeQuickTray() {
    if (isNativeApp) await invoke("hide_quick_tray");
  }
</script>

<div
  class="flex h-screen select-none flex-col overflow-hidden bg-boulder-50 font-sans text-[13px] text-boulder-950"
>
  <header
    class="flex h-12 shrink-0 cursor-default items-center justify-between border-b border-boulder-200 bg-white px-3.5"
    data-tauri-drag-region
  >
    <div class="flex items-center gap-2.5" data-tauri-drag-region>
      <span
        class="flex size-6.5 items-center justify-center rounded-md border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
        aria-hidden="true"
      >
        <Anchor size={14} strokeWidth={2.4} />
      </span>
      <div class="flex gap-0.5 rounded-md bg-boulder-100 p-0.5">
        <button
          type="button"
          class={`inline-flex h-6 cursor-pointer items-center gap-1 rounded px-2 text-[11.5px] font-semibold transition-all ${currentTab === "secrets" ? "bg-white text-east-bay-900 shadow-sm" : "text-boulder-600 hover:text-boulder-900"}`}
          onclick={() => (currentTab = "secrets")}
        >
          <KeyRound size={13} strokeWidth={2.2} />
          <span>{t("secrets.title")}</span>
        </button>
        <button
          type="button"
          class={`inline-flex h-6 cursor-pointer items-center gap-1 rounded px-2 text-[11.5px] font-semibold transition-all ${currentTab === "services" ? "bg-white text-east-bay-900 shadow-sm" : "text-boulder-600 hover:text-boulder-900"}`}
          onclick={() => (currentTab = "services")}
        >
          <Boxes size={13} strokeWidth={2.2} />
          <span>{t("services.titleShort")}</span>
        </button>
      </div>
    </div>

    <div class="flex items-center gap-1" data-tauri-drag-region="false">
      <button
        type="button"
        class="inline-flex size-7 cursor-pointer items-center justify-center rounded-md text-boulder-500 transition-colors hover:bg-boulder-100 hover:text-boulder-950"
        onclick={openMainWindow}
        title={t("secrets.quickTrayOpenFull")}
        aria-label={t("secrets.quickTrayOpenFull")}
      >
        <Maximize2 size={13} strokeWidth={2.2} />
      </button>
      <button
        type="button"
        class="inline-flex size-7 cursor-pointer items-center justify-center rounded-md text-boulder-500 transition-colors hover:bg-boulder-100 hover:text-boulder-950"
        onclick={closeQuickTray}
        title={t("secrets.quickTrayHide")}
        aria-label={t("secrets.quickTrayHide")}
      >
        <X size={14} strokeWidth={2.2} />
      </button>
    </div>
  </header>

  {#if secrets.isLoading}
    <div class="flex h-full flex-col items-center justify-center gap-2.5 text-boulder-500">
      <span
        class="size-5 animate-spin rounded-full border-2 border-boulder-200 border-t-east-bay-700"
      ></span>
      <span>Cargando Harbor...</span>
    </div>
  {:else}
    <div class="flex min-h-0 flex-1 flex-col bg-boulder-50">
      {#if statusMessage}
        <div
          class="bg-emerald-50 px-3.5 py-1.5 text-center text-[11.5px] font-medium text-emerald-800"
        >
          {statusMessage}
        </div>
      {/if}
      {#if secrets.error || services.error || clipboardErrorMessage}
        <div class="bg-red-50 px-3.5 py-1.5 text-center text-[11.5px] font-medium text-red-800">
          {secrets.error || services.error || clipboardErrorMessage}
        </div>
      {/if}

      {#if currentTab === "secrets"}
        <QuickTraySecretsPanel
          {secrets}
          {copiedId}
          {revealedSecretIds}
          {isActivating}
          onCreateEnvironment={openCreateEnvironmentDialog}
          onActivate={handleActivationRequest}
          onOpenAddVariable={openAddVariableDialog}
          onOpenEditVariable={openEditVariableDialog}
          onDeleteVariable={requestDeleteVariable}
          onCopy={copyText}
          onOpenMainWindow={openMainWindow}
        />
      {:else}
        <QuickTrayServicesPanel {services} onOpenMainWindow={openMainWindow} />
      {/if}
    </div>
  {/if}
</div>

<QuickTrayVariableDialog
  bind:open={isVariableDialogOpen}
  bind:formKey
  bind:formValue
  {editingSecretId}
  {formError}
  onSave={saveVariable}
/>
<QuickTrayEnvironmentDialog
  bind:open={isEnvironmentDialogOpen}
  bind:name={newEnvironmentName}
  onCreate={saveNewEnvironment}
/>
<QuickTrayConfirmationDialog
  bind:open={isActivationDialogOpen}
  kind="activation"
  onConfirm={() => {
    if (secrets.selectedEnvironment) void executeActivation(secrets.selectedEnvironment.id);
  }}
/>
<QuickTrayConfirmationDialog
  bind:open={isDeleteVariableDialogOpen}
  kind="delete"
  onConfirm={confirmDeleteVariable}
/>
