<script lang="ts">
  import { onMount } from "svelte";
  import { Check, ChevronDown, Download, FolderOpen, RotateCcw, Upload } from "@lucide/svelte";
  import { Switch } from "@ark-ui/svelte/switch";
  import { Select, createListCollection } from "@ark-ui/svelte/select";
  import { Field } from "@ark-ui/svelte/field";
  import { Fieldset } from "@ark-ui/svelte/fieldset";
  import { i18n, t } from "#lib/i18n";
  import { createSettingsStore, type ProxySettings } from "../application/settingsStore.svelte";
  import {
    listTerminals,
    openWorkspaceDirectory,
    type DetectedTerminal,
  } from "../infrastructure/settingsRepository";
  import { createSecretsStore } from "#lib/features/secrets/application/secretsStore.svelte";
  import SecretsBackupDialog from "#lib/features/secrets/components/SecretsBackupDialog.svelte";
  import ProxySettingsSection from "./ProxySettingsSection.svelte";

  const settingsStore = createSettingsStore();
  const secretsStore = createSecretsStore();
  const sectionClass =
    "grid grid-cols-[minmax(220px,1fr)_minmax(0,1.7fr)] gap-x-12 gap-y-4 py-7 max-md:grid-cols-1";
  const headingClass = "text-sm font-semibold text-boulder-950";
  const descriptionClass = "mt-1 max-w-xs text-xs leading-relaxed text-boulder-600";
  const switchClass =
    "relative h-5.5 w-9.5 shrink-0 rounded-full bg-boulder-300 transition-colors data-[state=checked]:bg-east-bay-700 data-focus:outline-2 data-focus:outline-offset-2 data-focus:outline-east-bay-400 after:absolute after:top-0.5 after:left-0.5 after:size-4.5 after:rounded-full after:bg-white after:transition-transform data-[state=checked]:after:translate-x-4";
  const selectTriggerClass =
    "flex h-9 w-full cursor-pointer items-center justify-between rounded-md border border-boulder-300 bg-white px-2.5 text-xs text-boulder-800 outline-none transition-colors hover:border-boulder-400 focus-visible:border-east-bay-500 focus-visible:ring-2 focus-visible:ring-east-bay-500/15";
  const selectContentClass =
    "z-50 min-w-[var(--reference-width)] rounded-md border border-boulder-200 bg-white p-1 shadow-lg outline-none";
  const selectItemClass =
    "flex h-8 cursor-pointer items-center justify-between rounded px-2 text-xs text-boulder-800 outline-none data-highlighted:bg-boulder-100 data-[state=checked]:font-semibold";
  const inputClass =
    "h-9 w-full rounded-md border border-boulder-300 bg-white px-2.5 text-xs text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-2 focus:ring-east-bay-500/15";
  const buttonClass =
    "inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md border border-boulder-300 bg-white px-3 text-xs font-semibold text-boulder-800 transition-colors hover:bg-boulder-100 disabled:cursor-not-allowed disabled:opacity-50";
  let backupOpen = $state(false);
  let backupMode = $state<"export" | "restore">("export");
  const locales = createListCollection({
    items: [
      { label: t("settings.spanish"), value: "es" },
      { label: t("settings.english"), value: "en" },
    ],
  });
  let detectedTerminals = $state<DetectedTerminal[]>([]);
  const terminals = $derived(
    createListCollection({
      items: [
        ...detectedTerminals.map((terminal) => ({ label: terminal.label, value: terminal.id })),
        { label: t("settings.customTerminal"), value: "custom" },
      ],
    }),
  );
  let settings = $derived(settingsStore.settings);
  let workspacePath = $derived(settingsStore.workspacePath);
  let saved = $derived(settingsStore.saved);
  let error = $derived(settingsStore.error);
  let ready = $state(false);
  let lastSaved = "";
  onMount(() => {
    void settingsStore.load();
    void listTerminals().then((found) => (detectedTerminals = found));
    lastSaved = JSON.stringify(settingsStore.settings);
    ready = true;
    secretsStore.start();
    void secretsStore.load();
    return () => secretsStore.dispose();
  });
  async function openBackup(mode: "export" | "restore") {
    if (!(await secretsStore.flush())) return;
    backupMode = mode;
    backupOpen = true;
  }
  function saveSettings() {
    const current = settingsStore.settings;
    const port = Number(current.proxyPort);
    let proxy: ProxySettings | null = null;
    if (current.proxyEnabled && !current.proxyHost.trim()) {
      settingsStore.setError(t("settings.errors.proxyHostRequired"));
    } else if (current.proxyEnabled && (!Number.isInteger(port) || port < 1 || port > 65535)) {
      settingsStore.setError(t("settings.errors.proxyPortInvalid"));
    } else {
      proxy = {
        enabled: current.proxyEnabled,
        host: current.proxyHost.trim(),
        port,
        username: current.proxyUsername.trim() || null,
        password: current.proxyPassword || null,
      };
    }
    void settingsStore
      .save(proxy)
      .catch((caught) =>
        settingsStore.setError(caught instanceof Error ? caught.message : String(caught)),
      );
  }
  $effect(() => {
    const snapshot = JSON.stringify($state.snapshot(settingsStore.settings));
    if (!ready || snapshot === lastSaved) return;
    const timer = window.setTimeout(() => {
      lastSaved = snapshot;
      saveSettings();
    }, 500);
    return () => window.clearTimeout(timer);
  });
  function resetSettings() {
    settingsStore.reset();
  }
  function changeLocale(details: { value: string[] }) {
    const nextLocale = details.value[0];
    if (nextLocale === "es" || nextLocale === "en") settingsStore.setLocale(nextLocale);
  }
  function changeTerminal(details: { value: string[] }) {
    const terminal = details.value[0];
    if (terminal) settings.terminalType = terminal;
  }
  async function openWorkspace() {
    await openWorkspaceDirectory(settingsStore.workspacePath);
  }
</script>

<svelte:head
  ><title>Harbor | {t("settings.title")}</title><meta
    name="description"
    content={t("settings.description")}
  /></svelte:head
>

<main
  class="mx-auto flex w-full max-w-270 flex-1 flex-col p-8 max-md:px-4 max-md:py-6"
  aria-labelledby="settings-title"
>
  <header class="flex items-center justify-between gap-4 max-md:flex-col max-md:items-start">
    <div>
      <h1
        id="settings-title"
        class="text-3xl leading-tight font-semibold tracking-[-0.02em] text-boulder-950"
      >
        {t("settings.title")}
      </h1>
      <p class="mt-2.5 text-[13px] leading-relaxed text-boulder-600">{t("settings.description")}</p>
    </div>
    <p
      class="inline-flex h-9 items-center gap-1.5 text-xs font-semibold text-emerald-800 transition-opacity {saved
        ? 'opacity-100'
        : 'opacity-0'}"
      role="status"
    >
      <Check size={15} /><span>{t("settings.saved")}</span>
    </p>
  </header>
  {#if error}<p
      class="mt-5 rounded-md border border-red-200 bg-red-50 px-3 py-2.5 text-xs text-red-700"
      role="alert"
    >
      {error}
    </p>{/if}
  <div
    class="mt-8 flex flex-col divide-y divide-boulder-200 border-y border-boulder-200 max-md:mt-6"
  >
    <section class={sectionClass} aria-labelledby="behavior-title">
      <div>
        <h2 id="behavior-title" class={headingClass}>{t("settings.behavior")}</h2>
        <p class={descriptionClass}>{t("settings.behaviorDescription")}</p>
      </div>
      <Fieldset.Root class="min-w-0">
        <Fieldset.Legend class="sr-only">{t("settings.behavior")}</Fieldset.Legend>
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 border-b border-boulder-100 pt-0 pb-4"
          bind:checked={settings.launchAtStartup}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-900"
              >{t("settings.launchAtStartup")}</strong
            ><small class="text-xs leading-snug text-boulder-600"
              >{t("settings.launchAtStartupDescription")}</small
            ></span
          ><Switch.Control class={switchClass}><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 border-b border-boulder-100 py-4"
          bind:checked={settings.minimizeToTray}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-900"
              >{t("settings.minimizeToTray")}</strong
            ><small class="text-xs leading-snug text-boulder-600"
              >{t("settings.minimizeToTrayDescription")}</small
            ></span
          ><Switch.Control class={switchClass}><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 border-b border-boulder-100 py-4"
          bind:checked={settings.closeToTray}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-900"
              >{t("settings.closeToTray")}</strong
            ><small class="text-xs leading-snug text-boulder-600"
              >{t("settings.closeToTrayDescription")}</small
            ></span
          ><Switch.Control class={switchClass}><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 pt-4 pb-0"
          bind:checked={settings.openLastSection}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-900"
              >{t("settings.restoreLastSection")}</strong
            ><small class="text-xs leading-snug text-boulder-600"
              >{t("settings.restoreLastSectionDescription")}</small
            ></span
          ><Switch.Control class={switchClass}><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
      </Fieldset.Root>
    </section>
    <section class={sectionClass} aria-labelledby="language-title">
      <div>
        <h2 id="language-title" class={headingClass}>{t("settings.language")}</h2>
        <p class={descriptionClass}>{t("settings.languageDescription")}</p>
      </div>
      <div class="max-w-sm">
        <Select.Root
          collection={locales}
          value={[i18n.locale]}
          onValueChange={changeLocale}
          positioning={{ sameWidth: true }}
        >
          <Select.Label class="mb-1.5 block text-xs font-semibold text-boulder-700"
            >{t("settings.language")}</Select.Label
          >
          <Select.Control class="flex w-full">
            <Select.Trigger class={selectTriggerClass}>
              <Select.ValueText />
              <Select.Indicator><ChevronDown size={14} aria-hidden="true" /></Select.Indicator>
            </Select.Trigger>
          </Select.Control>
          <Select.Positioner>
            <Select.Content class={selectContentClass}>
              {#each locales.items as item (item.value)}
                <Select.Item {item} class={selectItemClass}>
                  <Select.ItemText>{item.label}</Select.ItemText>
                  <Select.ItemIndicator><Check size={14} aria-hidden="true" /></Select.ItemIndicator>
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Positioner>
          <Select.HiddenSelect />
        </Select.Root>
      </div>
    </section>
    <section class={sectionClass} aria-labelledby="workspace-title">
      <div>
        <h2 id="workspace-title" class={headingClass}>{t("settings.workspace")}</h2>
        <p class={descriptionClass}>{t("settings.workspaceDescription")}</p>
      </div>
      <div class="min-w-0">
        <div class="flex items-end gap-2">
          <Field.Root class="flex min-w-0 flex-1 flex-col gap-1.5">
            <Field.Label class="text-xs font-semibold text-boulder-700"
              >{t("settings.workspaceLabel")}</Field.Label
            >
            <Field.Input
              id="workspace-path"
              class="h-9 w-full truncate rounded-md border border-boulder-200 bg-boulder-50 px-2.5 font-mono text-xs text-boulder-700"
              value={workspacePath}
              readonly
            />
          </Field.Root>
          <button
            class="inline-flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-md border border-boulder-300 bg-white text-boulder-700 transition-colors hover:bg-boulder-100"
            type="button"
            onclick={openWorkspace}
            title={t("settings.openWorkspace")}
            aria-label={t("settings.openWorkspace")}><FolderOpen size={16} /></button
          >
        </div>
        <p class="mt-2 text-xs leading-relaxed text-boulder-600">{t("settings.workspaceHint")}</p>
      </div>
    </section>
    <ProxySettingsSection
      bind:enabled={settings.proxyEnabled}
      bind:host={settings.proxyHost}
      bind:port={settings.proxyPort}
      bind:username={settings.proxyUsername}
      bind:password={settings.proxyPassword}
    />
    <section class={sectionClass} aria-labelledby="shell-title">
      <div>
        <h2 id="shell-title" class={headingClass}>{t("settings.shell")}</h2>
        <p class={descriptionClass}>{t("settings.shellDescription")}</p>
      </div>
      <div class="grid max-w-sm gap-4">
        <Field.Root class="flex flex-col gap-1.5">
          <Field.Label class="text-xs font-semibold text-boulder-700">
            {t("settings.terminal")}
          </Field.Label>
          <Select.Root
            collection={terminals}
            value={[settings.terminalType]}
            onValueChange={changeTerminal}
            positioning={{ sameWidth: true }}
          >
            <Select.Control class="flex w-full">
              <Select.Trigger class={selectTriggerClass}>
                <Select.ValueText />
                <Select.Indicator><ChevronDown size={14} aria-hidden="true" /></Select.Indicator>
              </Select.Trigger>
            </Select.Control>
            <Select.Positioner>
              <Select.Content class={selectContentClass}>
                {#each terminals.items as item (item.value)}
                  <Select.Item {item} class={selectItemClass}>
                    <Select.ItemText>{item.label}</Select.ItemText>
                    <Select.ItemIndicator><Check size={14} aria-hidden="true" /></Select.ItemIndicator>
                  </Select.Item>
                {/each}
              </Select.Content>
            </Select.Positioner>
            <Select.HiddenSelect />
          </Select.Root>
          {#if settings.terminalType === "custom"}
            <Field.Input
              class={inputClass}
              bind:value={settings.customTerminalPath}
              placeholder={t("settings.customTerminalPath")}
            />
          {/if}
        </Field.Root>
      </div>
    </section>
    <section class={sectionClass} aria-labelledby="secrets-data-title">
      <div>
        <h2 id="secrets-data-title" class={headingClass}>{t("settings.secretsData")}</h2>
        <p class={descriptionClass}>{t("settings.secretsDataDescription")}</p>
      </div>
      <div class="flex flex-wrap items-start gap-2">
        <button
          class={buttonClass}
          type="button"
          disabled={!secretsStore.canWrite || secretsStore.isSaving}
          onclick={() => void openBackup("export")}
          ><Download size={14} /><span>{t("secrets.exportBackupItem")}</span></button
        >
        <button
          class={buttonClass}
          type="button"
          disabled={!secretsStore.canWrite ||
            secretsStore.isSaving ||
            secretsStore.activeEnvironmentId !== null}
          onclick={() => void openBackup("restore")}
          ><Upload size={14} /><span>{t("settings.importConfiguration")}</span></button
        >
      </div>
    </section>
    <section class={sectionClass} aria-labelledby="reset-title">
      <div>
        <h2 id="reset-title" class={headingClass}>{t("settings.resetTitle")}</h2>
        <p class={descriptionClass}>{t("settings.resetDescription")}</p>
      </div>
      <div>
        <button class={buttonClass} type="button" onclick={resetSettings}><RotateCcw size={14} /><span>{t("settings.reset")}</span></button
        >
      </div>
    </section>
  </div>
</main>

<SecretsBackupDialog
  bind:open={backupOpen}
  mode={backupMode}
  configuration={secretsStore.exportConfiguration}
  onRestore={secretsStore.restoreConfiguration}
/>
