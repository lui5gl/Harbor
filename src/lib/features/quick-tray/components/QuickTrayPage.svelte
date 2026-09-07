<script lang="ts">
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import {
    Anchor,
    Boxes,
    Check,
    KeyRound,
    Maximize2,
    Plus,
    ShieldAlert,
    Trash2,
    X,
  } from "@lucide/svelte";
  import { AlertDialog, Button, Dialog, Switch, Tooltip } from "bits-ui";
  import { onMount } from "svelte";
  import type { Secret } from "$lib/features/secrets/types";
  import { createSecretsStore } from "$lib/features/secrets/application/secretsStore.svelte";
  import { createQuickTrayServices } from "$lib/features/quick-tray/application/quickTrayServices.svelte";
  import QuickTrayConfirmationDialog from "$lib/features/quick-tray/components/QuickTrayConfirmationDialog.svelte";
  import QuickTrayEnvironmentDialog from "$lib/features/quick-tray/components/QuickTrayEnvironmentDialog.svelte";
  import QuickTraySecretsPanel from "$lib/features/quick-tray/components/QuickTraySecretsPanel.svelte";
  import QuickTrayServicesPanel from "$lib/features/quick-tray/components/QuickTrayServicesPanel.svelte";
  import QuickTrayVariableDialog from "$lib/features/quick-tray/components/QuickTrayVariableDialog.svelte";

  const isNativeApp = isTauri();
  const secrets = createSecretsStore();
  const services = createQuickTrayServices((message) => flashStatus(message));
  let currentTab = $state<"secrets" | "services">("secrets");

  // Secrets state
  let revealedSecretIds = $state<number[]>([]);
  let copiedId = $state<string | null>(null);
  let isActivating = $state(false);
  let statusMessage = $state("");
  let clipboardErrorMessage = $state("");

  // Variable Dialog State
  let isVariableDialogOpen = $state(false);
  let editingSecretId = $state<number | null>(null);
  let formKey = $state("");
  let formValue = $state("");
  let formError = $state("");

  // Environment Dialog State
  let isEnvironmentDialogOpen = $state(false);
  let newEnvironmentName = $state("");

  // Confirmations
  let isProductionDialogOpen = $state(false);
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
      isProductionDialogOpen = true;
      return;
    }
    void executeActivation(environmentId);
  }

  async function executeActivation(environmentId: number) {
    isActivating = true;
    isProductionDialogOpen = false;
    const activated = await secrets.activateEnvironment(environmentId);
    if (activated) flashStatus("Entorno activado en el sistema");
    isActivating = false;
  }

  function flashStatus(msg: string) {
    statusMessage = msg;
    window.setTimeout(() => {
      if (statusMessage === msg) statusMessage = "";
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
    if (isNativeApp) {
      await invoke("show_main_window");
    }
  }

  async function closeQuickTray() {
    if (isNativeApp) {
      await invoke("hide_quick_tray");
    }
  }
</script>

<div class="tray-container">
  <!-- Top Navigation Header -->
  <header class="tray-header" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <span class="brand-badge" aria-hidden="true">
        <Anchor size={14} strokeWidth={2.4} />
      </span>
      <div class="header-tabs">
        <button
          type="button"
          class={`tab-btn${currentTab === "secrets" ? " active" : ""}`}
          onclick={() => (currentTab = "secrets")}
        >
          <KeyRound size={13} strokeWidth={2.2} />
          <span>Secrets</span>
        </button>
        <button
          type="button"
          class={`tab-btn${currentTab === "services" ? " active" : ""}`}
          onclick={() => (currentTab = "services")}
        >
          <Boxes size={13} strokeWidth={2.2} />
          <span>Services</span>
        </button>
      </div>
    </div>

    <div class="header-actions" data-tauri-drag-region="false">
      <Tooltip.Root>
        <Tooltip.Trigger
          class="header-action-btn"
          onclick={openMainWindow}
          aria-label="Abrir aplicación completa"
        >
          <Maximize2 size={13} strokeWidth={2.2} />
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content class="tooltip-content" sideOffset={6}>Abrir Harbor</Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>

      <Tooltip.Root>
        <Tooltip.Trigger
          class="header-action-btn"
          onclick={closeQuickTray}
          aria-label="Cerrar panel"
        >
          <X size={14} strokeWidth={2.2} />
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content class="tooltip-content" sideOffset={6}>Ocultar</Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    </div>
  </header>

  {#if secrets.isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span>Cargando Harbor...</span>
    </div>
  {:else}
    <!-- Main Content Area -->
    <div class="tray-body">
      <!-- Feedback notifications -->
      {#if statusMessage}
        <div class="feedback-banner success">{statusMessage}</div>
      {/if}
      {#if secrets.error || services.error || clipboardErrorMessage}
        <div class="feedback-banner error">
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
  bind:open={isProductionDialogOpen}
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

<style>
  :global {
    /* Base Container */
    .tray-container {
      background: var(--color-boulder-50);
      color: var(--color-boulder-950);
      display: flex;
      flex-direction: column;
      height: 100vh;
      overflow: hidden;
      user-select: none;
      box-sizing: border-box;
      font-family: var(--font-sans);
      font-size: 13px;
    }

    .tray-container :global(button) {
      appearance: none;
      -webkit-appearance: none;
      background: transparent;
      border: none;
      outline: none;
      font-family: inherit;
      cursor: pointer;
    }

    /* Header */
    .tray-header {
      align-items: center;
      background: #ffffff;
      border-bottom: 1px solid var(--color-boulder-200);
      display: flex;
      height: 48px;
      justify-content: space-between;
      padding: 0 14px;
      flex-shrink: 0;
      cursor: default;
    }

    .brand {
      align-items: center;
      display: flex;
      gap: 10px;
    }

    .brand-badge {
      align-items: center;
      background: var(--color-east-bay-50);
      border: 1px solid var(--color-east-bay-200);
      border-radius: 7px;
      color: var(--color-east-bay-700);
      display: flex;
      height: 26px;
      justify-content: center;
      width: 26px;
    }

    .header-tabs {
      background: var(--color-boulder-100);
      border-radius: 6px;
      display: flex;
      padding: 2px;
      gap: 2px;
    }

    .tab-btn {
      align-items: center;
      border-radius: 4px;
      color: var(--color-boulder-600);
      display: inline-flex;
      font-size: 11.5px;
      font-weight: 600;
      gap: 5px;
      height: 24px;
      padding: 0 8px;
      transition: all 0.15s ease;
    }

    .tab-btn.active {
      background: #ffffff;
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
      color: var(--color-east-bay-900);
    }

    .header-actions {
      align-items: center;
      display: flex;
      gap: 4px;
    }

    :global(.header-action-btn) {
      align-items: center;
      background: transparent;
      border: none !important;
      border-radius: 6px;
      color: var(--color-boulder-500);
      cursor: pointer;
      display: inline-flex;
      height: 28px;
      justify-content: center;
      padding: 0;
      width: 28px;
      transition:
        background 0.15s ease,
        color 0.15s ease;
    }

    :global(.header-action-btn:hover) {
      background: var(--color-boulder-100);
      color: var(--color-boulder-950);
    }

    /* Body */
    .tray-body {
      display: flex;
      flex-direction: column;
      flex: 1;
      min-height: 0;
      background: var(--color-boulder-50);
    }

    /* Profile Toolbar (Select & Activation) */
    .profile-toolbar {
      background: #ffffff;
      border-bottom: 1px solid var(--color-boulder-200);
      padding: 10px 14px;
      display: flex;
      flex-direction: column;
      gap: 8px;
      flex-shrink: 0;
    }

    .profile-select-group {
      display: flex;
      align-items: center;
      gap: 8px;
      width: 100%;
    }

    :global(.profile-select-trigger) {
      align-items: center;
      background: #ffffff;
      border: 1px solid var(--color-boulder-300) !important;
      border-radius: 6px;
      color: var(--color-boulder-900);
      cursor: pointer;
      display: flex;
      flex: 1;
      height: 32px;
      justify-content: space-between;
      padding: 0 10px;
      transition: all 0.15s ease;
    }

    :global(.profile-select-trigger:hover) {
      background: var(--color-boulder-50);
      border-color: var(--color-boulder-400) !important;
    }

    .select-label-wrapper {
      align-items: center;
      display: flex;
      gap: 7px;
      overflow: hidden;
    }

    .select-profile-name {
      font-size: 12.5px;
      font-weight: 650;
      color: var(--color-boulder-950);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    :global(.select-chevron) {
      color: var(--color-boulder-400);
      flex-shrink: 0;
    }

    .active-dot {
      background: #10b981;
      border-radius: 50%;
      box-shadow: 0 0 4px #10b981;
      height: 7px;
      width: 7px;
      flex-shrink: 0;
    }

    .dot-placeholder {
      height: 7px;
      width: 7px;
      flex-shrink: 0;
    }

    :global(.new-profile-btn) {
      font-size: 12px;
      font-weight: 600;
      height: 32px;
      padding: 0 10px;
      white-space: nowrap;
      flex-shrink: 0;
    }

    /* Profile Select Dropdown Menu */
    :global(.profile-select-content) {
      background: #ffffff;
      border: 1px solid var(--color-boulder-200);
      border-radius: 8px;
      box-shadow: 0 12px 32px rgb(11 11 11 / 14%);
      min-width: 220px;
      overflow: hidden;
      padding: 4px;
      z-index: 100;
    }

    :global(.profile-select-viewport) {
      display: flex;
      flex-direction: column;
      gap: 2px;
    }

    :global(.profile-select-item) {
      align-items: center;
      border-radius: 5px;
      color: var(--color-boulder-800);
      cursor: pointer;
      display: flex;
      font-size: 12.5px;
      font-weight: 550;
      justify-content: space-between;
      height: 32px;
      outline: none;
      padding: 0 10px;
      transition: background 0.1s ease;
    }

    :global(.profile-select-item[data-highlighted]) {
      background: var(--color-east-bay-50);
      color: var(--color-east-bay-900);
    }

    .item-left {
      align-items: center;
      display: flex;
      gap: 7px;
    }

    .item-name {
      font-weight: 600;
    }

    :global(.item-check) {
      color: var(--color-east-bay-700);
    }

    /* Activation Row */
    .activation-row {
      align-items: center;
      display: flex;
      justify-content: space-between;
    }

    .active-status-tag {
      align-items: center;
      background: #ecfdf5;
      border: 1px solid #a7f3d0;
      border-radius: 6px;
      color: #047857;
      display: flex;
      font-size: 11.5px;
      font-weight: 600;
      gap: 6px;
      padding: 4px 10px;
      width: 100%;
    }

    .active-pulse {
      background: #10b981;
      border-radius: 50%;
      height: 6px;
      width: 6px;
    }

    .inactive-status-row {
      align-items: center;
      display: flex;
      justify-content: space-between;
      width: 100%;
    }

    .inactive-label {
      color: var(--color-boulder-500);
      font-size: 12px;
    }

    :global(.activate-action-btn) {
      align-items: center;
      background: var(--color-east-bay-700);
      border: none !important;
      border-radius: 5px;
      color: #ffffff;
      display: inline-flex;
      gap: 5px;
      font-size: 11.5px;
      font-weight: 650;
      height: 26px;
      padding: 0 10px;
      transition: background 0.15s ease;
    }

    :global(.activate-action-btn:hover) {
      background: var(--color-east-bay-900);
    }

    /* Search & Action Toolbar */
    .search-section {
      align-items: center;
      background: #ffffff;
      border-bottom: 1px solid var(--color-boulder-200);
      box-sizing: border-box;
      display: flex;
      gap: 8px;
      padding: 8px 14px;
      flex-shrink: 0;
    }

    .search-input-wrapper {
      align-items: center;
      background: var(--color-boulder-100);
      border: 1px solid transparent;
      border-radius: 6px;
      box-sizing: border-box;
      display: flex;
      flex: 1;
      height: 30px;
      padding: 0 8px;
      transition: all 0.15s ease;
    }

    .search-input-wrapper:focus-within {
      background: #ffffff;
      border-color: var(--color-east-bay-400);
      box-shadow: 0 0 0 2px rgb(113 132 192 / 14%);
    }

    :global(.search-icon) {
      color: var(--color-boulder-400);
      flex-shrink: 0;
      margin-right: 6px;
    }

    .clear-search-btn {
      align-items: center;
      color: var(--color-boulder-400);
      display: inline-flex;
      height: 16px;
      justify-content: center;
      width: 16px;
    }

    :global(.add-var-btn) {
      align-items: center;
      background: var(--color-east-bay-50);
      border: 1px solid var(--color-east-bay-200) !important;
      border-radius: 6px;
      color: var(--color-east-bay-800);
      display: inline-flex;
      font-size: 11.5px;
      font-weight: 600;
      gap: 4px;
      height: 30px;
      padding: 0 10px;
      white-space: nowrap;
    }

    :global(.add-var-btn:hover) {
      background: var(--color-east-bay-100);
    }

    /* Feedback */
    .feedback-banner {
      font-size: 11.5px;
      font-weight: 550;
      padding: 6px 14px;
      text-align: center;
    }

    .feedback-banner.success {
      background: #ecfdf5;
      color: #065f46;
    }

    .feedback-banner.error {
      background: #fef2f2;
      color: #991b1b;
    }

    /* Secrets List Area */
    .secrets-container {
      display: flex;
      flex: 1;
      flex-direction: column;
      min-height: 0;
      overflow: hidden;
    }

    :global(.variables-scroll-area) {
      flex: 1;
      min-height: 0;
      height: 100%;
    }

    :global(.variables-viewport) {
      display: flex;
      flex-direction: column;
      gap: 6px;
      padding: 10px 14px;
      box-sizing: border-box;
    }

    .secret-card {
      align-items: center;
      background: #ffffff;
      border: 1px solid var(--color-boulder-200);
      border-radius: 6px;
      display: flex;
      justify-content: space-between;
      padding: 8px 10px;
      transition: border-color 0.15s ease;
    }

    .secret-card:hover {
      border-color: var(--color-boulder-300);
    }

    .secret-info {
      display: flex;
      flex-direction: column;
      gap: 2px;
      min-width: 0;
      flex: 1;
    }

    .secret-key-line {
      align-items: center;
      display: flex;
      gap: 5px;
    }

    .secret-key-text {
      color: var(--color-boulder-950);
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      font-size: 12px;
      font-weight: 700;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .copy-key-btn {
      align-items: center;
      color: var(--color-boulder-400);
      display: inline-flex;
      height: 16px;
      justify-content: center;
      width: 16px;
      border-radius: 3px;
    }

    .copy-key-btn:hover {
      color: var(--color-boulder-700);
      background: var(--color-boulder-100);
    }

    .copy-key-btn.copied {
      color: #10b981;
    }

    .secret-val-line {
      min-width: 0;
    }

    .secret-val-text {
      color: var(--color-boulder-600);
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
      font-size: 11px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      display: block;
    }

    .secret-val-masked {
      color: var(--color-boulder-400);
      font-size: 10px;
      letter-spacing: 2px;
    }

    .secret-card-actions {
      align-items: center;
      display: flex;
      gap: 2px;
      margin-left: 8px;
      flex-shrink: 0;
    }

    :global(.action-icon-btn) {
      align-items: center;
      border-radius: 4px;
      color: var(--color-boulder-400);
      display: inline-flex;
      height: 24px;
      justify-content: center;
      width: 24px;
      transition: all 0.1s ease;
    }

    :global(.action-icon-btn:hover) {
      background: var(--color-boulder-100);
      color: var(--color-boulder-800);
    }

    :global(.action-icon-btn.copied) {
      color: #10b981;
    }

    /* Services Tab Views */
    .services-tray-view {
      display: flex;
      flex-direction: column;
      flex: 1;
      min-height: 0;
    }

    .services-tray-list {
      display: flex;
      flex-direction: column;
      gap: 10px;
      padding: 14px;
      flex: 1;
      overflow-y: auto;
    }

    .service-tray-card {
      background: #ffffff;
      border: 1px solid var(--color-boulder-200);
      border-radius: 8px;
      padding: 12px 14px;
      display: flex;
      flex-direction: column;
      gap: 10px;
    }

    .service-tray-header {
      align-items: center;
      display: flex;
      justify-content: space-between;
    }

    .service-tray-title {
      align-items: center;
      display: flex;
      gap: 10px;
    }

    .service-icon-badge {
      align-items: center;
      border-radius: 6px;
      display: inline-flex;
      font-size: 10px;
      font-weight: 800;
      height: 28px;
      justify-content: center;
      width: 32px;
    }

    .php-badge {
      background: #ede9fe;
      color: #6d28d9;
    }

    .node-badge {
      background: #ecfdf5;
      color: #047857;
    }

    .apache-badge {
      background: #fef3c7;
      color: #b45309;
    }

    .service-name {
      font-size: 13.5px;
      font-weight: 700;
      margin: 0;
      color: var(--color-boulder-950);
    }

    .service-subtitle {
      font-size: 11px;
      color: var(--color-boulder-500);
      margin: 0;
    }

    :global(.service-toggle-btn) {
      align-items: center;
      background: var(--color-east-bay-900);
      border-radius: 5px;
      color: #ffffff;
      display: inline-flex;
      font-size: 11.5px;
      font-weight: 600;
      gap: 5px;
      height: 28px;
      padding: 0 10px;
    }

    :global(.service-toggle-btn.running) {
      background: #e11d48;
    }

    .service-tray-body {
      display: flex;
      flex-direction: column;
      gap: 8px;
      padding-top: 4px;
      border-top: 1px solid var(--color-boulder-100);
    }

    .service-field {
      align-items: center;
      display: flex;
      justify-content: space-between;
      font-size: 12px;
    }

    .field-desc {
      color: var(--color-boulder-600);
    }

    .service-select {
      background: var(--color-boulder-50);
      border: 1px solid var(--color-boulder-300);
      border-radius: 5px;
      color: var(--color-boulder-900);
      font-family: inherit;
      font-size: 12px;
      font-weight: 600;
      height: 26px;
      padding: 0 6px;
      outline: none;
    }

    .service-select:focus {
      border-color: var(--color-east-bay-500);
    }

    .service-static-ver {
      font-size: 12px;
      font-weight: 600;
      color: var(--color-boulder-800);
    }

    .not-installed-label {
      color: var(--color-boulder-400);
      font-size: 11.5px;
      font-style: italic;
    }

    .service-status-row {
      align-items: center;
      display: flex;
    }

    .status-pill {
      align-items: center;
      border-radius: 12px;
      display: inline-flex;
      font-size: 10.5px;
      font-weight: 600;
      gap: 5px;
      padding: 2px 8px;
    }

    .status-pill.running {
      background: #ecfdf5;
      color: #047857;
    }

    .status-pill.stopped {
      background: var(--color-boulder-100);
      color: var(--color-boulder-600);
    }

    .status-pill.ready {
      background: var(--color-east-bay-50);
      color: var(--color-east-bay-800);
    }

    .status-dot-indicator {
      border-radius: 50%;
      height: 6px;
      width: 6px;
      background: currentColor;
    }

    /* Empty state */
    .empty-state {
      align-items: center;
      display: flex;
      flex-direction: column;
      justify-content: center;
      padding: 36px 20px;
      text-align: center;
      margin: auto 0;
    }

    .empty-icon {
      align-items: center;
      background: var(--color-boulder-100);
      border-radius: 50%;
      color: var(--color-boulder-500);
      display: flex;
      height: 48px;
      justify-content: center;
      margin-bottom: 12px;
      width: 48px;
    }

    .empty-title {
      color: var(--color-boulder-950);
      font-size: 13.5px;
      font-weight: 650;
      margin: 0 0 4px;
    }

    .empty-subtitle {
      color: var(--color-boulder-500);
      font-size: 12px;
      line-height: 1.4;
      margin: 0 0 16px;
      max-width: 240px;
    }

    /* Footer */
    .tray-footer {
      align-items: center;
      background: #ffffff;
      border-top: 1px solid var(--color-boulder-200);
      display: flex;
      height: 38px;
      justify-content: space-between;
      padding: 0 14px;
      flex-shrink: 0;
    }

    .footer-status {
      align-items: center;
      color: var(--color-boulder-500);
      display: flex;
      font-size: 11.5px;
      gap: 6px;
    }

    .footer-indicator {
      background: var(--color-boulder-400);
      border-radius: 50%;
      height: 5px;
      width: 5px;
    }

    :global(.footer-open-btn) {
      align-items: center;
      color: var(--color-east-bay-700);
      display: inline-flex;
      font-size: 11.5px;
      font-weight: 600;
      gap: 5px;
    }

    :global(.footer-open-btn:hover) {
      color: var(--color-east-bay-950);
    }

    /* Buttons & Inputs */
    :global(.secondary-button) {
      align-items: center;
      background: #ffffff;
      border: 1px solid var(--color-boulder-300) !important;
      border-radius: 6px;
      color: var(--color-boulder-800);
      display: inline-flex;
      font-size: 12px;
      font-weight: 600;
      gap: 6px;
      height: 32px;
      justify-content: center;
      padding: 0 12px;
      transition: all 0.15s ease;
    }

    :global(.secondary-button:hover) {
      background: var(--color-boulder-100);
    }

    :global(.primary-button) {
      align-items: center;
      background: var(--color-east-bay-900);
      border: 1px solid transparent !important;
      border-radius: 6px;
      color: #ffffff;
      display: inline-flex;
      font-size: 12px;
      font-weight: 600;
      gap: 6px;
      height: 32px;
      justify-content: center;
      padding: 0 12px;
    }

    :global(.primary-button:hover) {
      background: var(--color-east-bay-950);
    }

    :global(.danger-button) {
      align-items: center;
      background: #e11d48;
      border-radius: 6px;
      color: #ffffff;
      display: inline-flex;
      font-size: 12px;
      font-weight: 600;
      height: 32px;
      padding: 0 12px;
    }

    :global(.btn-sm) {
      height: 30px;
      font-size: 11.5px;
      padding: 0 10px;
    }

    /* Modals and Dialogs */
    :global(.modal-backdrop) {
      background: rgba(0, 0, 0, 0.4);
      backdrop-filter: blur(2px);
      inset: 0;
      position: fixed;
      z-index: 200;
    }

    :global(.dialog-content) {
      background: #ffffff;
      border: 1px solid var(--color-boulder-200);
      border-radius: 10px;
      box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
      box-sizing: border-box;
      display: flex;
      flex-direction: column;
      left: 50%;
      max-width: 340px;
      padding: 18px;
      position: fixed;
      top: 50%;
      transform: translate(-50%, -50%);
      width: 90%;
      z-index: 201;
    }

    :global(.dialog-title) {
      color: var(--color-boulder-950);
      font-size: 15px;
      font-weight: 700;
      margin: 0 0 4px;
    }

    :global(.dialog-description) {
      color: var(--color-boulder-600);
      font-size: 12px;
      margin: 0 0 14px;
    }

    .dialog-form-fields {
      display: flex;
      flex-direction: column;
      gap: 12px;
      margin-bottom: 16px;
    }

    .field-group {
      display: flex;
      flex-direction: column;
      gap: 5px;
    }

    .field-label {
      color: var(--color-boulder-700);
      font-size: 11px;
      font-weight: 600;
    }

    .dialog-text-input {
      background: #ffffff;
      border: 1px solid var(--color-boulder-300);
      border-radius: 6px;
      box-sizing: border-box;
      color: var(--color-boulder-950);
      font-family: inherit;
      font-size: 12.5px;
      height: 32px;
      outline: none;
      padding: 0 10px;
      width: 100%;
    }

    .dialog-text-input:focus {
      border-color: var(--color-east-bay-500);
    }

    .dialog-text-input.font-mono {
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    }

    .form-error-box {
      background: #fef2f2;
      border: 1px solid #fecdd3;
      border-radius: 6px;
      color: #991b1b;
      font-size: 11.5px;
      margin-bottom: 12px;
      padding: 6px 10px;
    }

    .dialog-footer {
      display: flex;
      gap: 8px;
      justify-content: flex-end;
    }

    /* Confirmation Dialog */
    .confirmation-dialog {
      text-align: center;
      align-items: center;
    }

    .warning-icon-wrapper {
      align-items: center;
      background: #fffbeb;
      border-radius: 50%;
      color: #d97706;
      display: flex;
      height: 44px;
      justify-content: center;
      margin-bottom: 10px;
      width: 44px;
    }

    .danger-icon-wrapper {
      align-items: center;
      background: #fef2f2;
      border-radius: 50%;
      color: #e11d48;
      display: flex;
      height: 44px;
      justify-content: center;
      margin-bottom: 10px;
      width: 44px;
    }

    /* Dropdown Menu */
    :global(.dropdown-content) {
      background: #ffffff;
      border: 1px solid var(--color-boulder-200);
      border-radius: 6px;
      box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
      padding: 4px;
      z-index: 150;
    }

    :global(.dropdown-item) {
      align-items: center;
      border-radius: 4px;
      color: var(--color-boulder-800);
      cursor: pointer;
      display: flex;
      font-size: 12px;
      gap: 8px;
      height: 28px;
      outline: none;
      padding: 0 8px;
    }

    :global(.dropdown-item[data-highlighted]) {
      background: var(--color-east-bay-50);
      color: var(--color-east-bay-900);
    }

    :global(.dropdown-item.destructive) {
      color: #e11d48;
    }

    :global(.dropdown-item.destructive[data-highlighted]) {
      background: #fff1f2;
      color: #be123c;
    }

    :global(.dropdown-separator) {
      background: var(--color-boulder-200);
      height: 1px;
      margin: 4px 0;
    }

    /* Tooltip */
    :global(.tooltip-content) {
      background: var(--color-boulder-900);
      border-radius: 4px;
      color: #ffffff;
      font-size: 11px;
      padding: 3px 6px;
      z-index: 300;
    }

    /* Spinner */
    .loading-state {
      align-items: center;
      color: var(--color-boulder-500);
      display: flex;
      flex-direction: column;
      gap: 10px;
      justify-content: center;
      height: 100%;
    }

    .spinner {
      border: 2px solid var(--color-boulder-200);
      border-top-color: var(--color-east-bay-700);
      border-radius: 50%;
      height: 20px;
      width: 20px;
      animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
      to {
        transform: rotate(360deg);
      }
    }
  }
</style>
