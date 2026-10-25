<script lang="ts" module>
  export type EnvironmentAction =
    | "activate"
    | "deactivate"
    | "settings"
    | "generate"
    | "import"
    | "clone"
    | "compare"
    | "terminal"
    | "copy"
    | "download"
    | "delete";
</script>

<script lang="ts">
  import { t } from "#lib/i18n";
  import { Menu } from "@ark-ui/svelte/menu";
  import { Portal } from "@ark-ui/svelte/portal";
  import {
    Copy,
    Download,
    FileUp,
    GitCompare,
    KeyRound,
    MoreHorizontal,
    Pencil,
    Power,
    PowerOff,
    Terminal,
    Trash2,
  } from "@lucide/svelte";

  type Props = {
    disabled?: boolean;
    busy?: boolean;
    activationBusy?: boolean;
    isActive?: boolean;
    canCompare: boolean;
    visible?: boolean;
    onAction: (action: EnvironmentAction) => void;
  };

  let {
    disabled = false,
    busy = false,
    activationBusy = false,
    isActive = false,
    canCompare,
    visible = false,
    onAction,
  }: Props = $props();

  const menuContentClass =
    "z-100 min-w-52 rounded-md border border-boulder-200 bg-white p-1.5 shadow-lg outline-none";
  const menuItemClass =
    "flex h-8 cursor-pointer items-center gap-2 rounded px-2 text-xs font-medium text-boulder-700 outline-none transition-colors data-[highlighted]:bg-boulder-100 data-[highlighted]:text-boulder-950 data-[disabled]:cursor-not-allowed data-[disabled]:opacity-40";
</script>

<Menu.Root positioning={{ placement: "bottom-end", gutter: 6 }}>
  <Menu.Trigger
    class={`inline-flex size-6 cursor-pointer items-center justify-center rounded text-boulder-500 transition hover:bg-white hover:text-boulder-950 focus-visible:opacity-100 focus-visible:outline-2 focus-visible:outline-offset-1 focus-visible:outline-east-bay-500 data-[state=open]:bg-white data-[state=open]:opacity-100 data-[state=open]:text-boulder-950 ${visible ? "opacity-100" : "opacity-0 group-hover:opacity-100"}`}
    aria-label={t("secrets.actions")}><MoreHorizontal size={14} /></Menu.Trigger
  >
  <Portal>
    <Menu.Positioner
      ><Menu.Content class={menuContentClass}>
        {#if isActive}
          <Menu.Item
            value="deactivate"
            class={menuItemClass}
            disabled={activationBusy}
            onSelect={() => onAction("deactivate")}
            ><PowerOff size={14} /><span>{t("secrets.deactivate")}</span></Menu.Item
          >
        {:else}
          <Menu.Item
            value="activate"
            class={`${menuItemClass} text-emerald-700 data-highlighted:bg-emerald-50 data-highlighted:text-emerald-700`}
            disabled={busy}
            onSelect={() => onAction("activate")}
            ><Power size={14} /><span>{t("secrets.activateProfile")}</span></Menu.Item
          >
        {/if}
        <Menu.Separator class="mx-2 my-0.5 h-px border-0 bg-boulder-100" />
        <Menu.Item
          value="settings"
          class={menuItemClass}
          {disabled}
          onSelect={() => onAction("settings")}
          ><Pencil size={14} /><span>{t("secrets.environmentSettings")}</span></Menu.Item
        >
        <Menu.Item
          value="generate"
          class={menuItemClass}
          {disabled}
          onSelect={() => onAction("generate")}
          ><KeyRound size={14} /><span>{t("secrets.generateSecret")}</span></Menu.Item
        >
        <Menu.Item
          value="import"
          class={menuItemClass}
          {disabled}
          onSelect={() => onAction("import")}
          ><FileUp size={14} /><span>{t("secrets.importEnv")}</span></Menu.Item
        >
        <Menu.Item
          value="clone"
          class={menuItemClass}
          disabled={busy}
          onSelect={() => onAction("clone")}
          ><Copy size={14} /><span>{t("secrets.cloneEnv")}</span></Menu.Item
        >
        {#if canCompare}<Menu.Item
            value="compare"
            class={menuItemClass}
            onSelect={() => onAction("compare")}
            ><GitCompare size={14} /><span>{t("secrets.compareEnvs")}</span></Menu.Item
          >{/if}
        <Menu.Item
          value="terminal"
          class={menuItemClass}
          disabled={busy}
          onSelect={() => onAction("terminal")}
          ><Terminal size={14} /><span>{t("secrets.openTerminal")}</span></Menu.Item
        >
        <Menu.Separator class="mx-2 my-0.5 h-px border-0 bg-boulder-100" />
        <Menu.Item value="copy" class={menuItemClass} onSelect={() => onAction("copy")}
          ><Copy size={14} /><span>{t("secrets.copyEnv")}</span></Menu.Item
        >
        <Menu.Item value="download" class={menuItemClass} onSelect={() => onAction("download")}
          ><Download size={14} /><span>{t("secrets.downloadEnv")}</span></Menu.Item
        >
        <Menu.Separator class="mx-2 my-0.5 h-px border-0 bg-boulder-100" />
        <Menu.Item
          value="delete"
          class={`${menuItemClass} text-red-700 data-highlighted:bg-red-50 data-highlighted:text-red-700`}
          {disabled}
          onSelect={() => onAction("delete")}
          ><Trash2 size={14} /><span>{t("secrets.deleteEnvironment")}</span></Menu.Item
        >
      </Menu.Content></Menu.Positioner
    >
  </Portal>
</Menu.Root>
