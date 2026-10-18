<script lang="ts">
  import {
    ChevronDown,
    Download,
    Loader2,
    Lock,
    Plus,
    Power,
    RefreshCw,
    Save,
    ShieldCheck,
    Unlock,
    Upload,
  } from "@lucide/svelte";
  import { Button, DropdownMenu, Tooltip } from "bits-ui";
  import { onMount } from "svelte";
  import DeleteConfirmationDialog from "./DeleteConfirmationDialog.svelte";
  import EnvironmentDialog from "./EnvironmentDialog.svelte";
  import ProductionActivationDialog from "./ProductionActivationDialog.svelte";
  import ProfileEditor from "./ProfileEditor.svelte";
  import ProfilesPanel from "./ProfilesPanel.svelte";
  import ProjectDialog from "./ProjectDialog.svelte";
  import SecretsBackupDialog from "./SecretsBackupDialog.svelte";
  import { createSecretsStore } from "../application/secretsStore.svelte";
  import { t } from "#lib/i18n";

  const secrets = createSecretsStore();

  let projectDialogOpen = $state(false);
  let environmentDialogOpen = $state(false);
  let productionDialogOpen = $state(false);
  let deleteDialogOpen = $state(false);
  let deleteMode = $state<"project" | "environment" | "variable" | null>(null);
  let pendingVariableId = $state<number | null>(null);
  let pendingActivationId = $state<number | null>(null);
  let pendingActivationLabel = $state("");
  let backupOpen = $state(false);
  let backupMode = $state<"export" | "restore">("export");

  async function openBackup(mode: "export" | "restore") {
    if (!(await secrets.flush())) return;
    backupMode = mode;
    backupOpen = true;
  }

  let targetActivationLabel = $derived(
    secrets.selectedProject && secrets.selectedEnvironment
      ? `${secrets.selectedProject.name} / ${secrets.selectedEnvironment.name}`
      : "Selected environment",
  );
  let deleteTitle = $derived(
    deleteMode === "project"
      ? "Delete this project?"
      : deleteMode === "environment"
        ? "Delete this environment?"
        : "Delete this variable?",
  );
  let deleteDescription = $derived(
    deleteMode === "project"
      ? `This permanently removes "${secrets.selectedProject?.name ?? "this project"}" and all of its environments.`
      : deleteMode === "environment"
        ? `This permanently removes "${secrets.selectedEnvironment?.name ?? "this environment"}" and all its variables.`
        : "This removes the variable from the selected environment.",
  );

  let activeEnvironmentInfo = $derived.by(() => {
    if (secrets.activeEnvironmentId === null) return null;
    for (const project of secrets.projects) {
      for (const env of project.environments) {
        if (env.id === secrets.activeEnvironmentId) {
          return {
            projectName: project.name,
            environmentName: env.name,
            isProduction: env.isProduction,
          };
        }
      }
    }
    return null;
  });

  onMount(() => {
    secrets.start();
    void secrets.load();
    return () => secrets.dispose();
  });

  function openEnvironmentDialog(projectId = secrets.selectedProjectId) {
    if (secrets.prepareEnvironmentCreation(projectId)) environmentDialogOpen = true;
  }

  function requestDelete(mode: "project" | "environment" | "variable", variableId?: number) {
    deleteMode = mode;
    pendingVariableId = variableId ?? null;
    deleteDialogOpen = true;
  }

  function confirmDelete() {
    if (deleteMode === "project") secrets.deleteProject();
    if (deleteMode === "environment") secrets.deleteEnvironment();
    if (deleteMode === "variable" && pendingVariableId !== null)
      secrets.deleteVariable(pendingVariableId);
    deleteMode = null;
    pendingVariableId = null;
    deleteDialogOpen = false;
  }

  async function requestActivation() {
    const environment = secrets.selectedEnvironment;
    if (!environment) return;
    pendingActivationId = environment.id;
    pendingActivationLabel = `${secrets.selectedProject?.name ?? "Project"} / ${environment.name}`;
    productionDialogOpen = true;
  }

  function confirmProductionActivation() {
    const id = pendingActivationId;
    pendingActivationId = null;
    pendingActivationLabel = "";
    productionDialogOpen = false;
    if (id !== null) void secrets.activateEnvironment(id);
  }
</script>

<svelte:head>
  <title>Harbor | {t("secrets.title")}</title>
  <meta name="description" content={t("secrets.description")} />
</svelte:head>

<main class="secrets-page" aria-labelledby="secrets-title">
  <header class="page-header">
    <div class="header-left">
      <div class="top-row">
        <p class="eyebrow">{t("secrets.eyebrow")}</p>
        {#if !secrets.isLoading}
          <span class="save-status">
            {#if secrets.isSaving}
              <Loader2 size={12} class="spin" />
              <span>{t("secrets.saving")}</span>
            {:else if secrets.hasUnsavedChanges}
              <Save size={12} class="unsaved-icon" />
              <span>{t("secrets.unsavedChanges")}</span>
            {:else if !secrets.error && secrets.canWrite}
              <ShieldCheck size={13} class="shield-icon" />
              <span>{t("secrets.vaultStatus")}</span>
            {/if}
          </span>
        {/if}
      </div>
      <h1 id="secrets-title">{t("secrets.title")}</h1>
      <p class="page-description">{t("secrets.description")}</p>
    </div>

    <div class="header-right">
      <DropdownMenu.Root>
        <DropdownMenu.Trigger
          class="secondary-header-btn"
          type="button"
          disabled={!secrets.canWrite || secrets.isSaving}
          aria-label={t("secrets.encryptedBackup")}
        >
          <Download size={13.5} />
          <span>{t("secrets.encryptedBackup")}</span>
          <ChevronDown size={12} />
        </DropdownMenu.Trigger>
        <DropdownMenu.Portal>
          <DropdownMenu.Content class="actions-menu" sideOffset={6} align="end">
            <DropdownMenu.Item class="menu-item" onclick={() => void openBackup("export")}>
              <Download size={13.5} />
              <span>{t("secrets.exportBackupItem")}</span>
            </DropdownMenu.Item>
            <DropdownMenu.Item
              class="menu-item"
              disabled={secrets.activeEnvironmentId !== null}
              onclick={() => void openBackup("restore")}
            >
              <Upload size={13.5} />
              <span>{t("secrets.restoreBackup")}</span>
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Portal>
      </DropdownMenu.Root>

      <Tooltip.Root>
        <Tooltip.Trigger
          class="lock-vault-btn"
          type="button"
          disabled={secrets.isLocked || !secrets.canWrite}
          onclick={() => secrets.lock()}
          aria-label={t("secrets.lockVault")}
        >
          <Lock size={13.5} />
          <span>{t("secrets.lockVault")}</span>
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content class="tooltip-content" sideOffset={6}>
            {t("secrets.vaultLockedDesc")}
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>

      <Button.Root
        class="new-project-button"
        type="button"
        disabled={!secrets.canWrite}
        onclick={() => (projectDialogOpen = true)}
      >
        <Plus size={15} strokeWidth={2.4} />
        <span>{t("secrets.newProject")}</span>
      </Button.Root>
    </div>
  </header>

  {#if secrets.error}
    <div class="error" role="alert">
      <p>{secrets.error}</p>
      <div class="error-actions">
        <Button.Root
          type="button"
          class="error-btn"
          disabled={secrets.isBusy || secrets.isSaving}
          onclick={() => void secrets.reload()}
        >
          <RefreshCw size={13} />
          <span>Recargar</span>
        </Button.Root>
        {#if secrets.hasUnsavedChanges}
          <Button.Root
            type="button"
            class="error-btn primary-error-btn"
            disabled={secrets.isBusy || secrets.isSaving}
            onclick={() => void secrets.flush()}
          >
            <Save size={13} />
            <span>Reintentar</span>
          </Button.Root>
        {/if}
      </div>
    </div>
  {/if}

  {#if activeEnvironmentInfo}
    <div class="publication-status" role="region" aria-label="Perfil activo en Windows">
      <div class="pub-info">
        <span class="pub-pulse-dot" aria-hidden="true"></span>
        <div class="pub-text">
          <span class="pub-title">
            {t("secrets.activeInPowerShell")}:
            <strong
              >{activeEnvironmentInfo.projectName} / {activeEnvironmentInfo.environmentName}</strong
            >
            {#if activeEnvironmentInfo.isProduction}
              <span class="pub-prod-badge">PROD</span>
            {/if}
          </span>
          <span class="pub-desc">{t("secrets.publishedToWindows")}</span>
        </div>
      </div>

      <Button.Root
        type="button"
        class="deactivate-btn"
        disabled={secrets.isBusy || secrets.isSaving}
        onclick={() => void secrets.deactivateEnvironment()}
      >
        <Power size={13} strokeWidth={2.2} />
        <span>{t("secrets.deactivateFromWindows")}</span>
      </Button.Root>
    </div>
  {/if}

  {#if secrets.isLoading}
    <div class="loading" role="status">
      <Loader2 size={24} class="spin" />
      <span>{t("secrets.loading")}</span>
    </div>
  {:else if secrets.isLocked}
    <div class="locked-workspace">
      <div class="lock-icon-box">
        <Lock size={30} strokeWidth={2.2} />
      </div>
      <h2>{t("secrets.vaultLockedTitle")}</h2>
      <p>{t("secrets.vaultLockedDesc")}</p>
      <Button.Root
        class="primary-button unlock-btn"
        type="button"
        onclick={() => void secrets.unlock()}
      >
        <Unlock size={15} strokeWidth={2.2} />
        <span>{t("secrets.unlockVault")}</span>
      </Button.Root>
    </div>
  {:else if secrets.projects.length === 0}
    <div class="empty-workspace">
      <h2>{t("secrets.noProjects")}</h2>
      <p>{t("secrets.noProjectsDescription")}</p>
      <Button.Root
        class="new-project-button"
        type="button"
        disabled={!secrets.canWrite}
        onclick={() => (projectDialogOpen = true)}
      >
        <Plus size={16} strokeWidth={2.4} />
        <span>{t("secrets.createProject")}</span>
      </Button.Root>
    </div>
  {:else}
    <div class="workspace">
      <ProfilesPanel
        projects={secrets.projects}
        selectedProjectId={secrets.selectedProjectId}
        selectedEnvironmentId={secrets.selectedEnvironmentId}
        activeEnvironmentId={secrets.activeEnvironmentId}
        onSelectEnvironment={secrets.selectEnvironment}
        onAddEnvironment={openEnvironmentDialog}
      />
      {#if secrets.selectedProject && secrets.selectedEnvironment}
        <ProfileEditor
          project={secrets.selectedProject}
          environment={secrets.selectedEnvironment}
          activeEnvironmentId={secrets.activeEnvironmentId}
          disabled={!secrets.canWrite ||
            secrets.selectedEnvironment.id === secrets.activeEnvironmentId}
          busy={secrets.isBusy || secrets.isSaving || !secrets.canWrite}
          onSelectEnvironment={(id) => secrets.selectEnvironment(secrets.selectedProject!.id, id)}
          onRenameProject={secrets.renameProject}
          onRequestDeleteProject={() => requestDelete("project")}
          onSaveEnvironment={secrets.saveEnvironment}
          onRequestDeleteEnvironment={() => requestDelete("environment")}
          onCloneEnvironment={secrets.cloneEnvironment}
          onAddVariable={secrets.addVariable}
          onAddCustomVariable={secrets.addCustomVariable}
          onImportVariables={secrets.importVariables}
          onUpdateVariable={secrets.updateVariable}
          onRequestDeleteVariable={(id) => requestDelete("variable", id)}
          onActivateEnvironment={() => void requestActivation()}
          onOpenTerminal={() => {
            if (!secrets.selectedEnvironment) return;
            if (
              !secrets.selectedEnvironment.isProduction ||
              window.confirm(
                `Open an isolated terminal with production secrets for ${targetActivationLabel}?`,
              )
            )
              void secrets.openTerminal(secrets.selectedEnvironment.id);
          }}
        />
      {/if}
    </div>
  {/if}
</main>

<SecretsBackupDialog
  bind:open={backupOpen}
  mode={backupMode}
  configuration={secrets.exportConfiguration}
  onRestore={secrets.restoreConfiguration}
/>
<ProjectDialog bind:open={projectDialogOpen} onCreate={secrets.createProject} />
<EnvironmentDialog
  bind:open={environmentDialogOpen}
  projectName={secrets.selectedProject?.name ?? "Project"}
  onCreate={secrets.createEnvironment}
/>
<ProductionActivationDialog
  bind:open={productionDialogOpen}
  environmentLabel={pendingActivationLabel || targetActivationLabel}
  onOpenChange={(open) => {
    if (!open) {
      pendingActivationId = null;
      pendingActivationLabel = "";
    }
  }}
  onConfirm={confirmProductionActivation}
/>
<DeleteConfirmationDialog
  bind:open={deleteDialogOpen}
  title={deleteTitle}
  description={deleteDescription}
  actionLabel={deleteMode === "project"
    ? "Delete project"
    : deleteMode === "environment"
      ? "Delete environment"
      : "Delete variable"}
  confirmKeyword={deleteMode === "project" || secrets.selectedEnvironment?.isProduction
    ? "DELETE"
    : undefined}
  onOpenChange={(open) => {
    if (!open) deleteMode = null;
  }}
  onConfirm={confirmDelete}
/>

<style>
  .secrets-page {
    box-sizing: border-box;
    display: flex;
    flex: 1;
    flex-direction: column;
    margin: 0 auto;
    max-width: 1240px;
    padding: 24px 32px 32px;
    width: 100%;
  }
  .page-header {
    align-items: center;
    display: flex;
    justify-content: space-between;
    gap: 20px;
    flex-wrap: wrap;
  }
  .header-left {
    display: flex;
    flex-direction: column;
  }
  .header-right {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }
  .top-row {
    align-items: center;
    display: flex;
    gap: 12px;
    margin-bottom: 6px;
  }
  .eyebrow {
    color: var(--color-east-bay-700);
    font-size: 11px;
    font-weight: 750;
    letter-spacing: 0.06em;
    margin: 0;
    text-transform: uppercase;
  }
  .save-status {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 99px;
    color: var(--color-boulder-700);
    display: inline-flex;
    font-size: 11px;
    font-weight: 600;
    gap: 5px;
    padding: 2.5px 8px;
    box-shadow: 0 1px 2px rgb(0 0 0 / 4%);
  }
  :global(.shield-icon) {
    color: #16a34a;
  }
  :global(.unsaved-icon) {
    color: #d97706;
  }
  .save-status :global(.spin) {
    animation: spin 1s linear infinite;
    color: var(--color-east-bay-700);
  }
  h1 {
    color: var(--color-boulder-950);
    font-size: 26px;
    font-weight: 750;
    letter-spacing: -0.02em;
    line-height: 1.1;
    margin: 0;
  }
  .page-description {
    color: var(--color-boulder-600);
    font-size: 12.5px;
    line-height: 1.45;
    margin: 4px 0 0;
    max-width: 650px;
  }
  :global(.secondary-header-btn),
  :global(.lock-vault-btn) {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-250, #d6d6d6);
    border-radius: 6px;
    color: var(--color-boulder-750, #383838);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 600;
    gap: 6px;
    height: 32px;
    padding: 0 10px;
    transition: all 0.12s ease;
  }
  :global(.secondary-header-btn:hover),
  :global(.lock-vault-btn:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-400);
    color: var(--color-boulder-950);
  }
  :global(.new-project-button) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 6px;
    color: #fff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 700;
    gap: 6px;
    height: 32px;
    padding: 0 12px;
    transition: opacity 0.15s ease;
  }
  :global(.new-project-button:hover) {
    opacity: 0.92;
  }

  .error {
    background: #fef2f2;
    border: 1px solid #fee2e2;
    border-radius: 8px;
    color: #991b1b;
    font-size: 12px;
    margin: 14px 0 0;
    padding: 10px 14px;
  }
  .error p {
    margin: 0 0 8px;
    font-weight: 500;
  }
  .error-actions {
    display: flex;
    gap: 8px;
  }
  :global(.error-btn) {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: #fff;
    border: 1px solid #fca5a5;
    border-radius: 5px;
    color: #991b1b;
    font: inherit;
    font-size: 11.5px;
    font-weight: 600;
    padding: 4px 9px;
    cursor: pointer;
  }
  :global(.primary-error-btn) {
    background: #dc2626;
    color: #fff;
    border: 0;
  }

  .publication-status {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 12px;
    margin-top: 14px;
    padding: 8px 14px;
    background: #f0fdf4;
    border: 1px solid #bbf7d0;
    border-radius: 8px;
  }
  .pub-info {
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .pub-pulse-dot {
    background: #16a34a;
    border-radius: 50%;
    box-shadow: 0 0 0 2px rgb(22 163 74 / 20%);
    height: 7px;
    width: 7px;
    flex-shrink: 0;
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
  .pub-text {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }
  .pub-title {
    color: #14532d;
    font-size: 12px;
    font-weight: 600;
  }
  .pub-prod-badge {
    background: #fef3c7;
    border: 1px solid #fde68a;
    border-radius: 3px;
    color: #92400e;
    font-size: 9px;
    font-weight: 750;
    padding: 1px 4px;
    margin-left: 4px;
    letter-spacing: 0.04em;
  }
  .pub-desc {
    color: #166534;
    font-size: 11px;
    opacity: 0.85;
  }
  :global(.deactivate-btn) {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    background: #fff;
    border: 1px solid #86efac;
    border-radius: 5px;
    color: #15803d;
    font: inherit;
    font-size: 11px;
    font-weight: 650;
    padding: 4px 10px;
    cursor: pointer;
    transition: all 0.12s ease;
  }
  :global(.deactivate-btn:hover) {
    background: #fef2f2;
    border-color: #fca5a5;
    color: #b91c1c;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.5;
      transform: scale(1.15);
    }
  }

  :global(button:disabled) {
    cursor: not-allowed;
    opacity: 0.5;
  }
  .workspace {
    display: grid;
    gap: 16px;
    grid-template-columns: minmax(230px, 0.26fr) minmax(0, 0.74fr);
    margin-top: 18px;
    min-height: calc(100vh - 210px);
  }
  .loading,
  .empty-workspace,
  .locked-workspace {
    align-items: center;
    color: var(--color-boulder-500);
    display: flex;
    flex: 1;
    flex-direction: column;
    font-size: 13px;
    justify-content: center;
    min-height: 360px;
    text-align: center;
  }
  .locked-workspace {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    margin-top: 28px;
    min-height: 420px;
    padding: 32px;
  }
  .lock-icon-box {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    border-radius: 50%;
    color: var(--color-east-bay-700);
    display: flex;
    height: 64px;
    justify-content: center;
    margin-bottom: 16px;
    width: 64px;
  }
  .locked-workspace h2 {
    color: var(--color-boulder-950);
    font-size: 20px;
    margin: 0;
  }
  .locked-workspace p {
    color: var(--color-boulder-600);
    font-size: 13px;
    margin: 8px 0 20px;
    max-width: 440px;
  }
  .empty-workspace h2 {
    color: var(--color-boulder-900);
    font-size: 17px;
    margin: 0;
  }
  .empty-workspace p {
    margin: 7px 0 17px;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  @media (max-width: 800px) {
    .secrets-page {
      padding: 22px 16px;
    }
    .page-header {
      align-items: flex-start;
      flex-direction: column;
      gap: 18px;
    }
    .workspace {
      grid-template-columns: 1fr;
    }
    .workspace :global(.projects-panel) {
      max-height: 280px;
    }
    :global(.new-project-button) {
      justify-content: center;
      width: 100%;
    }
  }
</style>
