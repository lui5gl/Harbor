<script lang="ts">
  import { t } from "#lib/i18n";
  import { Menu } from "@ark-ui/svelte/menu";
  import {
    Copy,
    Download,
    FileUp,
    FolderGit2,
    GitCompare,
    KeyRound,
    MoreHorizontal,
    Pencil,
    ShieldAlert,
    Terminal,
    Trash2,
  } from "@lucide/svelte";
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
    onOpenTerminal,
  }: Props = $props();
  let isActive = $derived(environment.id === activeEnvironmentId);

  const menuContentClass =
    "z-100 min-w-52 rounded-md border border-boulder-200 bg-white p-1.5 shadow-lg outline-none";
  const menuItemClass =
    "flex h-8 cursor-pointer items-center gap-2 rounded px-2 text-xs font-medium text-boulder-700 outline-none transition-colors data-[highlighted]:bg-boulder-100 data-[highlighted]:text-boulder-950 data-[disabled]:cursor-not-allowed data-[disabled]:opacity-40";
</script>

<header class="mb-3 flex flex-col gap-3 border-b border-boulder-200 pb-3.5">
  <div class="flex flex-wrap items-center justify-between gap-3">
    <div class="flex min-w-0 items-center gap-2">
      <div
        class="inline-flex items-center gap-1.5 rounded-md border border-boulder-200 bg-boulder-100 py-1 pr-1 pl-2 text-[11.5px] font-semibold text-boulder-700"
      >
        <FolderGit2 size={15} class="shrink-0 text-east-bay-700" />
        <span class="max-w-32 truncate">{project.name}</span>
        <Menu.Root positioning={{ placement: "bottom-start", gutter: 5 }}>
          <Menu.Trigger
            class="inline-flex size-5 cursor-pointer items-center justify-center rounded text-boulder-500 transition-colors hover:bg-boulder-200 hover:text-boulder-900 data-[state=open]:bg-boulder-200 data-[state=open]:text-boulder-950"
            aria-label={t("secrets.actions")}><MoreHorizontal size={13} /></Menu.Trigger
          >
          <Menu.Positioner
            ><Menu.Content class={menuContentClass}>
              <Menu.Item value="rename-project" class={menuItemClass} onSelect={onRenameProject}
                ><Pencil size={13} /><span>{t("secrets.renameProject")}</span></Menu.Item
              >
              <Menu.Separator class="mx-2 my-0.5 h-px border-0 bg-boulder-100" />
              <Menu.Item
                value="delete-project"
                class={`${menuItemClass} text-red-700 data-highlighted:bg-red-50 data-highlighted:text-red-600`}
                onSelect={onRequestDeleteProject}
                ><Trash2 size={13} /><span>{t("secrets.deleteProject")}</span></Menu.Item
              >
            </Menu.Content></Menu.Positioner
          >
        </Menu.Root>
      </div>
      <span class="text-boulder-300">/</span>
      <div class="flex min-w-0 items-center gap-2">
        <h2 id="environment-title" class="truncate text-[17px] font-bold text-boulder-950">
          {environment.name}
        </h2>
        {#if environment.isProduction}<span
            class="inline-flex items-center gap-1 rounded-full border border-amber-200 bg-amber-50 px-2 py-0.5 text-[10px] font-bold uppercase text-amber-800"
            title="Entorno de producción"
            ><ShieldAlert size={11} strokeWidth={2.4} /><span>{t("secrets.production")}</span></span
          >{/if}
        {#if isActive}<span
            class="inline-flex items-center gap-1.5 rounded-full border border-emerald-100 bg-emerald-50 px-2 py-0.5 text-[10.5px] font-semibold text-emerald-700"
            ><span class="size-1.5 rounded-full bg-emerald-500 ring-2 ring-emerald-500/20"
            ></span><span>{t("secrets.inWindows")}</span></span
          >{/if}
      </div>
    </div>

    <div class="flex items-center gap-1.5">
      <Menu.Root positioning={{ placement: "bottom-end", gutter: 6 }}>
        <Menu.Trigger
          class="inline-flex size-8 cursor-pointer items-center justify-center rounded-md text-boulder-500 transition-colors hover:bg-boulder-100 hover:text-boulder-950 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-east-bay-500 data-[state=open]:bg-boulder-100 data-[state=open]:text-boulder-950"
          aria-label={t("secrets.actions")}><MoreHorizontal size={15} /></Menu.Trigger
        >
        <Menu.Positioner
          ><Menu.Content class={menuContentClass}>
            <Menu.Item value="settings" class={menuItemClass} {disabled} onSelect={onOpenSettings}
              ><Pencil size={14} /><span>{t("secrets.environmentSettings")}</span></Menu.Item
            >
            <Menu.Item value="generate" class={menuItemClass} {disabled} onSelect={onOpenGenerator}
              ><KeyRound size={14} /><span>{t("secrets.generateSecret")}</span></Menu.Item
            >
            <Menu.Item value="import" class={menuItemClass} {disabled} onSelect={onOpenImport}
              ><FileUp size={14} /><span>{t("secrets.importEnv")}</span></Menu.Item
            >
            <Menu.Item value="clone" class={menuItemClass} disabled={busy} onSelect={onOpenClone}
              ><Copy size={14} /><span>{t("secrets.cloneEnv")}</span></Menu.Item
            >
            {#if project.environments.length > 1}<Menu.Item
                value="compare"
                class={menuItemClass}
                onSelect={onOpenCompare}
                ><GitCompare size={14} /><span>{t("secrets.compareEnvs")}</span></Menu.Item
              >{/if}
            <Menu.Item
              value="terminal"
              class={menuItemClass}
              disabled={busy}
              onSelect={onOpenTerminal}
              ><Terminal size={14} /><span>{t("secrets.openTerminal")}</span></Menu.Item
            >
            <Menu.Separator class="mx-2 my-0.5 h-px border-0 bg-boulder-100" />
            <Menu.Item value="copy" class={menuItemClass} onSelect={onCopyEnvironment}
              ><Copy size={14} /><span>{t("secrets.copyEnv")}</span></Menu.Item
            >
            <Menu.Item value="download" class={menuItemClass} onSelect={onDownloadEnvironment}
              ><Download size={14} /><span>{t("secrets.downloadEnv")}</span></Menu.Item
            >
            <Menu.Separator class="mx-2 my-0.5 h-px border-0 bg-boulder-100" />
            <Menu.Item
              value="delete"
              class={`${menuItemClass} text-red-700 data-[highlighted]:bg-red-50 data-[highlighted]:text-red-700`}
              {disabled}
              onSelect={onRequestDeleteEnvironment}
              ><Trash2 size={14} /><span>{t("secrets.deleteEnvironment")}</span></Menu.Item
            >
          </Menu.Content></Menu.Positioner
        >
      </Menu.Root>
    </div>
  </div>

  <nav class="overflow-x-auto" aria-label={t("secrets.environments")}>
    <div class="flex min-w-max gap-1 rounded-md bg-boulder-50 p-1">
      {#each project.environments as item (item.id)}
        {@const isSelected = item.id === environment.id}
        {@const isEnvActive = item.id === activeEnvironmentId}
        <button
          class={`inline-flex h-7 cursor-pointer items-center gap-1.5 rounded px-2 text-[11.5px] font-semibold transition-colors ${isSelected ? "bg-white text-east-bay-900 shadow-sm" : "text-boulder-600 hover:bg-white hover:text-boulder-900"}`}
          type="button"
          onclick={() => onSelectEnvironment(item.id)}
        >
          <span class={`size-1.5 rounded-full ${isEnvActive ? "bg-emerald-500" : "bg-boulder-300"}`}
          ></span><span>{item.name}</span>
          {#if item.isProduction}<span
              class="rounded bg-amber-100 px-1 text-[8px] font-bold text-amber-800">PROD</span
            >{/if}
          <span class="rounded-full bg-boulder-100 px-1.5 text-[10px] text-boulder-500"
            >{item.secrets.length}</span
          >
        </button>
      {/each}
    </div>
  </nav>
</header>
