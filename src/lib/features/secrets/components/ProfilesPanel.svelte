<script lang="ts">
  import { t } from "#lib/i18n";
  import {
    ChevronDown,
    ChevronRight,
    Code2,
    FlaskConical,
    FolderGit2,
    Plus,
    Search,
    ShieldAlert,
    X,
  } from "@lucide/svelte";
  import type { Project } from "../types";

  type ProfilesPanelProps = {
    projects: Project[];
    selectedProjectId: number | null;
    selectedEnvironmentId: number | null;
    activeEnvironmentId: number | null;
    onSelectEnvironment: (projectId: number, environmentId: number) => void;
    onAddEnvironment: (projectId: number) => void;
  };

  let {
    projects,
    selectedProjectId,
    selectedEnvironmentId,
    activeEnvironmentId,
    onSelectEnvironment,
    onAddEnvironment,
  }: ProfilesPanelProps = $props();
  let searchQuery = $state("");
  let collapsedProjectIds = $state<number[]>([]);

  let filteredProjects = $derived.by(() => {
    const query = searchQuery.trim().toLowerCase();
    if (!query) return projects;
    return projects
      .map((project) => ({
        ...project,
        environments: project.environments.filter(
          (environment) =>
            project.name.toLowerCase().includes(query) ||
            environment.name.toLowerCase().includes(query),
        ),
      }))
      .filter((project) => project.environments.length > 0);
  });

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
  <div class="mb-2.5 border-b border-boulder-200 px-1 pt-0.5 pb-2.5">
    <h2 class="text-[13px] font-bold text-boulder-950">{t("secrets.projects")}</h2>
    <p class="mt-0.5 text-[11px] text-boulder-500">
      {projects.length}
      {projects.length === 1 ? t("secrets.project") : t("secrets.projectPlural")}
    </p>
  </div>

  {#if projects.length > 1}
    <div
      class="mb-2.5 flex items-center gap-1.5 rounded-md border border-boulder-200 bg-boulder-50 px-2 text-boulder-400 transition focus-within:border-east-bay-400 focus-within:bg-white focus-within:ring-2 focus-within:ring-east-bay-400/15"
    >
      <Search size={13} />
      <input
        class="h-7.5 min-w-0 flex-1 border-0 bg-transparent text-xs text-boulder-800 outline-none"
        placeholder={t("secrets.searchProjects")}
        bind:value={searchQuery}
      />
      {#if searchQuery}<button
          class="inline-flex size-5 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-boulder-100 hover:text-east-bay-800"
          type="button"
          aria-label="Clear search"
          onclick={() => (searchQuery = "")}><X size={12} /></button
        >{/if}
    </div>
  {/if}

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
              <button
                class="inline-flex size-5.5 cursor-pointer items-center justify-center rounded text-boulder-400 hover:bg-boulder-100 hover:text-east-bay-800"
                type="button"
                title={`Add environment to ${project.name}`}
                aria-label={`Add environment to ${project.name}`}
                onclick={() => onAddEnvironment(project.id)}><Plus size={14} /></button
              >
            </div>

            {#if !isCollapsed}
              <div class="mt-0.5 flex flex-col gap-0.5 pl-5">
                {#each project.environments as environment (environment.id)}
                  {@const isSelected = environment.id === selectedEnvironmentId}
                  {@const isActive = environment.id === activeEnvironmentId}
                  <button
                    class={`flex h-7.5 w-full cursor-pointer items-center gap-2 rounded-md px-2 text-left text-xs transition-colors ${isSelected ? "bg-east-bay-100 text-east-bay-950" : "text-boulder-700 hover:bg-boulder-50 hover:text-boulder-950"}`}
                    type="button"
                    onclick={() => onSelectEnvironment(project.id, environment.id)}
                  >
                    {#if environment.isProduction}<ShieldAlert
                        size={13}
                        class="shrink-0 text-amber-700"
                        aria-label="Production"
                      />{:else if environment.name.toLowerCase() === "development"}<Code2
                        size={13}
                        class="shrink-0 text-east-bay-700"
                        aria-label="Development"
                      />{:else if environment.name.toLowerCase() === "staging"}<FlaskConical
                        size={13}
                        class="shrink-0 text-violet-700"
                        aria-label="Staging"
                      />{:else}<span
                        class={`size-1.5 shrink-0 rounded-full ${isActive ? "bg-emerald-500 ring-2 ring-emerald-500/20" : "bg-boulder-300"}`}
                      ></span>{/if}
                    <span class="min-w-0 flex-1 truncate font-medium">{environment.name}</span>
                    {#if environment.isProduction}<span
                        class="rounded border border-amber-200 bg-amber-50 px-1 text-[8px] font-bold text-amber-800"
                        title="Production environment">PROD</span
                      >{/if}
                    <span
                      class="min-w-5 rounded-full bg-boulder-100 px-1.5 text-center text-[10px] font-semibold text-boulder-500"
                      >{environment.secrets.length}</span
                    >
                  </button>
                {/each}
              </div>
            {/if}
          </section>
        {/each}
      </div>
    {/if}
  </div>
</aside>
