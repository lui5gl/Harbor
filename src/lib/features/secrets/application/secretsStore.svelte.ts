import { emit, listen } from "@tauri-apps/api/event";
import { invoke, isTauri } from "@tauri-apps/api/core";
import {
  activateSecretProfile,
  deactivateSecretEnvironment,
  loadSecretProfiles,
  openSecretTerminal,
  saveSecretProfiles,
} from "../infrastructure/secretsRepository";
import { createSecretsConfiguration, validateSecretsConfiguration } from "./secretsConfiguration";
import {
  appendEnvironment,
  appendProject,
  appendSecret,
  cloneEnvironment as cloneEnvironmentData,
  getSecretCounters,
  moveItem,
  removeSecret,
  updateEnvironment as updateEnvironmentData,
  updateProject as updateProjectData,
  updateSecret,
} from "./secretsMutations";
import type { Environment, Project, Secret, SecretsConfiguration } from "../types";

const demoProjects: Project[] = [
  {
    id: 1,
    name: "General",
    environments: [
      {
        id: 1,
        name: "Development",
        secrets: [{ id: 1, key: "API_URL", value: "https://api-dev.example.test" }],
      },
      {
        id: 2,
        name: "Production",
        secrets: [
          { id: 2, key: "API_URL", value: "https://api.example.com" },
          { id: 3, key: "API_TOKEN", value: "replace-with-a-secret" },
        ],
      },
    ],
  },
];

const isNativeApp = isTauri();

type SecretsStore = {
  readonly projects: Project[];
  readonly selectedProjectId: number | null;
  readonly selectedEnvironmentId: number | null;
  readonly activeEnvironmentId: number | null;
  readonly selectedProject: Project | undefined;
  readonly selectedEnvironment: Environment | undefined;
  readonly isLoading: boolean;
  readonly isSaving: boolean;
  readonly isBusy: boolean;
  readonly hasUnsavedChanges: boolean;
  readonly canWrite: boolean;
  readonly isLocked: boolean;
  readonly error: string;
  start: () => void;
  load: () => Promise<void>;
  reload: () => Promise<void>;
  flush: () => Promise<boolean>;
  lock: () => Promise<boolean>;
  unlock: () => Promise<void>;
  dispose: () => void;
  selectEnvironment: (projectId: number, environmentId: number) => void;
  prepareEnvironmentCreation: (projectId: number | null) => boolean;
  createProject: (name: string) => void;
  createEnvironment: (name: string) => void;
  cloneEnvironment: (name: string) => void;
  renameProject: (name: string) => void;
  moveProject: (sourceId: number, targetId: number) => void;
  moveEnvironment: (projectId: number, sourceId: number, targetId: number) => void;
  saveEnvironment: (name: string) => void;
  addVariable: () => number | null;
  addCustomVariable: (key: string, value: string) => void;
  updateVariable: (id: number, field: "key" | "value", value: string) => void;
  deleteProject: () => void;
  deleteEnvironment: () => void;
  deleteVariable: (id: number) => void;
  importVariables: (items: { key: string; value: string }[], replaceAll: boolean) => void;
  activateEnvironment: (environmentId: number) => Promise<boolean>;
  deactivateEnvironment: () => Promise<boolean>;
  openTerminal: (environmentId: number, terminalExecutable: string) => Promise<boolean>;
  exportConfiguration: () => SecretsConfiguration;
  restoreConfiguration: (configuration: SecretsConfiguration) => Promise<boolean>;
};

export function createSecretsStore(): SecretsStore {
  let projects = $state<Project[]>([]);
  let selectedProjectId = $state<number | null>(null);
  let selectedEnvironmentId = $state<number | null>(null);
  let activeEnvironmentId = $state<number | null>(null);
  let isLoading = $state(true);
  let isSaving = $state(false);
  let isBusy = $state(false);
  let hasUnsavedChanges = $state(false);
  let loaded = $state(false);
  let revision = 0;
  let editVersion = 0;
  let saving: Promise<boolean> | undefined;
  let loading: Promise<void> | undefined;
  let isLocked = $state(false);
  let error = $state("");
  let nextProjectId = 2;
  let nextEnvironmentId = 3;
  let nextSecretId = 4;
  let saveTimer: number | undefined;
  let isSelfEmitting = false;
  let unlisten: (() => void) | undefined;

  const selectedProject = $derived(projects.find((project) => project.id === selectedProjectId));
  const selectedEnvironment = $derived(
    selectedProject?.environments.find((environment) => environment.id === selectedEnvironmentId),
  );

  function allEnvironments(): Environment[] {
    return projects.flatMap((project) => project.environments);
  }

  function setCounters() {
    const counters = getSecretCounters(projects);
    nextProjectId = counters.nextProjectId;
    nextEnvironmentId = counters.nextEnvironmentId;
    nextSecretId = counters.nextSecretId;
  }

  async function loadWithTimeout(): Promise<SecretsConfiguration> {
    return new Promise((resolve, reject) => {
      const timeout = window.setTimeout(
        () =>
          reject(
            new Error(
              "Loading secure secrets timed out. Check that Harbor is running and try again.",
            ),
          ),
        8_000,
      );
      loadSecretProfiles()
        .then(resolve, reject)
        .finally(() => window.clearTimeout(timeout));
    });
  }

  async function read() {
    try {
      const configuration = isNativeApp
        ? await loadWithTimeout()
        : { projects: structuredClone(demoProjects), activeEnvironmentId: null, revision: 0 };
      projects = configuration.projects;
      activeEnvironmentId = configuration.activeEnvironmentId;
      revision = configuration.revision;
      loaded = true;
      const currentProject =
        projects.find((project) => project.id === selectedProjectId) ?? projects[0];
      selectedProjectId = currentProject?.id ?? null;
      selectedEnvironmentId = currentProject?.environments.some(
        (environment) => environment.id === selectedEnvironmentId,
      )
        ? selectedEnvironmentId
        : (currentProject?.environments[0]?.id ?? null);
      setCounters();
      error = "";
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
      loaded = false;
    } finally {
      isLoading = false;
    }
  }

  async function load() {
    if (hasUnsavedChanges || isSaving || isBusy) return;
    if (!loading) loading = read().finally(() => (loading = undefined));
    await loading;
  }

  async function reload() {
    if (isSaving || isBusy) return;
    if (hasUnsavedChanges && !window.confirm("Discard unsaved changes and reload secure storage?"))
      return;
    window.clearTimeout(saveTimer);
    hasUnsavedChanges = false;
    await load();
  }

  function writable() {
    return loaded && !isBusy && !isLoading;
  }

  function editableEnvironment() {
    if (!writable()) return false;
    if (selectedEnvironmentId === activeEnvironmentId && activeEnvironmentId !== null) {
      error = "Deactivate this environment before editing it.";
      return false;
    }
    return true;
  }

  function scheduleSave() {
    hasUnsavedChanges = true;
    editVersion++;
    window.clearTimeout(saveTimer);
    saveTimer = window.setTimeout(() => void save(), 450);
  }

  function updateProject(
    projectId: number,
    update: (project: Project) => Project,
    shouldSave = true,
  ) {
    if (!writable()) return;
    projects = updateProjectData(projects, projectId, update);
    if (shouldSave) scheduleSave();
  }

  function updateEnvironment(update: (environment: Environment) => Environment, shouldSave = true) {
    if (!editableEnvironment()) return;
    if (selectedProjectId === null || selectedEnvironmentId === null) return;
    projects = updateEnvironmentData(projects, selectedProjectId, selectedEnvironmentId, update);
    if (shouldSave) scheduleSave();
  }

  async function persist(): Promise<boolean> {
    isSaving = true;
    try {
      while (hasUnsavedChanges) {
        const validationError = validateSecretsConfiguration(projects);
        if (validationError) {
          error = validationError;
          return false;
        }
        const version = editVersion;
        const configuration = createSecretsConfiguration(projects, activeEnvironmentId, revision);
        if (isNativeApp) revision = await saveSecretProfiles(configuration);
        if (version === editVersion) hasUnsavedChanges = false;
      }
      error = "";
      if (isNativeApp) {
        isSelfEmitting = true;
        try {
          await emit("secrets-updated");
        } catch {
          /* Storage is already committed. */
        }
        window.setTimeout(() => (isSelfEmitting = false), 300);
      }
      return true;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
      return false;
    } finally {
      isSaving = false;
    }
  }

  async function save(): Promise<boolean> {
    if (!loaded) return false;
    if (!saving) saving = persist().finally(() => (saving = undefined));
    return saving;
  }

  async function flush() {
    window.clearTimeout(saveTimer);
    return save();
  }

  function selectEnvironment(projectId: number, environmentId: number) {
    selectedProjectId = projectId;
    selectedEnvironmentId = environmentId;
  }

  function prepareEnvironmentCreation(projectId: number | null): boolean {
    if (projectId === null || !writable()) return false;
    selectedProjectId = projectId;
    const project = projects.find((item) => item.id === projectId);
    if (!project?.environments.some((environment) => environment.id === selectedEnvironmentId)) {
      selectedEnvironmentId = project?.environments[0]?.id ?? null;
    }
    return true;
  }

  function createProject(name: string) {
    if (!writable()) return;
    const projectId = nextProjectId++;
    const environments: Environment[] = [
      { id: nextEnvironmentId++, name: "Development", secrets: [] },
      { id: nextEnvironmentId++, name: "Staging", secrets: [] },
      { id: nextEnvironmentId++, name: "Production", secrets: [] },
    ];
    projects = appendProject(projects, projectId, environments, name);
    selectedProjectId = projectId;
    selectedEnvironmentId = environments[0].id;
    scheduleSave();
  }

  function createEnvironment(name: string) {
    if (selectedProjectId === null || !writable()) return;
    const environmentId = nextEnvironmentId++;
    projects = appendEnvironment(projects, selectedProjectId, environmentId, name);
    scheduleSave();
    selectedEnvironmentId = environmentId;
  }

  function cloneEnvironment(name: string) {
    if (selectedProjectId === null || selectedEnvironmentId === null || !writable()) return;
    const environmentId = nextEnvironmentId++;
    const startSecretId = nextSecretId;
    const source = selectedEnvironment;
    if (!source) return;
    nextSecretId += source.secrets.length;
    projects = cloneEnvironmentData(
      projects,
      selectedProjectId,
      selectedEnvironmentId,
      environmentId,
      name,
      startSecretId,
    );
    selectedEnvironmentId = environmentId;
    scheduleSave();
  }

  function renameProject(name: string) {
    if (selectedProjectId === null) return;
    updateProject(selectedProjectId, (project) => ({ ...project, name }));
  }

  function moveProject(sourceId: number, targetId: number) {
    if (!writable()) return;
    projects = moveItem(projects, sourceId, targetId);
    scheduleSave();
  }

  function moveEnvironment(projectId: number, sourceId: number, targetId: number) {
    updateProject(projectId, (project) => ({
      ...project,
      environments: moveItem(project.environments, sourceId, targetId),
    }));
  }

  function saveEnvironment(name: string) {
    updateEnvironment((environment) => ({ ...environment, name }));
  }

  function addVariable(): number | null {
    if (!selectedEnvironment || !editableEnvironment()) return null;
    const id = nextSecretId++;
    projects = appendSecret(projects, selectedProjectId, selectedEnvironmentId, id, {
      key: "",
      value: "",
    });
    return id;
  }

  function addCustomVariable(key: string, value: string) {
    if (!editableEnvironment()) return;
    projects = appendSecret(projects, selectedProjectId, selectedEnvironmentId, nextSecretId++, {
      key,
      value,
    });
    scheduleSave();
  }

  function updateVariable(id: number, field: "key" | "value", value: string) {
    if (!editableEnvironment()) return;
    projects = updateSecret(projects, selectedProjectId, selectedEnvironmentId, id, field, value);
    scheduleSave();
  }

  function deleteProject() {
    if (selectedProjectId === null || !writable()) return;
    if (
      selectedProject?.environments.some((environment) => environment.id === activeEnvironmentId)
    ) {
      error = "Deactivate the active environment before deleting this project.";
      return;
    }
    const remaining = projects.filter((project) => project.id !== selectedProjectId);
    projects = remaining;
    selectedProjectId = remaining[0]?.id ?? null;
    selectedEnvironmentId = remaining[0]?.environments[0]?.id ?? null;
    scheduleSave();
  }

  function deleteEnvironment() {
    if (!editableEnvironment()) return;
    if (!selectedProject || !selectedEnvironment || selectedProject.environments.length <= 1)
      return;
    const remaining = selectedProject.environments.filter(
      (environment) => environment.id !== selectedEnvironment.id,
    );
    projects = updateProjectData(projects, selectedProject.id, (project) => ({
      ...project,
      environments: remaining,
    }));
    scheduleSave();
    selectedEnvironmentId = remaining[0].id;
  }

  function deleteVariable(id: number) {
    if (!editableEnvironment()) return;
    projects = removeSecret(projects, selectedProjectId, selectedEnvironmentId, id);
    scheduleSave();
  }

  function importVariables(items: { key: string; value: string }[], replaceAll: boolean) {
    updateEnvironment((environment) => {
      if (replaceAll) {
        return {
          ...environment,
          secrets: items.map((item) => ({ id: nextSecretId++, ...item })),
        };
      }
      const secrets = [...environment.secrets];
      for (const item of items) {
        const index = secrets.findIndex(
          (secret) => secret.key.toUpperCase() === item.key.toUpperCase(),
        );
        if (index < 0) secrets.push({ id: nextSecretId++, ...item });
        else secrets[index] = { ...secrets[index], ...item };
      }
      return { ...environment, secrets };
    });
  }

  async function activateEnvironment(environmentId: number): Promise<boolean> {
    if (!writable()) return false;
    isBusy = true;
    try {
      if (!(await flush())) return false;
      if (isNativeApp) {
        await activateSecretProfile(environmentId);
        const configuration = await loadWithTimeout();
        revision = configuration.revision;
        activeEnvironmentId = configuration.activeEnvironmentId;
        try {
          await emit("secrets-updated");
        } catch {
          /* Storage is already committed. */
        }
      } else {
        activeEnvironmentId = environmentId;
      }
      error = "";
      return true;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
      return false;
    } finally {
      isBusy = false;
    }
  }

  async function deactivateEnvironment(): Promise<boolean> {
    if (!writable()) return false;
    isBusy = true;
    try {
      if (!(await flush())) return false;
      if (isNativeApp) {
        await deactivateSecretEnvironment();
        const configuration = await loadWithTimeout();
        revision = configuration.revision;
        activeEnvironmentId = configuration.activeEnvironmentId;
        try {
          await emit("secrets-updated");
        } catch {
          /* Storage is already committed. */
        }
      } else activeEnvironmentId = null;
      error = "";
      return true;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
      return false;
    } finally {
      isBusy = false;
    }
  }

  async function openTerminal(environmentId: number, terminalExecutable: string): Promise<boolean> {
    if (!writable()) return false;
    isBusy = true;
    try {
      if (!(await flush())) return false;
      if (!isNativeApp)
        throw new Error("Isolated terminals are available in the desktop app only.");
      await openSecretTerminal(environmentId, terminalExecutable);
      error = "";
      return true;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
      return false;
    } finally {
      isBusy = false;
    }
  }

  function exportConfiguration() {
    return createSecretsConfiguration(projects, null, 0);
  }

  async function restoreConfiguration(configuration: SecretsConfiguration): Promise<boolean> {
    if (!writable() || activeEnvironmentId !== null) {
      error = "Deactivate the published environment before restoring a backup.";
      return false;
    }
    const validationError = validateSecretsConfiguration(configuration.projects);
    if (validationError) {
      error = validationError;
      return false;
    }
    projects = configuration.projects;
    selectedProjectId = projects[0]?.id ?? null;
    selectedEnvironmentId = projects[0]?.environments[0]?.id ?? null;
    setCounters();
    scheduleSave();
    return flush();
  }

  function beforeUnload(event: BeforeUnloadEvent) {
    if (hasUnsavedChanges || isSaving || isBusy) {
      event.preventDefault();
      event.returnValue = "";
    }
  }

  function dispose() {
    window.clearTimeout(saveTimer);
    if (hasUnsavedChanges) void save();
    window.removeEventListener("focus", handleFocus);
    window.removeEventListener("beforeunload", beforeUnload);
    unlisten?.();
  }

  async function lock(): Promise<boolean> {
    if (isLocked) return true;
    if (!(await flush())) return false;
    projects = [];
    selectedProjectId = null;
    selectedEnvironmentId = null;
    isLocked = true;
    return true;
  }

  async function unlock() {
    isLocked = false;
    await load();
  }

  function handleFocus() {
    if (isLocked) return;
    if (!isSaving && !isSelfEmitting) void load();
  }

  function start() {
    if (unlisten) return;
    window.addEventListener("focus", handleFocus);
    window.addEventListener("beforeunload", beforeUnload);
    if (isNativeApp) {
      listen("secrets-updated", () => {
        if (isLocked) return;
        if (!isSaving && !isSelfEmitting) void load();
      }).then((callback) => (unlisten = callback));
    }
  }

  return {
    get projects() {
      return projects;
    },
    get selectedProjectId() {
      return selectedProjectId;
    },
    get selectedEnvironmentId() {
      return selectedEnvironmentId;
    },
    get activeEnvironmentId() {
      return activeEnvironmentId;
    },
    get selectedProject() {
      return selectedProject;
    },
    get selectedEnvironment() {
      return selectedEnvironment;
    },
    get isLoading() {
      return isLoading;
    },
    get isSaving() {
      return isSaving;
    },
    get isBusy() {
      return isBusy;
    },
    get hasUnsavedChanges() {
      return hasUnsavedChanges;
    },
    get canWrite() {
      return loaded && !isLoading && !isBusy && !isLocked;
    },
    get isLocked() {
      return isLocked;
    },
    get error() {
      return error;
    },
    start,
    load,
    reload,
    flush,
    lock,
    unlock,
    dispose,
    selectEnvironment,
    prepareEnvironmentCreation,
    createProject,
    createEnvironment,
    cloneEnvironment,
    renameProject,
    moveProject,
    moveEnvironment,
    saveEnvironment,
    addVariable,
    addCustomVariable,
    updateVariable,
    deleteProject,
    deleteEnvironment,
    deleteVariable,
    importVariables,
    activateEnvironment,
    deactivateEnvironment,
    openTerminal,
    exportConfiguration,
    restoreConfiguration,
  };
}
