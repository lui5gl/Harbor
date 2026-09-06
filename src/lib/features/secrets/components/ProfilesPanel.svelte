<script lang="ts">
  import { ChevronDown, ChevronRight, FolderGit2, Lock, Plus, Search, X } from "@lucide/svelte";
  import { t } from "$lib/i18n";
  import { Button, ScrollArea } from "bits-ui";
  import type { Project } from "../types";

  type ProfilesPanelProps = {
    projects: Project[];
    selectedProjectId: number | null;
    selectedEnvironmentId: number | null;
    activeEnvironmentId: number | null;
    onSelectEnvironment: (projectId: number, environmentId: number) => void;
    onAddEnvironment: (projectId: number) => void;
  };

  let { projects, selectedProjectId, selectedEnvironmentId, activeEnvironmentId, onSelectEnvironment, onAddEnvironment }: ProfilesPanelProps = $props();
  let searchQuery = $state("");
  let collapsedProjectIds = $state<number[]>([]);

  let filteredProjects = $derived.by(() => {
    const query = searchQuery.trim().toLowerCase();
    if (!query) return projects;
    return projects
      .map((project) => ({
        ...project,
        environments: project.environments.filter((environment) =>
          project.name.toLowerCase().includes(query) || environment.name.toLowerCase().includes(query)
        )
      }))
      .filter((project) => project.environments.length > 0);
  });

  function toggleProject(projectId: number) {
    collapsedProjectIds = collapsedProjectIds.includes(projectId)
      ? collapsedProjectIds.filter((id) => id !== projectId)
      : [...collapsedProjectIds, projectId];
  }
</script>

<aside class="projects-panel" aria-label={`${t("secrets.projects")} & ${t("secrets.environments")}`}>
  <div class="panel-header">
    <div>
      <h2>{t("secrets.projects")}</h2>
      <p>{projects.length} {projects.length === 1 ? t("secrets.project") : t("secrets.projectPlural")}</p>
    </div>
  </div>

  {#if projects.length > 1}
    <div class="search-box">
      <Search size={13} />
      <input class="search-input" placeholder={t("secrets.searchProjects")} bind:value={searchQuery} />
      {#if searchQuery}
        <Button.Root class="clear-search" type="button" aria-label="Clear search" onclick={() => (searchQuery = "")}><X size={12} /></Button.Root>
      {/if}
    </div>
  {/if}

  <ScrollArea.Root class="project-scroll-area" type="auto">
    <ScrollArea.Viewport class="project-list-viewport">
      {#if filteredProjects.length === 0}
        <p class="empty-state">{t("secrets.noProjectsFound")}</p>
      {:else}
        <div class="project-list">
          {#each filteredProjects as project (project.id)}
            {@const isCollapsed = collapsedProjectIds.includes(project.id)}
            <section class={`project-group${project.id === selectedProjectId ? " selected-project" : ""}`}>
              <div class="project-heading">
                <Button.Root class="project-toggle" type="button" aria-expanded={!isCollapsed} onclick={() => toggleProject(project.id)}>
                  {#if isCollapsed}<ChevronRight size={13} />{:else}<ChevronDown size={13} />{/if}
                  <FolderGit2 size={14} class="project-icon" />
                  <span>{project.name}</span>
                </Button.Root>
                <Button.Root class="add-environment" type="button" title={`Add environment to ${project.name}`} aria-label={`Add environment to ${project.name}`} onclick={() => onAddEnvironment(project.id)}><Plus size={14} /></Button.Root>
              </div>

              {#if !isCollapsed}
                <div class="environment-list">
                  {#each project.environments as environment (environment.id)}
                    {@const isSelected = environment.id === selectedEnvironmentId}
                    {@const isActive = environment.id === activeEnvironmentId}
                    <Button.Root class={`environment-item${isSelected ? " selected" : ""}`} type="button" onclick={() => onSelectEnvironment(project.id, environment.id)}>
                      <span class={`status-dot${isActive ? " active" : ""}`}></span>
                      <span class="environment-name">{environment.name}</span>
                      {#if environment.isProduction}<Lock size={11} class="production-lock" />{/if}
                    </Button.Root>
                  {/each}
                </div>
              {/if}
            </section>
          {/each}
        </div>
      {/if}
    </ScrollArea.Viewport>
    <ScrollArea.Scrollbar class="project-scrollbar" orientation="vertical"><ScrollArea.Thumb class="project-scrollbar-thumb" /></ScrollArea.Scrollbar>
  </ScrollArea.Root>
</aside>

<style>
  .projects-panel { background: #fff; border: 1px solid var(--color-boulder-200); border-radius: 8px; box-sizing: border-box; display: flex; flex-direction: column; height: 100%; max-height: 580px; padding: 14px; }
  .panel-header { border-bottom: 1px solid var(--color-boulder-100); margin-bottom: 10px; padding: 0 4px 10px; }
  h2, p { margin: 0; }
  h2 { color: var(--color-boulder-950); font-size: 13px; font-weight: 700; }
  .panel-header p { color: var(--color-boulder-500); font-size: 11px; margin-top: 3px; }
  .search-box { align-items: center; background: var(--color-boulder-50); border: 1px solid var(--color-boulder-200); border-radius: 6px; color: var(--color-boulder-400); display: flex; gap: 6px; margin-bottom: 10px; padding: 0 8px; }
  .search-input { background: transparent; border: 0; color: var(--color-boulder-800); font: inherit; font-size: 12px; height: 30px; outline: none; width: 100%; }
  :global(.clear-search), :global(.add-environment) { align-items: center; background: transparent; border: 0; border-radius: 4px; color: var(--color-boulder-400); cursor: pointer; display: inline-flex; height: 22px; justify-content: center; padding: 0; width: 22px; }
  :global(.clear-search:hover), :global(.add-environment:hover) { background: var(--color-boulder-100); color: var(--color-east-bay-800); }
  :global(.project-scroll-area) { flex: 1; min-height: 0; }
  :global(.project-list-viewport) { height: 100%; width: 100%; }
  .project-list { display: flex; flex-direction: column; gap: 10px; padding-right: 3px; }
  .project-group { border-radius: 6px; }
  .project-heading { align-items: center; display: flex; justify-content: space-between; }
  :global(.project-toggle) { align-items: center; background: transparent; border: 0; border-radius: 4px; color: var(--color-boulder-800); cursor: pointer; display: flex; flex: 1; font: inherit; font-size: 12px; font-weight: 700; gap: 5px; min-width: 0; padding: 5px; text-align: left; }
  :global(.project-toggle:hover) { background: var(--color-boulder-50); }
  :global(.project-icon) { color: var(--color-east-bay-700); flex-shrink: 0; }
  .environment-list { display: flex; flex-direction: column; gap: 2px; margin-top: 2px; padding-left: 22px; }
  :global(.environment-item) { align-items: center; background: transparent; border: 1px solid transparent; border-radius: 5px; color: var(--color-boulder-700); cursor: pointer; display: flex; font: inherit; font-size: 12px; gap: 6px; min-height: 29px; padding: 0 7px; text-align: left; width: 100%; }
  :global(.environment-item:hover) { background: var(--color-boulder-50); }
  :global(.environment-item.selected) { background: var(--color-east-bay-50); border-color: var(--color-east-bay-200); color: var(--color-east-bay-950); font-weight: 650; }
  .status-dot { background: var(--color-boulder-300); border-radius: 50%; flex-shrink: 0; height: 6px; width: 6px; }
  .status-dot.active { background: #16a34a; box-shadow: 0 0 0 2px rgb(22 163 74 / 18%); }
  .environment-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  :global(.production-lock) { color: #b45309; flex-shrink: 0; margin-left: auto; }
  .empty-state { color: var(--color-boulder-400); font-size: 12px; padding: 24px 6px; text-align: center; }
  :global(.project-scrollbar) { display: flex; padding: 2px; width: 6px; }
  :global(.project-scrollbar-thumb) { background: var(--color-boulder-300); border-radius: 999px; flex: 1; }
</style>
