<script lang="ts">
  import { t } from "#lib/i18n";
  import { Menu } from "@ark-ui/svelte/menu";
  import {
    Copy,
    Download,
    FileUp,
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
      <span class="max-w-40 truncate text-sm font-semibold text-boulder-500">{project.name}</span>
      <span class="text-sm font-medium text-boulder-300">/</span>
      <div class="flex min-w-0 items-center gap-2">
        <h2 id="environment-title" class="truncate text-sm font-semibold text-boulder-950">
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
</header>
