<script lang="ts">
  import { onMount } from "svelte";
  import { FolderOpen, MonitorCog, RotateCcw, Save, Settings2 } from "@lucide/svelte";
  import { Switch } from "@ark-ui/svelte/switch";
  import { Select, createListCollection } from "@ark-ui/svelte/select";
  import { Field } from "@ark-ui/svelte/field";
  import { Fieldset } from "@ark-ui/svelte/fieldset";
  import { i18n, t } from "#lib/i18n";
  import { createSettingsStore, type ProxySettings } from "../application/settingsStore.svelte";
  import { openWorkspaceDirectory } from "../infrastructure/settingsRepository";
  import ProxySettingsSection from "./ProxySettingsSection.svelte";

  const settingsStore = createSettingsStore();
  const locales = createListCollection({
    items: [
      { label: t("settings.spanish"), value: "es" },
      { label: t("settings.english"), value: "en" },
    ],
  });
  let settings = $derived(settingsStore.settings);
  let workspacePath = $derived(settingsStore.workspacePath);
  let saved = $derived(settingsStore.saved);
  let error = $derived(settingsStore.error);
  onMount(() => settingsStore.load());
  function saveSettings() {
    const current = settingsStore.settings;
    const port = Number(current.proxyPort);
    if (current.proxyEnabled) {
      if (!current.proxyHost.trim()) {
        settingsStore.setError(t("settings.errors.proxyHostRequired"));
        return;
      }
      if (!Number.isInteger(port) || port < 1 || port > 65535) {
        settingsStore.setError(t("settings.errors.proxyPortInvalid"));
        return;
      }
    }
    const proxy: ProxySettings = {
      enabled: current.proxyEnabled,
      host: current.proxyHost.trim(),
      port,
      username: current.proxyUsername.trim() || null,
      password: current.proxyPassword || null,
    };
    void settingsStore
      .save(proxy)
      .catch((caught) =>
        settingsStore.setError(caught instanceof Error ? caught.message : String(caught)),
      );
  }
  function resetSettings() {
    settingsStore.reset();
    saveSettings();
  }
  function changeLocale(details: { value: string[] }) {
    const nextLocale = details.value[0];
    if (nextLocale === "es" || nextLocale === "en") settingsStore.setLocale(nextLocale);
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
      <p class="mb-2 text-[11px] font-bold uppercase tracking-[0.06em] text-east-bay-700">
        {t("settings.eyebrow")}
      </p>
      <h1 id="settings-title" class="text-3xl leading-none font-semibold text-boulder-950">
        {t("settings.title")}
      </h1>
      <p class="mt-2.5 text-[13px] leading-relaxed text-boulder-600">{t("settings.description")}</p>
    </div>
    <div class="flex gap-2 max-md:w-full">
      <button
        class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md bg-boulder-100 px-3 text-xs font-bold text-boulder-800 hover:bg-boulder-200 max-md:flex-1 max-md:justify-center"
        type="button"
        onclick={resetSettings}><RotateCcw size={15} /><span>{t("settings.reset")}</span></button
      ><button
        class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-bold text-white hover:bg-east-bay-950 max-md:flex-1 max-md:justify-center"
        type="button"
        onclick={saveSettings}
        ><Save size={15} /><span>{saved ? t("settings.saved") : t("settings.save")}</span></button
      >
    </div>
  </header>
  {#if error}<p
      class="mt-5 rounded-md border border-red-200 bg-red-50 px-3 py-2.5 text-xs text-red-700"
      role="alert"
    >
      {error}
    </p>{/if}
  <div
    class="mt-8 grid grid-cols-[minmax(0,1.35fr)_minmax(280px,1fr)] gap-4 max-md:mt-6 max-md:grid-cols-1"
  >
    <section
      class="row-span-2 min-w-0 rounded-lg border border-boulder-200 bg-white max-md:row-auto"
      aria-labelledby="behavior-title"
    >
      <div class="flex items-center gap-3 border-b border-boulder-100 p-4.5">
        <span
          class="flex size-8.5 items-center justify-center rounded-md border border-east-bay-100 bg-east-bay-50 text-east-bay-700"
          ><MonitorCog size={18} /></span
        >
        <div>
          <h2 id="behavior-title" class="text-sm font-semibold text-boulder-900">
            {t("settings.behavior")}
          </h2>
          <p class="mt-1 text-xs leading-relaxed text-boulder-500">
            {t("settings.behaviorDescription")}
          </p>
        </div>
      </div>
      <Fieldset.Root class="px-4.5 py-1">
        <Fieldset.Legend class="sr-only">{t("settings.behavior")}</Fieldset.Legend>
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 border-b border-boulder-100 py-3.5"
          bind:checked={settings.launchAtStartup}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-800"
              >{t("settings.launchAtStartup")}</strong
            ><small class="text-[11px] leading-snug text-boulder-500"
              >{t("settings.launchAtStartupDescription")}</small
            ></span
          ><Switch.Control
            class="relative h-5.5 w-9.5 shrink-0 rounded-full bg-boulder-300 transition data-[state=checked]:bg-east-bay-700 data-[focus]:outline-2 data-[focus]:outline-east-bay-400 after:absolute after:top-0.5 after:left-0.5 after:size-4.5 after:rounded-full after:bg-white after:transition-transform data-[state=checked]:after:translate-x-4"
            ><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 border-b border-boulder-100 py-3.5"
          bind:checked={settings.minimizeToTray}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-800"
              >{t("settings.minimizeToTray")}</strong
            ><small class="text-[11px] leading-snug text-boulder-500"
              >{t("settings.minimizeToTrayDescription")}</small
            ></span
          ><Switch.Control
            class="relative h-5.5 w-9.5 shrink-0 rounded-full bg-boulder-300 transition data-[state=checked]:bg-east-bay-700 data-[focus]:outline-2 data-[focus]:outline-east-bay-400 after:absolute after:top-0.5 after:left-0.5 after:size-4.5 after:rounded-full after:bg-white after:transition-transform data-[state=checked]:after:translate-x-4"
            ><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 border-b border-boulder-100 py-3.5"
          bind:checked={settings.closeToTray}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-800"
              >{t("settings.closeToTray")}</strong
            ><small class="text-[11px] leading-snug text-boulder-500"
              >{t("settings.closeToTrayDescription")}</small
            ></span
          ><Switch.Control
            class="relative h-5.5 w-9.5 shrink-0 rounded-full bg-boulder-300 transition data-[state=checked]:bg-east-bay-700 data-[focus]:outline-2 data-[focus]:outline-east-bay-400 after:absolute after:top-0.5 after:left-0.5 after:size-4.5 after:rounded-full after:bg-white after:transition-transform data-[state=checked]:after:translate-x-4"
            ><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
        <Switch.Root
          class="flex cursor-pointer items-center justify-between gap-4 py-3.5"
          bind:checked={settings.openLastSection}
          ><span class="flex flex-col gap-1"
            ><strong class="text-[13px] font-semibold text-boulder-800"
              >{t("settings.restoreLastSection")}</strong
            ><small class="text-[11px] leading-snug text-boulder-500"
              >{t("settings.restoreLastSectionDescription")}</small
            ></span
          ><Switch.Control
            class="relative h-5.5 w-9.5 shrink-0 rounded-full bg-boulder-300 transition data-[state=checked]:bg-east-bay-700 data-[focus]:outline-2 data-[focus]:outline-east-bay-400 after:absolute after:top-0.5 after:left-0.5 after:size-4.5 after:rounded-full after:bg-white after:transition-transform data-[state=checked]:after:translate-x-4"
            ><Switch.Thumb class="sr-only" /></Switch.Control
          ><Switch.HiddenInput /></Switch.Root
        >
      </Fieldset.Root>
    </section>
    <section
      class="min-w-0 rounded-lg border border-boulder-200 bg-white"
      aria-labelledby="language-title"
    >
      <div class="flex items-center gap-3 border-b border-boulder-100 p-4.5">
        <span
          class="flex size-8.5 items-center justify-center rounded-md border border-east-bay-100 bg-east-bay-50 text-east-bay-700"
          ><Settings2 size={18} /></span
        >
        <div>
          <h2 id="language-title" class="text-sm font-semibold text-boulder-900">
            {t("settings.language")}
          </h2>
          <p class="mt-1 text-xs leading-relaxed text-boulder-500">
            {t("settings.languageDescription")}
          </p>
        </div>
      </div>
      <div class="p-4.5">
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
            <Select.Trigger
              class="flex h-9 w-full cursor-pointer items-center justify-between rounded-md border border-boulder-200 bg-white px-2.5 text-xs text-boulder-800 outline-none focus-visible:border-east-bay-400 focus-visible:ring-2 focus-visible:ring-east-bay-100"
            >
              <Select.ValueText />
              <Select.Indicator aria-hidden="true">⌄</Select.Indicator>
            </Select.Trigger>
          </Select.Control>
          <Select.Positioner>
            <Select.Content
              class="z-50 min-w-[var(--reference-width)] rounded-md border border-boulder-200 bg-white p-1 shadow-lg outline-none"
            >
              {#each locales.items as item (item.value)}
                <Select.Item
                  {item}
                  class="flex h-8 cursor-pointer items-center justify-between rounded px-2 text-xs text-boulder-800 outline-none data-[highlighted]:bg-boulder-100 data-[state=checked]:font-semibold"
                >
                  <Select.ItemText>{item.label}</Select.ItemText>
                  <Select.ItemIndicator aria-hidden="true">✓</Select.ItemIndicator>
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Positioner>
          <Select.HiddenSelect />
        </Select.Root>
      </div>
    </section>
    <section
      class="min-w-0 rounded-lg border border-boulder-200 bg-white"
      aria-labelledby="workspace-title"
    >
      <div class="flex items-center gap-3 border-b border-boulder-100 p-4.5">
        <span
          class="flex size-8.5 items-center justify-center rounded-md border border-east-bay-100 bg-east-bay-50 text-east-bay-700"
          ><Settings2 size={18} /></span
        >
        <div>
          <h2 id="workspace-title" class="text-sm font-semibold text-boulder-900">
            {t("settings.workspace")}
          </h2>
          <p class="mt-1 text-xs leading-relaxed text-boulder-500">
            {t("settings.workspaceDescription")}
          </p>
        </div>
      </div>
      <div class="p-4.5">
        <div class="flex items-center gap-2">
          <Field.Root class="flex min-w-0 flex-1 flex-col gap-1.5">
            <Field.Label class="text-xs font-semibold text-boulder-700"
              >{t("settings.workspaceLabel")}</Field.Label
            >
            <Field.Input
              id="workspace-path"
              class="h-9 w-full rounded-md border border-boulder-200 bg-boulder-50 px-2.5 text-xs text-boulder-700"
              value={workspacePath}
              readonly
            />
          </Field.Root>
          <button
            class="inline-flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-md border border-boulder-300 bg-white text-boulder-700 hover:bg-boulder-50"
            type="button"
            onclick={openWorkspace}
            title={t("settings.openWorkspace")}
            aria-label={t("settings.openWorkspace")}><FolderOpen size={16} /></button
          >
        </div>
        <p class="mt-1 text-xs leading-relaxed text-boulder-500">{t("settings.workspaceHint")}</p>
      </div>
    </section>
    <ProxySettingsSection
      bind:enabled={settings.proxyEnabled}
      bind:host={settings.proxyHost}
      bind:port={settings.proxyPort}
      bind:username={settings.proxyUsername}
      bind:password={settings.proxyPassword}
    />
  </div>
</main>
