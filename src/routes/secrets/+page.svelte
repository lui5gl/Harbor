<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { Loader2, Plus, ShieldCheck } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import { onMount } from "svelte";
  import DeleteConfirmationDialog from "$lib/features/secrets/DeleteConfirmationDialog.svelte";
  import EnvironmentDialog from "$lib/features/secrets/EnvironmentDialog.svelte";
  import ProductionActivationDialog from "$lib/features/secrets/ProductionActivationDialog.svelte";
  import ProfileEditor from "$lib/features/secrets/ProfileEditor.svelte";
  import ProfilesPanel from "$lib/features/secrets/ProfilesPanel.svelte";
  import ProjectDialog from "$lib/features/secrets/ProjectDialog.svelte";
  import type { Environment, Project, SecretsConfiguration } from "$lib/features/secrets/types";

  const demoProjects: Project[] = [{ id: 1, name: "General", environments: [
    { id: 1, name: "Development", isProduction: false, secrets: [{ id: 1, key: "API_URL", value: "https://api-dev.example.test" }] },
    { id: 2, name: "Production", isProduction: true, secrets: [{ id: 2, key: "API_URL", value: "https://api.example.com" }, { id: 3, key: "API_TOKEN", value: "replace-with-a-secret" }] }
  ] }];
  const isNativeApp = isTauri();
  let projects = $state<Project[]>([]);
  let selectedProjectId = $state<number | null>(null);
  let selectedEnvironmentId = $state<number | null>(null);
  let activeEnvironmentId = $state<number | null>(null);
  let nextProjectId = 2;
  let nextEnvironmentId = 3;
  let nextSecretId = 4;
  let isLoading = $state(true);
  let isSaving = $state(false);
  let error = $state("");
  let saveTimer: number | undefined;
  let isSelfEmitting = false;
  let projectDialogOpen = $state(false);
  let environmentDialogOpen = $state(false);
  let productionDialogOpen = $state(false);
  let deleteDialogOpen = $state(false);
  let deleteMode = $state<"project" | "environment" | "variable" | null>(null);
  let pendingVariableId = $state<number | null>(null);
  let pendingActivationId = $state<number | null>(null);
  let pendingActivationLabel = $state("");

  let selectedProject = $derived(projects.find((project) => project.id === selectedProjectId));
  let selectedEnvironment = $derived(selectedProject?.environments.find((environment) => environment.id === selectedEnvironmentId));
  let targetActivationLabel = $derived(
    selectedProject && selectedEnvironment
      ? `${selectedProject.name} / ${selectedEnvironment.name}`
      : "Selected environment"
  );
  let deleteTitle = $derived(deleteMode === "project" ? "Delete this project?" : deleteMode === "environment" ? "Delete this environment?" : "Delete this variable?");
  let deleteDescription = $derived(deleteMode === "project" ? `This permanently removes "${selectedProject?.name ?? "this project"}" and all of its environments.` : deleteMode === "environment" ? `This permanently removes "${selectedEnvironment?.name ?? "this environment"}" and all its variables.` : "This removes the variable from the selected environment.");

  onMount(() => {
    void loadConfiguration();
    const handleFocus = () => { if (!isSaving && !isSelfEmitting) void loadConfiguration(); };
    window.addEventListener("focus", handleFocus);
    let unlisten: (() => void) | undefined;
    if (isNativeApp) listen("secrets-updated", () => { if (!isSaving && !isSelfEmitting) void loadConfiguration(); }).then((callback) => (unlisten = callback));
    return () => { window.clearTimeout(saveTimer); window.removeEventListener("focus", handleFocus); unlisten?.(); };
  });

  async function loadConfiguration() {
    try {
      const configuration = isNativeApp ? await loadWithTimeout() : { projects: structuredClone(demoProjects), activeEnvironmentId: 1 };
      projects = configuration.projects.length ? configuration.projects : structuredClone(demoProjects);
      activeEnvironmentId = configuration.activeEnvironmentId ?? projects[0]?.environments[0]?.id ?? null;
      const currentProject = projects.find((project) => project.id === selectedProjectId) ?? projects[0];
      selectedProjectId = currentProject?.id ?? null;
      selectedEnvironmentId = currentProject?.environments.some((environment) => environment.id === selectedEnvironmentId) ? selectedEnvironmentId : currentProject?.environments[0]?.id ?? null;
      nextProjectId = Math.max(0, ...projects.map((project) => project.id)) + 1;
      nextEnvironmentId = Math.max(0, ...projects.flatMap((project) => project.environments.map((environment) => environment.id))) + 1;
      nextSecretId = Math.max(0, ...projects.flatMap((project) => project.environments.flatMap((environment) => environment.secrets.map((secret) => secret.id)))) + 1;
      error = "";
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
      projects = structuredClone(demoProjects);
      selectedProjectId = 1;
      selectedEnvironmentId = 1;
      activeEnvironmentId = 1;
    } finally { isLoading = false; }
  }

  function loadWithTimeout() {
    return new Promise<SecretsConfiguration>((resolve, reject) => {
      const timeout = window.setTimeout(() => reject(new Error("Loading secure secrets timed out. Check that Harbor is running and try again.")), 8_000);
      invoke<SecretsConfiguration>("load_secret_profiles").then(resolve, reject).finally(() => window.clearTimeout(timeout));
    });
  }

  function allEnvironments() { return projects.flatMap((project) => project.environments); }
  function scheduleSave() { window.clearTimeout(saveTimer); saveTimer = window.setTimeout(() => void saveConfiguration(), 450); }
  function updateProject(projectId: number, update: (project: Project) => Project, shouldSave = true) { projects = projects.map((project) => project.id === projectId ? update(project) : project); if (shouldSave) scheduleSave(); }
  function updateEnvironment(update: (environment: Environment) => Environment, shouldSave = true) { if (selectedProjectId === null || selectedEnvironmentId === null) return; updateProject(selectedProjectId, (project) => ({ ...project, environments: project.environments.map((environment) => environment.id === selectedEnvironmentId ? update(environment) : environment) }), shouldSave); }

  async function saveConfiguration() {
    if (!isNativeApp) return;
    for (const environment of allEnvironments()) {
      const keys = new Set<string>();
      for (const secret of environment.secrets) {
        const key = secret.key.trim();
        if (!key) continue;
        if (!/^[a-zA-Z0-9_]+$/.test(key)) { error = `Variable "${key}" in ${environment.name} may only use letters, numbers, and underscores`; return; }
        if (keys.has(key.toUpperCase())) { error = `Variable "${key}" is duplicated in ${environment.name}`; return; }
        keys.add(key.toUpperCase());
      }
    }
    isSaving = true;
    error = "";
    try {
      const configuration: SecretsConfiguration = { projects: projects.map((project) => ({ ...project, environments: project.environments.map((environment) => ({ ...environment, secrets: environment.secrets.filter((secret) => secret.key.trim()) })) })), activeEnvironmentId };
      await invoke("save_secret_profiles", { configuration });
      isSelfEmitting = true;
      await emit("secrets-updated");
      window.setTimeout(() => (isSelfEmitting = false), 300);
    } catch (caught) { error = caught instanceof Error ? caught.message : String(caught); } finally { isSaving = false; }
  }

  function selectEnvironment(projectId: number, environmentId: number) { selectedProjectId = projectId; selectedEnvironmentId = environmentId; }
  function createProject(name: string, environmentName: string) { const project: Project = { id: nextProjectId++, name, environments: [{ id: nextEnvironmentId++, name: environmentName, isProduction: false, secrets: [] }] }; projects = [...projects, project]; selectedProjectId = project.id; selectedEnvironmentId = project.environments[0].id; scheduleSave(); }
  function openEnvironmentDialog(projectId = selectedProjectId) { if (projectId === null) return; selectedProjectId = projectId; const project = projects.find((item) => item.id === projectId); if (!project?.environments.some((environment) => environment.id === selectedEnvironmentId)) selectedEnvironmentId = project?.environments[0]?.id ?? null; environmentDialogOpen = true; }
  function createEnvironment(name: string, isProduction: boolean) { if (selectedProjectId === null) return; const environment: Environment = { id: nextEnvironmentId++, name, isProduction, secrets: [] }; updateProject(selectedProjectId, (project) => ({ ...project, environments: [...project.environments, environment] })); selectedEnvironmentId = environment.id; }
  function requestDelete(mode: "project" | "environment" | "variable", variableId?: number) { deleteMode = mode; pendingVariableId = variableId ?? null; deleteDialogOpen = true; }
  function confirmDelete() {
    if (deleteMode === "project" && selectedProjectId !== null) { const remaining = projects.filter((project) => project.id !== selectedProjectId); projects = remaining; selectedProjectId = remaining[0]?.id ?? null; selectedEnvironmentId = remaining[0]?.environments[0]?.id ?? null; if (!allEnvironments().some((environment) => environment.id === activeEnvironmentId)) activeEnvironmentId = selectedEnvironmentId; scheduleSave(); }
    if (deleteMode === "environment" && selectedProject && selectedEnvironment && selectedProject.environments.length > 1) { const remaining = selectedProject.environments.filter((environment) => environment.id !== selectedEnvironment.id); updateProject(selectedProject.id, (project) => ({ ...project, environments: remaining })); selectedEnvironmentId = remaining[0].id; if (activeEnvironmentId === selectedEnvironment.id) activeEnvironmentId = remaining[0].id; }
    if (deleteMode === "variable" && pendingVariableId !== null) updateEnvironment((environment) => ({ ...environment, secrets: environment.secrets.filter((secret) => secret.id !== pendingVariableId) }));
    deleteMode = null; pendingVariableId = null; deleteDialogOpen = false;
  }
  function importVariables(items: { key: string; value: string }[], replaceAll: boolean) { updateEnvironment((environment) => { if (replaceAll) return { ...environment, secrets: items.map((item) => ({ id: nextSecretId++, ...item })) }; const secrets = [...environment.secrets]; for (const item of items) { const index = secrets.findIndex((secret) => secret.key.toUpperCase() === item.key.toUpperCase()); if (index < 0) secrets.push({ id: nextSecretId++, ...item }); else secrets[index] = { ...secrets[index], ...item }; } return { ...environment, secrets }; }); }
  async function requestActivation() {
    if (!selectedProject || !selectedEnvironment) return;
    if (selectedEnvironment.isProduction) {
      pendingActivationId = selectedEnvironment.id;
      pendingActivationLabel = `${selectedProject.name} / ${selectedEnvironment.name}`;
      productionDialogOpen = true;
      return;
    }
    await activate(selectedEnvironment.id);
  }
  async function activate(environmentId: number) { const previous = activeEnvironmentId; activeEnvironmentId = environmentId; window.clearTimeout(saveTimer); await saveConfiguration(); if (error) { activeEnvironmentId = previous; return; } try { if (isNativeApp) { await invoke("activate_secret_profile_for_powershell", { profileId: environmentId }); await emit("secrets-updated"); } } catch (caught) { error = caught instanceof Error ? caught.message : String(caught); activeEnvironmentId = previous; } }
  function confirmProductionActivation() { const id = pendingActivationId; pendingActivationId = null; pendingActivationLabel = ""; productionDialogOpen = false; if (id !== null) void activate(id); }
</script>

<svelte:head><title>Harbor | Secrets</title><meta name="description" content="Manage secrets by project and environment." /></svelte:head>
<main class="secrets-page" aria-labelledby="secrets-title">
  <header class="page-header"><div><div class="top-row"><p class="eyebrow">Environment configuration</p>{#if !isLoading}<span class="save-status">{#if isSaving}<Loader2 size={12} class="spin" />Saving...{:else if !error}<ShieldCheck size={13} />Encrypted keyring{/if}</span>{/if}</div><h1 id="secrets-title">Secrets</h1><p class="page-description">Keep variables in projects, then activate an environment only when you need it in PowerShell.</p></div><Button.Root class="new-project-button" type="button" onclick={() => (projectDialogOpen = true)}><Plus size={17} />New project</Button.Root></header>
  {#if error}<p class="error" role="alert">{error}</p>{/if}
  {#if isLoading}<div class="loading" role="status">Loading secure secrets...</div>
  {:else if projects.length === 0}<div class="empty-workspace"><h2>No projects yet</h2><p>Create a project to add environments and their variables.</p><Button.Root class="new-project-button" type="button" onclick={() => (projectDialogOpen = true)}><Plus size={16} />Create project</Button.Root></div>
  {:else}<div class="workspace"><ProfilesPanel {projects} {selectedProjectId} {selectedEnvironmentId} {activeEnvironmentId} onSelectEnvironment={selectEnvironment} onAddEnvironment={openEnvironmentDialog} />{#if selectedProject && selectedEnvironment}<ProfileEditor project={selectedProject} environment={selectedEnvironment} {activeEnvironmentId} onSelectEnvironment={(id) => selectEnvironment(selectedProject.id, id)} onAddEnvironment={() => openEnvironmentDialog()} onRenameProject={(name) => updateProject(selectedProject.id, (project) => ({ ...project, name }))} onRequestDeleteProject={() => requestDelete("project")} onSaveEnvironment={(name, isProduction) => updateEnvironment((environment) => ({ ...environment, name, isProduction }))} onRequestDeleteEnvironment={() => requestDelete("environment")} onAddVariable={() => updateEnvironment((environment) => ({ ...environment, secrets: [...environment.secrets, { id: nextSecretId++, key: "", value: "" }] }), false)} onAddCustomVariable={(key, value) => updateEnvironment((environment) => ({ ...environment, secrets: [...environment.secrets, { id: nextSecretId++, key, value }] }))} onImportVariables={importVariables} onUpdateVariable={(id, field, value) => updateEnvironment((environment) => ({ ...environment, secrets: environment.secrets.map((secret) => secret.id === id ? { ...secret, [field]: value } : secret) }))} onRequestDeleteVariable={(id) => requestDelete("variable", id)} onActivateEnvironment={() => void requestActivation()} />{/if}</div>{/if}
</main>
<ProjectDialog bind:open={projectDialogOpen} onCreate={createProject} />
<EnvironmentDialog bind:open={environmentDialogOpen} projectName={selectedProject?.name ?? "Project"} onCreate={createEnvironment} />
<ProductionActivationDialog bind:open={productionDialogOpen} environmentLabel={pendingActivationLabel || targetActivationLabel} onOpenChange={(open) => { if (!open) { pendingActivationId = null; pendingActivationLabel = ""; } }} onConfirm={confirmProductionActivation} />
<DeleteConfirmationDialog bind:open={deleteDialogOpen} title={deleteTitle} description={deleteDescription} actionLabel={deleteMode === "project" ? "Delete project" : deleteMode === "environment" ? "Delete environment" : "Delete variable"} confirmKeyword={deleteMode === "project" || selectedEnvironment?.isProduction ? "DELETE" : undefined} onOpenChange={(open) => { if (!open) deleteMode = null; }} onConfirm={confirmDelete} />

<style>
  .secrets-page { box-sizing: border-box; display: flex; flex: 1; flex-direction: column; margin: 0 auto; max-width: 1240px; padding: 32px; width: 100%; }.page-header,.top-row { align-items: center; display: flex; }.page-header { justify-content: space-between; }.top-row { gap: 12px; margin-bottom: 8px; }.eyebrow { color: var(--color-east-bay-700); font-size: 11px; font-weight: 750; letter-spacing: .06em; margin: 0; text-transform: uppercase; }.save-status { align-items: center; background: var(--color-boulder-100); border: 1px solid var(--color-boulder-200); border-radius: 99px; color: var(--color-boulder-600); display: inline-flex; font-size: 11px; font-weight: 650; gap: 5px; padding: 3px 8px; }.save-status :global(svg) { color: #15803d; }.save-status :global(.spin) { animation: spin 1s linear infinite; color: var(--color-east-bay-700); }h1 { color: var(--color-boulder-950); font-size: 30px; letter-spacing: 0; line-height: 1; margin: 0; }.page-description { color: var(--color-boulder-600); font-size: 13px; line-height: 1.5; margin: 10px 0 0; max-width: 650px; }:global(.new-project-button) { align-items: center; background: var(--color-east-bay-900); border: 0; border-radius: 6px; color: #fff; cursor: pointer; display: inline-flex; font: inherit; font-size: 12px; font-weight: 700; gap: 7px; height: 36px; padding: 0 13px; }.error { background: #fef2f2; border: 1px solid #fecaca; border-radius: 6px; color: #b91c1c; font-size: 12px; margin: 20px 0 0; padding: 10px 12px; }.workspace { display: grid; gap: 16px; grid-template-columns: minmax(225px,.28fr) minmax(0,.72fr); margin-top: 28px; min-height: 580px; }.loading,.empty-workspace { align-items: center; color: var(--color-boulder-500); display: flex; flex: 1; flex-direction: column; font-size: 13px; justify-content: center; min-height: 360px; text-align: center; }.empty-workspace h2 { color: var(--color-boulder-900); font-size: 17px; margin: 0; }.empty-workspace p { margin: 7px 0 17px; }@keyframes spin { to { transform: rotate(360deg); } }@media (max-width:800px) { .secrets-page { padding: 22px 16px; }.page-header { align-items: flex-start; flex-direction: column; gap: 18px; }.workspace { grid-template-columns: 1fr; }.workspace :global(.projects-panel) { max-height: 280px; }.new-project-button { justify-content: center; width: 100%; } }
</style>