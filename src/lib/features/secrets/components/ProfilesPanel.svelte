<script lang="ts">
  import { t } from "#lib/i18n";
  import {
    ArrowDown,
    ArrowUp,
    ArrowUpDown,
    ChevronDown,
    ChevronRight,
    ChevronsDownUp,
    ChevronsUpDown,
    FolderGit2,
    Layers,
    Plus,
    Search,
    X,
  } from "@lucide/svelte";
  import EnvironmentActionsMenu, { type EnvironmentAction } from "./EnvironmentActionsMenu.svelte";
  import type { Project } from "../types";

  type ProfilesPanelProps = {
    projects: Project[];
    selectedProjectId: number | null;
    selectedEnvironmentId: number | null;
    activeEnvironmentId: number | null;
    onSelectEnvironment: (projectId: number, environmentId: number) => void;
    onAddEnvironment: (projectId: number) => void;
    onEnvironmentAction: (
      projectId: number,
      environmentId: number,
      action: EnvironmentAction,
    ) => void;
    busy?: boolean;
    activationBusy?: boolean;
    canAddProject: boolean;
    onAddProject: () => void;
  };

  let {
    projects,
    selectedProjectId,
    selectedEnvironmentId,
    activeEnvironmentId,
    onSelectEnvironment,
    onAddEnvironment,
    onEnvironmentAction,
    busy = false,
    activationBusy = false,
    canAddProject,
    onAddProject,
  }: ProfilesPanelProps = $props();
  type SortDirection = "none" | "asc" | "desc";

  let searchQuery = $state("");
  let collapsedProjectIds = $state<number[]>([]);
  let projectSort = $state<SortDirection>("none");
  let environmentSorts = $state<Record<number, SortDirection>>({});

  const collator = new Intl.Collator(undefined, { numeric: true, sensitivity: "base" });

  function sortByName<T extends { name: string }>(items: T[], direction: SortDirection): T[] {
    if (direction === "none") return items;
    const sign = direction === "asc" ? 1 : -1;
    return [...items].sort((a, b) => sign * collator.compare(a.name, b.name));
  }

  function nextSort(direction: SortDirection): SortDirection {
    return direction === "none" ? "asc" : direction === "asc" ? "desc" : "none";
  }

  let filteredProjects = $derived.by(() => {
    const query = searchQuery.trim().toLowerCase();
    const visible = query
      ? projects.filter((project) => project.name.toLowerCase().includes(query))
      : projects;
    return sortByName(visible, projectSort);
  });

  function visibleEnvironments(project: Project) {
    return sortByName(project.environments, environmentSorts[project.id] ?? "none");
  }

  let allCollapsed = $derived(
    filteredProjects.length > 0 &&
      filteredProjects.every((project) => collapsedProjectIds.includes(project.id)),
  );

  function toggleAllProjects() {
    collapsedProjectIds = allCollapsed ? [] : projects.map((project) => project.id);
  }

  function toggleProject(projectId: number) {
    collapsedProjectIds = collapsedProjectIds.includes(projectId)
      ? collapsedProjectIds.filter((id) => id !== projectId)
      : [...collapsedProjectIds, projectId];
  }
</script>

<aside
  class="flex h-full min-h-[min(560px,calc(100dvh-280px))] flex-col rounded-lg border border-boulder-200 bg-white p-3.5 shadow-sm"
  aria-label={`${t("secrets.projects")} & ${t("secrets.environments")}`}
>
  <div
    class="mb-2.5 flex items-start justify-between gap-2 border-b border-boulder-200 px-1 pt-0.5 pb-2.5"
  >
    <div>
      <h2 class="text-[13px] font-bold text-boulder-950">{t("secrets.projects")}</h2>
      <p class="mt-0.5 text-[11px] text-boulder-500">
        {projects.length}
        {projects.length === 1 ? t("secrets.project") : t("secrets.projectPlural")}
      </p>
    </div>
    <button
      class="inline-flex size-7 shrink-0 cursor-pointer items-center justify-center rounded-md bg-east-bay-900 text-white transition-colors hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
      type="button"
      title={t("secrets.newProject")}
      aria-label={t("secrets.newProject")}
      disabled={!canAddProject}
      onclick={onAddProject}><Plus size={15} strokeWidth={2.4} /></button
    >
  </div>

  <div class="mb-2.5 flex items-center gap-1.5">
    <div
      class="flex min-w-0 flex-1 items-center gap-1.5 rounded-md border border-boulder-200 bg-boulder-50 px-2 text-boulder-400 transition focus-within:border-east-bay-400 focus-within:bg-white focus-within:ring-2 focus-within:ring-east-bay-400/15"
    >
      <Search size={13} />
      <input
        class="h-7.5 min-w-0 flex-1 border-0 bg-transparent text-xs text-boulder-800 outline-none"
        placeholder={t("secrets.searchProjects")}
        aria-label={t("secrets.searchProjects")}
        bind:value={searchQuery}
      />
      {#if searchQuery}<button
          class="inline-flex size-5 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-boulder-100 hover:text-east-bay-800"
          type="button"
          aria-label={t("secrets.clearFilter")}
          onclick={() => (searchQuery = "")}><X size={12} /></button
        >{/if}
    </div>
    <button
      class="inline-flex size-8.5 shrink-0 cursor-pointer items-center justify-center rounded-md border border-boulder-200 bg-white text-boulder-500 transition-colors hover:bg-boulder-100 disabled:cursor-not-allowed disabled:opacity-50"
      type="button"
      title={allCollapsed ? t("secrets.expandAll") : t("secrets.collapseAll")}
      aria-label={allCollapsed ? t("secrets.expandAll") : t("secrets.collapseAll")}
      disabled={filteredProjects.length === 0}
      onclick={toggleAllProjects}
      >{#if allCollapsed}<ChevronsUpDown size={14} />{:else}<ChevronsDownUp
          size={14}
        />{/if}</button
    >
    <button
      class={`inline-flex size-8.5 shrink-0 cursor-pointer items-center justify-center rounded-md border transition-colors ${projectSort === "none" ? "border-boulder-200 bg-white text-boulder-500 hover:bg-boulder-100" : "border-east-bay-200 bg-east-bay-50 text-east-bay-700"}`}
      type="button"
      title={`${t("secrets.sortBy")} ${t("secrets.projects")}`}
      aria-label={`${t("secrets.sortBy")} ${t("secrets.projects")}`}
      onclick={() => (projectSort = nextSort(projectSort))}
      >{#if projectSort === "asc"}<ArrowUp size={14} />{:else if projectSort === "desc"}<ArrowDown
          size={14}
        />{:else}<ArrowUpDown size={14} />{/if}</button
    >
  </div>

  <div class="min-h-0 flex-1 overflow-y-auto">
    {#if filteredProjects.length === 0}
      <p class="py-8 text-center text-xs text-boulder-500">{t("secrets.noProjectsFound")}</p>
    {:else}
      <div class="flex flex-col gap-2 pr-1">
        {#each filteredProjects as project (project.id)}
          {@const isCollapsed = collapsedProjectIds.includes(project.id)}
          <section
            class={`rounded-md ${project.id === selectedProjectId ? "bg-east-bay-50/40" : ""}`}
          >
            <div class="flex items-center justify-between">
              <button
                class="flex min-w-0 flex-1 cursor-pointer items-center gap-1.5 rounded-md px-1.5 py-1.5 text-left text-xs font-bold text-boulder-800 hover:bg-boulder-50"
                type="button"
                aria-expanded={!isCollapsed}
                onclick={() => toggleProject(project.id)}
              >
                {#if isCollapsed}<ChevronRight size={13} />{:else}<ChevronDown size={13} />{/if}
                <FolderGit2 size={14} class="shrink-0 text-east-bay-700" />
                <span class="truncate">{project.name}</span>
              </button>
              {#if project.environments.length > 1}
                {@const environmentSort = environmentSorts[project.id] ?? "none"}
                <button
                  class={`inline-flex size-5.5 cursor-pointer items-center justify-center rounded transition-colors ${environmentSort === "none" ? "text-boulder-400 hover:bg-boulder-100 hover:text-east-bay-800" : "bg-east-bay-100 text-east-bay-700"}`}
                  type="button"
                  title={`${t("secrets.sortBy")} ${t("secrets.environments")}`}
                  aria-label={`${t("secrets.sortBy")} ${t("secrets.environments")} · ${project.name}`}
                  onclick={() => (environmentSorts[project.id] = nextSort(environmentSort))}
                  >{#if environmentSort === "asc"}<ArrowUp size={13} />{:else if environmentSort === "desc"}<ArrowDown
                      size={13}
                    />{:else}<ArrowUpDown size={13} />{/if}</button
                >
              {/if}
              <button
                class="inline-flex size-5.5 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-boulder-100 hover:text-east-bay-800"
                type="button"
                title={`Add environment to ${project.name}`}
                aria-label={`Add environment to ${project.name}`}
                onclick={() => onAddEnvironment(project.id)}><Plus size={14} /></button
              >
            </div>

            {#if !isCollapsed}
              {@const environments = visibleEnvironments(project)}
              <div class="mt-0.5 ml-3 flex flex-col gap-0.5 pl-4">
                {#each environments as environment, index (environment.id)}
                  {@const isSelected = environment.id === selectedEnvironmentId}
                  {@const isActive = environment.id === activeEnvironmentId}
                  {@const isLast = index === environments.length - 1}
                  <div class="group relative">
                    {#if isLast}
                      <span
                        class="pointer-events-none absolute top-0 -left-4 h-1/2 w-3 rounded-bl-md border-b border-l border-boulder-300"
                        aria-hidden="true"
                      ></span>
                    {:else}
                      <span
                        class="pointer-events-none absolute top-0 -bottom-0.5 -left-4 w-px bg-boulder-300"
                        aria-hidden="true"
                      ></span>
                      <span
                        class="pointer-events-none absolute top-1/2 -left-4 h-px w-3 bg-boulder-300"
                        aria-hidden="true"
                      ></span>
                    {/if}
                    <button
                      class={`flex h-7.5 w-full cursor-pointer items-center gap-2 rounded-md pr-8 pl-2 text-left text-xs transition-colors ${isSelected ? "bg-east-bay-100 text-east-bay-950" : "text-boulder-700 hover:bg-boulder-50 hover:text-boulder-950"}`}
                      type="button"
                      onclick={() => onSelectEnvironment(project.id, environment.id)}
                    >
                      <Layers size={13} class="shrink-0 text-east-bay-700" aria-hidden="true" />
                      <span class="min-w-0 flex-1 truncate font-medium">{environment.name}</span>
                      {#if isActive}<span
                          class="size-1.5 shrink-0 rounded-full bg-emerald-500 ring-2 ring-emerald-500/20"
                          title={t("secrets.inWindows")}
                        ></span>{/if}
                    </button>
                    <div class="absolute top-1/2 right-1 -translate-y-1/2">
                      <EnvironmentActionsMenu
                        disabled={!canAddProject || isActive}
                        {busy}
                        {activationBusy}
                        {isActive}
                        canCompare={project.environments.length > 1}
                        visible={isSelected}
                        onAction={(action) =>
                          onEnvironmentAction(project.id, environment.id, action)}
                      />
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </section>
        {/each}
      </div>
    {/if}
  </div>
</aside>
