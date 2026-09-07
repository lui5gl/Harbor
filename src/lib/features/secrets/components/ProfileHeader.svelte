<script lang="ts">
  import { t } from "$lib/i18n";
  import {
    Copy,
    Download,
    FileUp,
    FolderGit2,
    KeyRound,
    MoreHorizontal,
    Pencil,
    Shield,
    Trash2,
  } from "@lucide/svelte";
  import { Button, DropdownMenu } from "bits-ui";
  import type { Environment, Project } from "../types";

  type Props = {
    project: Project;
    environment: Environment;
    activeEnvironmentId: number | null;
    onSelectEnvironment: (id: number) => void;
    onRenameProject: () => void;
    onRequestDeleteProject: () => void;
    onOpenSettings: () => void;
    onOpenGenerator: () => void;
    onOpenImport: () => void;
    onCopyEnvironment: () => void;
    onDownloadEnvironment: () => void;
    onRequestDeleteEnvironment: () => void;
    onActivateEnvironment: () => void;
  };

  let {
    project,
    environment,
    activeEnvironmentId,
    onSelectEnvironment,
    onRenameProject,
    onRequestDeleteProject,
    onOpenSettings,
    onOpenGenerator,
    onOpenImport,
    onCopyEnvironment,
    onDownloadEnvironment,
    onRequestDeleteEnvironment,
    onActivateEnvironment,
  }: Props = $props();

  let isActive = $derived(environment.id === activeEnvironmentId);
</script>

<header class="project-header">
  <div class="project-identity">
    <FolderGit2 size={17} />
    <div>
      <p>{t("secrets.project")}</p>
      <h2>{project.name}</h2>
    </div>
  </div>
  <DropdownMenu.Root>
    <DropdownMenu.Trigger class="icon-button" type="button" aria-label={t("secrets.actions")}>
      <MoreHorizontal size={17} />
    </DropdownMenu.Trigger>
    <DropdownMenu.Portal>
      <DropdownMenu.Content class="actions-menu" sideOffset={6} align="end">
        <DropdownMenu.Item class="menu-item" onclick={onRenameProject}>
          <Pencil size={14} />{t("secrets.renameProject")}
        </DropdownMenu.Item>
        <DropdownMenu.Separator class="menu-separator" />
        <DropdownMenu.Item class="menu-item destructive" onclick={onRequestDeleteProject}>
          <Trash2 size={14} />{t("secrets.deleteProject")}
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Portal>
  </DropdownMenu.Root>
</header>

<nav class="environment-tabs" aria-label={t("secrets.environments")}>
  {#each project.environments as item (item.id)}
    <Button.Root
      class={item.id === environment.id ? "selected" : ""}
      type="button"
      onclick={() => onSelectEnvironment(item.id)}
    >
      <span class:active={item.id === activeEnvironmentId} class="tab-dot"></span>
      {item.name}
    </Button.Root>
  {/each}
</nav>

<div class="environment-heading">
  <div>
    <div class="title-row">
      <h3>{environment.name}</h3>
    </div>
    <p>
      {environment.secrets.length}
      {environment.secrets.length === 1 ? t("secrets.variable") : t("secrets.variables")}
      {#if isActive}<span class="active-label">{t("secrets.activeInShell")}</span>{/if}
    </p>
  </div>
  <div class="header-actions">
    {#if !isActive}
      <Button.Root class="activate-button" type="button" onclick={onActivateEnvironment}>
        <Shield size={14} />{t("secrets.activate")}
      </Button.Root>
    {/if}
    <DropdownMenu.Root>
      <DropdownMenu.Trigger class="icon-button" type="button" aria-label={t("secrets.actions")}>
        <MoreHorizontal size={17} />
      </DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content class="actions-menu" sideOffset={6} align="end">
          <DropdownMenu.Item class="menu-item" onclick={onOpenSettings}>
            <Pencil size={14} />{t("secrets.environmentSettings")}
          </DropdownMenu.Item>
          <DropdownMenu.Item class="menu-item" onclick={onOpenGenerator}>
            <KeyRound size={14} />{t("secrets.generateSecret")}
          </DropdownMenu.Item>
          <DropdownMenu.Item class="menu-item" onclick={onOpenImport}>
            <FileUp size={14} />{t("secrets.importEnv")}
          </DropdownMenu.Item>
          <DropdownMenu.Item class="menu-item" onclick={onCopyEnvironment}>
            <Copy size={14} />{t("secrets.copyEnv")}
          </DropdownMenu.Item>
          <DropdownMenu.Item class="menu-item" onclick={onDownloadEnvironment}>
            <Download size={14} />{t("secrets.downloadEnv")}
          </DropdownMenu.Item>
          <DropdownMenu.Separator class="menu-separator" />
          <DropdownMenu.Item class="menu-item destructive" onclick={onRequestDeleteEnvironment}>
            <Trash2 size={14} />{t("secrets.deleteEnvironment")}
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  </div>
</div>

<style>
  .project-header,
  .environment-heading,
  .project-identity,
  .header-actions,
  .title-row {
    align-items: center;
    display: flex;
  }
  .project-header,
  .environment-heading {
    justify-content: space-between;
  }
  .project-header {
    border-bottom: 1px solid var(--color-boulder-100);
    padding-bottom: 12px;
  }
  .project-identity {
    color: var(--color-east-bay-700);
    gap: 9px;
  }
  .project-identity p {
    color: var(--color-boulder-500);
    font-size: 10px;
    font-weight: 700;
    margin: 0 0 2px;
    text-transform: uppercase;
  }
  .project-identity h2 {
    color: var(--color-boulder-950);
    font-size: 15px;
    margin: 0;
  }
  .header-actions {
    gap: 7px;
  }
  :global(.activate-button) {
    align-items: center;
    background: #166534;
    border: 0;
    border-radius: 6px;
    color: #fff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    gap: 5px;
    height: 32px;
    padding: 0 10px;
  }
  :global(.icon-button) {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: inline-flex;
    height: 32px;
    justify-content: center;
    width: 32px;
  }
  .environment-tabs {
    border-bottom: 1px solid var(--color-boulder-200);
    display: flex;
    gap: 3px;
    overflow-x: auto;
    padding-top: 10px;
  }
  .environment-tabs :global(button) {
    align-items: center;
    background: transparent;
    border: 0;
    border-bottom: 2px solid transparent;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    gap: 5px;
    height: 34px;
    padding: 0 9px;
    white-space: nowrap;
  }
  .environment-tabs :global(button.selected) {
    border-bottom-color: var(--color-east-bay-700);
    color: var(--color-east-bay-900);
  }
  .tab-dot {
    background: var(--color-boulder-300);
    border-radius: 50%;
    height: 6px;
    width: 6px;
  }
  .tab-dot.active {
    background: #16a34a;
  }
  .environment-heading {
    padding: 14px 0 11px;
  }
  .environment-heading h3 {
    color: var(--color-boulder-950);
    font-size: 16px;
    margin: 0;
  }
  .environment-heading p {
    color: var(--color-boulder-500);
    font-size: 11px;
    margin: 4px 0 0;
  }
  .active-label {
    color: #15803d;
    font-weight: 650;
    margin-left: 8px;
  }
  :global(.actions-menu) {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    min-width: 185px;
    padding: 4px;
    z-index: 20;
  }
  :global(.menu-item) {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: flex;
    font: inherit;
    font-size: 12px;
    gap: 8px;
    padding: 8px;
    width: 100%;
  }
  :global(.menu-item:hover) {
    background: var(--color-boulder-50);
  }
  :global(.menu-item.destructive) {
    color: #b91c1c;
  }
  :global(.menu-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }
  @media (max-width: 700px) {
    .project-header,
    .environment-heading {
      align-items: flex-start;
      flex-direction: column;
    }
    .header-actions {
      width: 100%;
    }
    .header-actions :global(.activate-button) {
      flex: 1;
      justify-content: center;
    }
  }
</style>
