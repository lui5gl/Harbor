<script lang="ts">
  import { t } from "#lib/i18n";
  import { Menu } from "@ark-ui/svelte/menu";
  import { Loader2, Lock, Plus, Power, RefreshCw, Save, ShieldCheck, Unlock } from "@lucide/svelte";
  import { onMount } from "svelte";
  import DeleteConfirmationDialog from "./DeleteConfirmationDialog.svelte";
  import EnvironmentDialog from "./EnvironmentDialog.svelte";
  import ProductionActivationDialog from "./ProductionActivationDialog.svelte";
  import ProfileEditor from "./ProfileEditor.svelte";
  import ProfilesPanel from "./ProfilesPanel.svelte";
  import ProjectDialog from "./ProjectDialog.svelte";
  import { createSecretsStore } from "../application/secretsStore.svelte";
  import { createSettingsStore } from "#lib/features/settings/application/settingsStore.svelte";

  const secrets = createSecretsStore();
  const preferences = createSettingsStore();
  let projectDialogOpen = $state(false);
  let environmentDialogOpen = $state(false);
  let productionDialogOpen = $state(false);
  let deleteDialogOpen = $state(false);
  let deleteMode = $state<"project" | "environment" | "variable" | null>(null);
  let pendingVariableId = $state<number | null>(null);
  let pendingActivationId = $state<number | null>(null);
  let pendingActivationLabel = $state("");
  let inactivityTimer: number | undefined;
  function resetInactivityTimer() {
    window.clearTimeout(inactivityTimer);
    if (!secrets.isLocked) {
      inactivityTimer = window.setTimeout(() => void secrets.lock(), 5 * 60 * 1000);
    }
  }
  async function unlockSecrets() {
    await secrets.unlock();
    resetInactivityTimer();
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
      for (const environment of project.environments) {
        if (environment.id === secrets.activeEnvironmentId)
          return {
            projectName: project.name,
            environmentName: environment.name,
            isProduction: environment.isProduction,
          };
      }
    }
    return null;
  });

  onMount(() => {
    preferences.load();
    secrets.start();
    void secrets.load();
    const activityEvents = ["pointerdown", "keydown", "touchstart"] as const;
    activityEvents.forEach((event) => window.addEventListener(event, resetInactivityTimer));
    resetInactivityTimer();
    return () => {
      window.clearTimeout(inactivityTimer);
      activityEvents.forEach((event) => window.removeEventListener(event, resetInactivityTimer));
      secrets.dispose();
    };
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

  function selectEnvironment(projectId: number, environmentId: number) {
    secrets.selectEnvironment(projectId, environmentId);
    if (!preferences.settings.autoActivateSecrets || environmentId === secrets.activeEnvironmentId)
      return;
    const environment = secrets.projects
      .find((project) => project.id === projectId)
      ?.environments.find((item) => item.id === environmentId);
    if (!environment) return;
    if (environment.isProduction) {
      pendingActivationId = environment.id;
      pendingActivationLabel = `${secrets.projects.find((project) => project.id === projectId)?.name ?? "Project"} / ${environment.name}`;
      productionDialogOpen = true;
    } else {
      void secrets.activateEnvironment(environment.id);
    }
  }

  function terminalExecutable() {
    if (preferences.settings.terminalType === "custom")
      return preferences.settings.customTerminalPath;
    return preferences.settings.terminalType;
  }

  function confirmProductionActivation() {
    const id = pendingActivationId;
    pendingActivationId = null;
    pendingActivationLabel = "";
    productionDialogOpen = false;
    if (id !== null) void secrets.activateEnvironment(id);
  }

  const primaryButtonClass =
    "inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-bold text-white transition-colors hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50";
</script>

<svelte:head
  ><title>Harbor | {t("secrets.title")}</title><meta
    name="description"
    content={t("secrets.description")}
  /></svelte:head
>

<main
  class="mx-auto flex w-full max-w-310 flex-1 flex-col px-4 py-6 sm:px-8 sm:pb-8"
  aria-labelledby="secrets-title"
>
  <header class="flex flex-wrap items-center justify-between gap-5">
    <div class="flex flex-col">
      <div class="mb-1.5 flex items-center gap-3">
        <p class="text-[11px] font-bold uppercase tracking-[0.06em] text-east-bay-700">
          {t("secrets.eyebrow")}
        </p>
        {#if !secrets.isLoading}
          <span
            class="inline-flex items-center gap-1.5 rounded-full border border-boulder-200 bg-white px-2 py-0.5 text-[11px] font-semibold text-boulder-700 shadow-sm"
          >
            {#if secrets.isSaving}<Loader2 size={12} class="animate-spin text-east-bay-700" /><span
                >{t("secrets.saving")}</span
              >{:else if secrets.hasUnsavedChanges}<Save size={12} class="text-amber-600" /><span
                >{t("secrets.unsavedChanges")}</span
              >{:else if !secrets.error && secrets.canWrite}<ShieldCheck
                size={13}
                class="text-emerald-600"
              /><span>{t("secrets.vaultStatus")}</span>{/if}
          </span>
        {/if}
      </div>
      <h1 id="secrets-title" class="text-[26px] leading-tight font-bold text-boulder-950">
        {t("secrets.title")}
      </h1>
      <p class="mt-1 max-w-2xl text-[12.5px] leading-relaxed text-boulder-600">
        {t("secrets.description")}
      </p>
    </div>

    <div class="flex flex-wrap items-center gap-2">
      <button
        class={primaryButtonClass}
        type="button"
        disabled={!secrets.canWrite}
        onclick={() => (projectDialogOpen = true)}
        ><Plus size={15} strokeWidth={2.4} /><span>{t("secrets.newProject")}</span></button
      >
    </div>
  </header>

  {#if secrets.error}
    <div
      class="mt-3.5 rounded-lg border border-red-100 bg-red-50 px-3.5 py-2.5 text-xs text-red-800"
      role="alert"
    >
      <p class="font-medium">{secrets.error}</p>
      <div class="mt-2 flex gap-2">
        <button
          type="button"
          class="inline-flex items-center gap-1.5 rounded-md border border-red-300 bg-white px-2.5 py-1 text-[11.5px] font-semibold text-red-800"
          disabled={secrets.isBusy || secrets.isSaving}
          onclick={() => void secrets.reload()}><RefreshCw size={13} /><span>Recargar</span></button
        >
        {#if secrets.hasUnsavedChanges}<button
            type="button"
            class="inline-flex items-center gap-1.5 rounded-md bg-red-600 px-2.5 py-1 text-[11.5px] font-semibold text-white"
            disabled={secrets.isBusy || secrets.isSaving}
            onclick={() => void secrets.flush()}><Save size={13} /><span>Reintentar</span></button
          >{/if}
      </div>
    </div>
  {/if}

  {#if activeEnvironmentInfo}
    <div
      class="mt-3.5 flex flex-wrap items-center justify-between gap-3 rounded-lg border border-emerald-200 bg-emerald-50 px-3.5 py-2"
      role="region"
      aria-label="Perfil activo en Windows"
    >
      <div class="flex items-center gap-2.5">
        <span
          class="size-2 shrink-0 animate-pulse rounded-full bg-emerald-600 ring-2 ring-emerald-600/20"
        ></span>
        <div class="flex flex-wrap items-center gap-2">
          <span class="text-xs font-semibold text-emerald-900"
            >{t("secrets.activeInPowerShell")}:
            <strong
              >{activeEnvironmentInfo.projectName} / {activeEnvironmentInfo.environmentName}</strong
            >{#if activeEnvironmentInfo.isProduction}<span
                class="ml-1 rounded border border-amber-200 bg-amber-100 px-1 text-[9px] font-bold text-amber-800"
                >PROD</span
              >{/if}</span
          >
          <span class="text-[11px] text-emerald-800/85">{t("secrets.publishedToWindows")}</span>
        </div>
      </div>
      <button
        type="button"
        class="inline-flex cursor-pointer items-center gap-1.5 rounded-md border border-emerald-300 bg-white px-2.5 py-1 text-[11px] font-semibold text-emerald-700 transition-colors hover:border-red-300 hover:bg-red-50 hover:text-red-700 disabled:opacity-50"
        disabled={secrets.isBusy || secrets.isSaving}
        onclick={() => void secrets.deactivateEnvironment()}
        ><Power size={13} strokeWidth={2.2} /><span>{t("secrets.deactivateFromWindows")}</span
        ></button
      >
    </div>
  {/if}

  {#if secrets.isLoading}
    <div
      class="flex min-h-90 flex-1 flex-col items-center justify-center gap-2 text-[13px] text-boulder-500"
      role="status"
    >
      <Loader2 size={24} class="animate-spin" /><span>{t("secrets.loading")}</span>
    </div>
  {:else if secrets.isLocked}
    <div
      class="mt-7 flex min-h-105 flex-1 flex-col items-center justify-center rounded-lg border border-boulder-200 bg-white p-8 text-center"
    >
      <div
        class="mb-4 flex size-16 items-center justify-center rounded-full border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
      >
        <Lock size={30} strokeWidth={2.2} />
      </div>
      <h2 class="text-xl font-bold text-boulder-950">{t("secrets.vaultLockedTitle")}</h2>
      <p class="my-2 max-w-md text-[13px] text-boulder-600">{t("secrets.vaultLockedDesc")}</p>
      <button
        class={`mt-3 ${primaryButtonClass}`}
        type="button"
        onclick={() => void unlockSecrets()}
        ><Unlock size={15} strokeWidth={2.2} /><span>{t("secrets.unlockVault")}</span></button
      >
    </div>
  {:else if secrets.projects.length === 0}
    <div
      class="flex min-h-90 flex-1 flex-col items-center justify-center text-center text-[13px] text-boulder-500"
    >
      <h2 class="text-[17px] font-bold text-boulder-900">{t("secrets.noProjects")}</h2>
      <p class="mt-2 mb-4">{t("secrets.noProjectsDescription")}</p>
      <button
        class={primaryButtonClass}
        type="button"
        disabled={!secrets.canWrite}
        onclick={() => (projectDialogOpen = true)}
        ><Plus size={16} strokeWidth={2.4} /><span>{t("secrets.createProject")}</span></button
      >
    </div>
  {:else}
    <div
      class="mt-4.5 grid min-h-[calc(100vh-210px)] grid-cols-1 gap-4 lg:grid-cols-[minmax(230px,0.26fr)_minmax(0,0.74fr)]"
    >
      <div class="max-h-70 lg:max-h-none">
        <ProfilesPanel
          projects={secrets.projects}
          selectedProjectId={secrets.selectedProjectId}
          selectedEnvironmentId={secrets.selectedEnvironmentId}
          activeEnvironmentId={secrets.activeEnvironmentId}
          onSelectEnvironment={selectEnvironment}
          onAddEnvironment={openEnvironmentDialog}
        />
      </div>
      {#if secrets.selectedProject && secrets.selectedEnvironment}
        <ProfileEditor
          project={secrets.selectedProject}
          environment={secrets.selectedEnvironment}
          activeEnvironmentId={secrets.activeEnvironmentId}
          disabled={!secrets.canWrite ||
            secrets.selectedEnvironment.id === secrets.activeEnvironmentId}
          busy={secrets.isBusy || secrets.isSaving || !secrets.canWrite}
          onSelectEnvironment={(id) => selectEnvironment(secrets.selectedProject!.id, id)}
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
          onOpenTerminal={() => {
            if (!secrets.selectedEnvironment) return;
            if (
              !secrets.selectedEnvironment.isProduction ||
              window.confirm(
                `Open an isolated terminal with production secrets for ${targetActivationLabel}?`,
              )
            )
              void secrets.openTerminal(secrets.selectedEnvironment.id, terminalExecutable());
          }}
        />
      {/if}
    </div>
  {/if}
</main>

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
