import { emit, listen } from "@tauri-apps/api/event";
import { isTauri } from "@tauri-apps/api/core";
import {
  activateSecretProfile,
  loadSecretProfiles,
  saveSecretProfiles,
} from "../infrastructure/secretsRepository";
import { createSecretsConfiguration, validateSecretsConfiguration } from "./secretsConfiguration";
import {
  appendEnvironment,
  appendProject,
  appendSecret,
  getSecretCounters,
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
        isProduction: false,
        secrets: [{ id: 1, key: "API_URL", value: "https://api-dev.example.test" }],
      },
      {
        id: 2,
        name: "Production",
        isProduction: true,
        secrets: [
          { id: 2, key: "API_URL", value: "https://api.example.com" },
          { id: 3, key: "API_TOKEN", value: "replace-with-a-secret" },
        ],
      },
    ],
  },
];

const isNativeApp = isTauri();

type DeleteMode = "project" | "environment" | "variable";

type SecretsStore = {
  readonly projects: Project[];
  readonly selectedProjectId: number | null;
  readonly selectedEnvironmentId: number | null;
  readonly activeEnvironmentId: number | null;
  readonly selectedProject: Project | undefined;
  readonly selectedEnvironment: Environment | undefined;
  readonly isLoading: boolean;
  readonly isSaving: boolean;
  readonly error: string;
  start: () => void;
  load: () => Promise<void>;
  dispose: () => void;
  selectEnvironment: (projectId: number, environmentId: number) => void;
  prepareEnvironmentCreation: (projectId: number | null) => boolean;
  createProject: (name: string, environmentName: string) => void;
  createEnvironment: (name: string, isProduction: boolean) => void;
  renameProject: (name: string) => void;
  saveEnvironment: (name: string, isProduction: boolean) => void;
  addVariable: () => number | null;
  addCustomVariable: (key: string, value: string) => void;
  updateVariable: (id: number, field: "key" | "value", value: string) => void;
  deleteProject: () => void;
  deleteEnvironment: () => void;
  deleteVariable: (id: number) => void;
  importVariables: (items: { key: string; value: string }[], replaceAll: boolean) => void;
  activateEnvironment: (environmentId: number) => Promise<boolean>;
  isProductionEnvironment: (environmentId: number) => boolean;
};

export function createSecretsStore(): SecretsStore {
  let projects = $state<Project[]>([]);
  let selectedProjectId = $state<number | null>(null);
  let selectedEnvironmentId = $state<number | null>(null);
  let activeEnvironmentId = $state<number | null>(null);
  let isLoading = $state(true);
  let isSaving = $state(false);
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

  async function load() {
    try {
      const configuration = isNativeApp
        ? await loadWithTimeout()
        : { projects: structuredClone(demoProjects), activeEnvironmentId: 1 };
      projects = configuration.projects.length
        ? configuration.projects
        : structuredClone(demoProjects);
      activeEnvironmentId =
        configuration.activeEnvironmentId ?? projects[0]?.environments[0]?.id ?? null;
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
      projects = structuredClone(demoProjects);
      selectedProjectId = 1;
      selectedEnvironmentId = 1;
      activeEnvironmentId = 1;
      setCounters();
    } finally {
      isLoading = false;
    }
  }

  function scheduleSave() {
    window.clearTimeout(saveTimer);
    saveTimer = window.setTimeout(() => void save(), 450);
  }

  function updateProject(
    projectId: number,
    update: (project: Project) => Project,
    shouldSave = true,
  ) {
    projects = updateProjectData(projects, projectId, update);
    if (shouldSave) scheduleSave();
  }

  function updateEnvironment(update: (environment: Environment) => Environment, shouldSave = true) {
    if (selectedProjectId === null || selectedEnvironmentId === null) return;
    projects = updateEnvironmentData(projects, selectedProjectId, selectedEnvironmentId, update);
    if (shouldSave) scheduleSave();
  }

  async function save() {
    if (!isNativeApp) return;

    const validationError = validateSecretsConfiguration(projects);
    if (validationError) {
      error = validationError;
      return;
    }

    isSaving = true;
    error = "";
    try {
      const configuration: SecretsConfiguration = createSecretsConfiguration(
        projects,
        activeEnvironmentId,
      );
      await saveSecretProfiles(configuration);
      isSelfEmitting = true;
      await emit("secrets-updated");
      window.setTimeout(() => (isSelfEmitting = false), 300);
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
    } finally {
      isSaving = false;
    }
  }

  function selectEnvironment(projectId: number, environmentId: number) {
    selectedProjectId = projectId;
    selectedEnvironmentId = environmentId;
  }

  function prepareEnvironmentCreation(projectId: number | null): boolean {
    if (projectId === null) return false;
    selectedProjectId = projectId;
    const project = projects.find((item) => item.id === projectId);
    if (!project?.environments.some((environment) => environment.id === selectedEnvironmentId)) {
      selectedEnvironmentId = project?.environments[0]?.id ?? null;
    }
    return true;
  }

  function createProject(name: string, environmentName: string) {
    const projectId = nextProjectId++;
    const environmentId = nextEnvironmentId++;
    projects = appendProject(projects, projectId, environmentId, name, environmentName);
    selectedProjectId = projectId;
    selectedEnvironmentId = environmentId;
    scheduleSave();
  }

  function createEnvironment(name: string, isProduction: boolean) {
    if (selectedProjectId === null) return;
    const environmentId = nextEnvironmentId++;
    projects = appendEnvironment(projects, selectedProjectId, environmentId, name, isProduction);
    scheduleSave();
    selectedEnvironmentId = environmentId;
  }

  function renameProject(name: string) {
    if (selectedProjectId === null) return;
    updateProject(selectedProjectId, (project) => ({ ...project, name }));
  }

  function saveEnvironment(name: string, isProduction: boolean) {
    updateEnvironment((environment) => ({ ...environment, name, isProduction }));
  }

  function addVariable(): number | null {
    if (!selectedEnvironment) return null;
    const id = nextSecretId++;
    projects = appendSecret(projects, selectedProjectId, selectedEnvironmentId, id, {
      key: "",
      value: "",
    });
    return id;
  }

  function addCustomVariable(key: string, value: string) {
    projects = appendSecret(projects, selectedProjectId, selectedEnvironmentId, nextSecretId++, {
      key,
      value,
    });
    scheduleSave();
  }

  function updateVariable(id: number, field: "key" | "value", value: string) {
    projects = updateSecret(projects, selectedProjectId, selectedEnvironmentId, id, field, value);
    scheduleSave();
  }

  function deleteProject() {
    if (selectedProjectId === null) return;
    const remaining = projects.filter((project) => project.id !== selectedProjectId);
    projects = remaining;
    selectedProjectId = remaining[0]?.id ?? null;
    selectedEnvironmentId = remaining[0]?.environments[0]?.id ?? null;
    if (!allEnvironments().some((environment) => environment.id === activeEnvironmentId)) {
      activeEnvironmentId = selectedEnvironmentId;
    }
    scheduleSave();
  }

  function deleteEnvironment() {
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
    if (activeEnvironmentId === selectedEnvironment.id) activeEnvironmentId = remaining[0].id;
  }

  function deleteVariable(id: number) {
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

  function isProductionEnvironment(environmentId: number) {
    return allEnvironments().some(
      (environment) => environment.id === environmentId && environment.isProduction,
    );
  }

  async function activateEnvironment(environmentId: number): Promise<boolean> {
    const previous = activeEnvironmentId;
    activeEnvironmentId = environmentId;
    window.clearTimeout(saveTimer);
    await save();
    if (error) {
      activeEnvironmentId = previous;
      return false;
    }

    try {
      if (isNativeApp) {
        await activateSecretProfile(environmentId);
        await emit("secrets-updated");
      }
      return true;
    } catch (caught) {
      error = caught instanceof Error ? caught.message : String(caught);
      activeEnvironmentId = previous;
      return false;
    }
  }

  function dispose() {
    window.clearTimeout(saveTimer);
    window.removeEventListener("focus", handleFocus);
    unlisten?.();
  }

  function handleFocus() {
    if (!isSaving && !isSelfEmitting) void load();
  }

  function start() {
    if (unlisten) return;
    window.addEventListener("focus", handleFocus);
    if (isNativeApp) {
      listen("secrets-updated", () => {
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
    get error() {
      return error;
    },
    start,
    load,
    dispose,
    selectEnvironment,
    prepareEnvironmentCreation,
    createProject,
    createEnvironment,
    renameProject,
    saveEnvironment,
    addVariable,
    addCustomVariable,
    updateVariable,
    deleteProject,
    deleteEnvironment,
    deleteVariable,
    importVariables,
    activateEnvironment,
    isProductionEnvironment,
  };
}
