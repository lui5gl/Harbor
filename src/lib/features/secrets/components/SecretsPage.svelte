<script lang="ts">
  import {
    Loader2,
    Plus,
    ShieldCheck,
    RefreshCw,
    Save,
    Power,
    Download,
    Upload,
    Lock,
    Unlock,
  } from "@lucide/svelte";
  import { Button } from "bits-ui";
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
    <div>
      <div class="top-row">
        <p class="eyebrow">{t("secrets.eyebrow")}</p>
        {#if !secrets.isLoading}
          <span class="save-status">
            {#if secrets.isSaving}
              <Loader2 size={12} class="spin" />{t("secrets.saving")}
            {:else if secrets.hasUnsavedChanges}
              <Save size={13} />Unsaved changes
            {:else if !secrets.error && secrets.canWrite}
              <ShieldCheck size={13} />{t("secrets.encryptedKeyring")}
            {/if}
          </span>
        {/if}
      </div>
      <h1 id="secrets-title">{t("secrets.title")}</h1>
      <p class="page-description">{t("secrets.description")}</p>
    </div>
    <Button.Root
      class="new-project-button"
      type="button"
      disabled={!secrets.canWrite}
      onclick={() => (projectDialogOpen = true)}
    >
      <Plus size={17} />{t("secrets.newProject")}
    </Button.Root>
  </header>

  <div class="backup-toolbar">
    <Button.Root
      type="button"
      disabled={!secrets.canWrite || secrets.isSaving}
      onclick={() => void openBackup("export")}><Download size={14} />Encrypted backup</Button.Root
    >
    <Button.Root
      type="button"
      disabled={!secrets.canWrite || secrets.isSaving || secrets.activeEnvironmentId !== null}
      onclick={() => void openBackup("restore")}><Upload size={14} />Restore backup</Button.Root
    >
    <Button.Root
      type="button"
      class="lock-toolbar-btn"
      disabled={secrets.isLocked || !secrets.canWrite}
      onclick={() => secrets.lock()}><Lock size={14} />Lock vault</Button.Root
    >
  </div>

  {#if secrets.error}
    <div class="error" role="alert">
      <p>{secrets.error}</p>
      <Button.Root
        type="button"
        disabled={secrets.isBusy || secrets.isSaving}
        onclick={() => void secrets.reload()}><RefreshCw size={14} />Reload</Button.Root
      >
      {#if secrets.hasUnsavedChanges}<Button.Root
          type="button"
          disabled={secrets.isBusy || secrets.isSaving}
          onclick={() => void secrets.flush()}><Save size={14} />Retry save</Button.Root
        >{/if}
    </div>
  {/if}
  {#if secrets.activeEnvironmentId !== null}
    <div class="publication-status">
      <span>Published to Windows. Existing terminals keep their own copies after deactivation.</span
      >
      <Button.Root
        type="button"
        disabled={secrets.isBusy || secrets.isSaving}
        onclick={() => void secrets.deactivateEnvironment()}
        ><Power size={14} />Deactivate</Button.Root
      >
    </div>
  {/if}

  {#if secrets.isLoading}
    <div class="loading" role="status">{t("secrets.loading")}</div>
  {:else if secrets.isLocked}
    <div class="locked-workspace">
      <div class="lock-icon-box"><Lock size={32} /></div>
      <h2>Secrets vault is locked</h2>
      <p>
        All sensitive credentials have been unloaded from application memory to protect your
        privacy.
      </p>
      <Button.Root
        class="primary-button unlock-btn"
        type="button"
        onclick={() => void secrets.unlock()}
      >
        <Unlock size={15} /><span>Unlock vault</span>
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
        <Plus size={16} />{t("secrets.createProject")}
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
    padding: 32px;
    width: 100%;
  }
  .page-header,
  .top-row {
    align-items: center;
    display: flex;
  }
  .page-header {
    justify-content: space-between;
  }
  .top-row {
    gap: 12px;
    margin-bottom: 8px;
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
    background: var(--color-boulder-100);
    border: 1px solid var(--color-boulder-200);
    border-radius: 99px;
    color: var(--color-boulder-600);
    display: inline-flex;
    font-size: 11px;
    font-weight: 650;
    gap: 5px;
    padding: 3px 8px;
  }
  .save-status :global(svg) {
    color: #15803d;
  }
  .save-status :global(.spin) {
    animation: spin 1s linear infinite;
    color: var(--color-east-bay-700);
  }
  h1 {
    color: var(--color-boulder-950);
    font-size: 30px;
    letter-spacing: 0;
    line-height: 1;
    margin: 0;
  }
  .page-description {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.5;
    margin: 10px 0 0;
    max-width: 650px;
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
    gap: 7px;
    height: 36px;
    padding: 0 13px;
  }
  .error {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 6px;
    color: #b91c1c;
    font-size: 12px;
    margin: 20px 0 0;
    padding: 10px 12px;
  }
  .error p {
    margin: 0 0 8px;
  }
  .backup-toolbar {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 16px;
  }
  .backup-toolbar :global(button) {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    background: transparent;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    padding: 7px 10px;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
  }
  .publication-status {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 10px;
    margin-top: 16px;
    font-size: 12px;
    color: var(--color-boulder-700);
  }
  .publication-status :global(button),
  .error :global(button) {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
    background: white;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    padding: 7px 10px;
    font: inherit;
    margin-right: 6px;
  }
  :global(button:disabled) {
    cursor: not-allowed;
    opacity: 0.5;
  }
  .workspace {
    display: grid;
    gap: 16px;
    grid-template-columns: minmax(225px, 0.28fr) minmax(0, 0.72fr);
    margin-top: 28px;
    min-height: 580px;
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
