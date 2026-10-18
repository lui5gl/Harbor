<script lang="ts">
  import { t } from "#lib/i18n";
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
    Terminal,
    GitCompare,
    ShieldAlert,
  } from "@lucide/svelte";
  import { Button, DropdownMenu, Tooltip } from "bits-ui";
  import type { Environment, Project } from "../types";

  type Props = {
    disabled?: boolean;
    busy?: boolean;
    project: Project;
    environment: Environment;
    activeEnvironmentId: number | null;
    onSelectEnvironment: (id: number) => void;
    onRenameProject: () => void;
    onRequestDeleteProject: () => void;
    onOpenSettings: () => void;
    onOpenGenerator: () => void;
    onOpenImport: () => void;
    onOpenClone: () => void;
    onOpenCompare: () => void;
    onCopyEnvironment: () => void;
    onDownloadEnvironment: () => void;
    onRequestDeleteEnvironment: () => void;
    onActivateEnvironment: () => void;
    onOpenTerminal: () => void;
  };

  let {
    disabled = false,
    busy = false,
    project,
    environment,
    activeEnvironmentId,
    onSelectEnvironment,
    onRenameProject,
    onRequestDeleteProject,
    onOpenSettings,
    onOpenGenerator,
    onOpenImport,
    onOpenClone,
    onOpenCompare,
    onCopyEnvironment,
    onDownloadEnvironment,
    onRequestDeleteEnvironment,
    onActivateEnvironment,
    onOpenTerminal,
  }: Props = $props();

  let isActive = $derived(environment.id === activeEnvironmentId);
</script>

<header class="workspace-header">
  <div class="breadcrumb-toolbar-row">
    <div class="breadcrumb-group">
      <div class="project-tag">
        <FolderGit2 size={15} class="project-icon" />
        <span class="project-name">{project.name}</span>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger
            class="project-menu-btn"
            type="button"
            aria-label={t("secrets.actions")}
          >
            <MoreHorizontal size={13} />
          </DropdownMenu.Trigger>
          <DropdownMenu.Portal>
            <DropdownMenu.Content class="actions-menu" sideOffset={5} align="start">
              <DropdownMenu.Item class="menu-item" onclick={onRenameProject}>
                <Pencil size={13} />
                <span>{t("secrets.renameProject")}</span>
              </DropdownMenu.Item>
              <DropdownMenu.Separator class="menu-separator" />
              <DropdownMenu.Item class="menu-item destructive" onclick={onRequestDeleteProject}>
                <Trash2 size={13} />
                <span>{t("secrets.deleteProject")}</span>
              </DropdownMenu.Item>
            </DropdownMenu.Content>
          </DropdownMenu.Portal>
        </DropdownMenu.Root>
      </div>

      <span class="breadcrumb-slash">/</span>

      <div class="env-title-cluster">
        <h2 class="environment-heading">{environment.name}</h2>
        {#if environment.isProduction}
          <span class="prod-tag" title="Entorno de producción">
            <ShieldAlert size={11} strokeWidth={2.4} />
            <span>{t("secrets.production")}</span>
          </span>
        {/if}
        {#if isActive}
          <span class="shell-active-tag">
            <span class="active-dot-pulsing"></span>
            <span>{t("secrets.inWindows")}</span>
          </span>
        {/if}
      </div>
    </div>

    <!-- Actions Toolbar -->
    <div class="header-action-group">
      {#if project.environments.length > 1}
        <Tooltip.Root>
          <Tooltip.Trigger
            class="btn-subtle"
            type="button"
            onclick={onOpenCompare}
            aria-label={t("secrets.compare")}
          >
            <GitCompare size={13.5} />
            <span>{t("secrets.compare")}</span>
          </Tooltip.Trigger>
          <Tooltip.Portal>
            <Tooltip.Content class="tooltip-content" sideOffset={6}>
              {t("secrets.compareEnvs")}
            </Tooltip.Content>
          </Tooltip.Portal>
        </Tooltip.Root>
      {/if}

      <Tooltip.Root>
        <Tooltip.Trigger
          class="btn-subtle"
          type="button"
          disabled={busy}
          onclick={onOpenTerminal}
          aria-label={t("secrets.terminal")}
        >
          <Terminal size={13.5} />
          <span>{t("secrets.terminal")}</span>
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content class="tooltip-content" sideOffset={6}>
            {t("secrets.openTerminal")}
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>

      {#if !isActive}
        <Button.Root
          class="btn-activate"
          type="button"
          disabled={busy}
          onclick={onActivateEnvironment}
        >
          <Shield size={13.5} />
          <span>{t("secrets.activate")}</span>
        </Button.Root>
      {/if}

      <DropdownMenu.Root>
        <DropdownMenu.Trigger class="btn-icon-more" type="button" aria-label={t("secrets.actions")}>
          <MoreHorizontal size={15} />
        </DropdownMenu.Trigger>
        <DropdownMenu.Portal>
          <DropdownMenu.Content class="actions-menu" sideOffset={6} align="end">
            <DropdownMenu.Item class="menu-item" {disabled} onclick={onOpenSettings}>
              <Pencil size={13.5} />
              <span>{t("secrets.environmentSettings")}</span>
            </DropdownMenu.Item>
            <DropdownMenu.Item class="menu-item" {disabled} onclick={onOpenGenerator}>
              <KeyRound size={13.5} />
              <span>{t("secrets.generateSecret")}</span>
            </DropdownMenu.Item>
            <DropdownMenu.Item class="menu-item" {disabled} onclick={onOpenImport}>
              <FileUp size={13.5} />
              <span>{t("secrets.importEnv")}</span>
            </DropdownMenu.Item>
            <DropdownMenu.Item class="menu-item" disabled={busy} onclick={onOpenClone}>
              <Copy size={13.5} />
              <span>{t("secrets.cloneEnv")}</span>
            </DropdownMenu.Item>
            {#if project.environments.length > 1}
              <DropdownMenu.Item class="menu-item" onclick={onOpenCompare}>
                <GitCompare size={13.5} />
                <span>{t("secrets.compareEnvs")}</span>
              </DropdownMenu.Item>
            {/if}
            <DropdownMenu.Separator class="menu-separator" />
            <DropdownMenu.Item class="menu-item" onclick={onCopyEnvironment}>
              <Copy size={13.5} />
              <span>{t("secrets.copyEnv")}</span>
            </DropdownMenu.Item>
            <DropdownMenu.Item class="menu-item" onclick={onDownloadEnvironment}>
              <Download size={13.5} />
              <span>{t("secrets.downloadEnv")}</span>
            </DropdownMenu.Item>
            <DropdownMenu.Separator class="menu-separator" />
            <DropdownMenu.Item
              class="menu-item destructive"
              {disabled}
              onclick={onRequestDeleteEnvironment}
            >
              <Trash2 size={13.5} />
              <span>{t("secrets.deleteEnvironment")}</span>
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Portal>
      </DropdownMenu.Root>
    </div>
  </div>

  <!-- Segmented environment selector tabs -->
  <nav class="environment-tabs-nav" aria-label={t("secrets.environments")}>
    <div class="tabs-track">
      {#each project.environments as item (item.id)}
        {@const isSelected = item.id === environment.id}
        {@const isEnvActive = item.id === activeEnvironmentId}
        <Button.Root
          class={`env-chip${isSelected ? " selected" : ""}`}
          type="button"
          onclick={() => onSelectEnvironment(item.id)}
        >
          <span class={`chip-dot${isEnvActive ? " active" : ""}`}></span>
          <span class="chip-name">{item.name}</span>
          {#if item.isProduction}
            <span class="chip-prod-pill">PROD</span>
          {/if}
          <span class="chip-count">{item.secrets.length}</span>
        </Button.Root>
      {/each}
    </div>
  </nav>
</header>

<style>
  .workspace-header {
    border-bottom: 1px solid var(--color-boulder-150, #ececec);
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding-bottom: 14px;
    margin-bottom: 12px;
  }

  .breadcrumb-toolbar-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }

  .breadcrumb-group {
    align-items: center;
    display: flex;
    gap: 8px;
    min-width: 0;
  }

  .project-tag {
    align-items: center;
    background: var(--color-boulder-100);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-700);
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 6px;
    padding: 3px 6px 3px 8px;
  }

  :global(.project-icon) {
    color: var(--color-east-bay-700);
  }

  .project-name {
    max-width: 130px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.project-menu-btn) {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    height: 18px;
    justify-content: center;
    padding: 0;
    width: 18px;
  }

  :global(.project-menu-btn:hover) {
    background: var(--color-boulder-200);
    color: var(--color-boulder-900);
  }

  .breadcrumb-slash {
    color: var(--color-boulder-300);
    font-size: 13px;
    font-weight: 400;
    user-select: none;
  }

  .env-title-cluster {
    align-items: center;
    display: flex;
    gap: 8px;
    min-width: 0;
  }

  .environment-heading {
    color: var(--color-boulder-950);
    font-size: 17px;
    font-weight: 750;
    letter-spacing: -0.015em;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .prod-tag {
    align-items: center;
    background: #fffbeb;
    border: 1px solid #fef3c7;
    border-radius: 99px;
    color: #92400e;
    display: inline-flex;
    font-size: 10px;
    font-weight: 700;
    gap: 4px;
    letter-spacing: 0.03em;
    padding: 2px 7px;
    text-transform: uppercase;
  }

  .shell-active-tag {
    align-items: center;
    background: #f0fdf4;
    border: 1px solid #dcfce7;
    border-radius: 99px;
    color: #15803d;
    display: inline-flex;
    font-size: 10.5px;
    font-weight: 650;
    gap: 5px;
    padding: 2px 8px;
  }

  .active-dot-pulsing {
    background: #16a34a;
    border-radius: 50%;
    box-shadow: 0 0 0 2px rgb(22 163 74 / 20%);
    height: 6px;
    width: 6px;
  }

  .header-action-group {
    align-items: center;
    display: flex;
    gap: 6px;
  }

  :global(.btn-subtle) {
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
    height: 31px;
    padding: 0 10px;
    transition: all 0.12s ease;
  }

  :global(.btn-subtle:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-400);
    color: var(--color-boulder-950);
  }

  :global(.btn-activate) {
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
    gap: 6px;
    height: 31px;
    padding: 0 12px;
    transition: opacity 0.15s ease;
  }

  :global(.btn-activate:hover) {
    opacity: 0.92;
  }

  :global(.btn-icon-more) {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-250, #d6d6d6);
    border-radius: 6px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: inline-flex;
    height: 31px;
    justify-content: center;
    width: 31px;
    transition: all 0.12s ease;
  }

  :global(.btn-icon-more:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-400);
    color: var(--color-boulder-950);
  }

  .environment-tabs-nav {
    display: flex;
    overflow-x: auto;
    padding-bottom: 2px;
  }

  .tabs-track {
    background: var(--color-boulder-100);
    border-radius: 7px;
    display: inline-flex;
    gap: 2px;
    padding: 3px;
  }

  :global(.env-chip) {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 5px;
    color: var(--color-boulder-600);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 11.5px;
    font-weight: 550;
    gap: 6px;
    height: 27px;
    padding: 0 9px;
    transition: all 0.12s ease;
    white-space: nowrap;
  }

  :global(.env-chip:hover) {
    background: rgb(255 255 255 / 60%);
    color: var(--color-boulder-900);
  }

  :global(.env-chip.selected) {
    background: #fff;
    box-shadow: 0 1px 3px rgb(0 0 0 / 8%);
    color: var(--color-boulder-950);
    font-weight: 650;
  }

  .chip-dot {
    background: var(--color-boulder-350, #bebebe);
    border-radius: 50%;
    flex-shrink: 0;
    height: 6px;
    width: 6px;
  }

  .chip-dot.active {
    background: #16a34a;
    box-shadow: 0 0 0 2px rgb(22 163 74 / 25%);
  }

  .chip-name {
    max-width: 160px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .chip-prod-pill {
    background: #fef3c7;
    border-radius: 3px;
    color: #92400e;
    font-size: 8.5px;
    font-weight: 750;
    letter-spacing: 0.04em;
    padding: 1px 3px;
  }

  .chip-count {
    color: var(--color-boulder-400);
    font-size: 10.5px;
  }

  :global(.env-chip.selected .chip-count) {
    color: var(--color-boulder-600);
  }

  :global(.actions-menu) {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 7px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    min-width: 195px;
    padding: 4px;
    z-index: 50;
  }

  :global(.menu-item) {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 5px;
    color: var(--color-boulder-750, #383838);
    cursor: pointer;
    display: flex;
    font: inherit;
    font-size: 12px;
    gap: 8px;
    padding: 7px 9px;
    width: 100%;
    transition: background 0.1s ease;
  }

  :global(.menu-item:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  :global(.menu-item.destructive) {
    color: #b91c1c;
  }

  :global(.menu-item.destructive:hover) {
    background: #fef2f2;
    color: #dc2626;
  }

  :global(.menu-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }

  :global(.tooltip-content) {
    background: var(--color-boulder-950);
    border-radius: 5px;
    color: #fff;
    font-size: 11px;
    font-weight: 550;
    padding: 4px 8px;
    z-index: 150;
  }
</style>
