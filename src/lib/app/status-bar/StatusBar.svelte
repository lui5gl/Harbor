<script lang="ts">
  import { isTauri } from "@tauri-apps/api/core";
  import { emit, listen } from "@tauri-apps/api/event";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { Menu } from "@ark-ui/svelte/menu";
  import { Portal } from "@ark-ui/svelte/portal";
  import {
    ArrowUpRight,
    Check,
    ChevronUp,
    FileCode2,
    Folder,
    Hexagon,
    KeyRound,
    Play,
    PowerOff,
    Server,
    Settings2,
    Square,
    Terminal,
  } from "@lucide/svelte";
  import type { Project } from "#lib/features/secrets/types";
  import {
    activateSecretProfile,
    deactivateSecretEnvironment,
    loadSecretProfiles,
  } from "#lib/features/secrets/infrastructure/secretsRepository";
  import { activateCatalogVersion } from "#lib/features/services/application/runtimeCatalog";
  import { RuntimeService } from "#lib/features/services/runtimeConstants";
  import {
    getActiveRuntimes,
    getInstalledVersions,
    getPhpStatus,
    startPhp,
    stopPhp,
  } from "#lib/features/services/infrastructure/runtimesRepository";
  import { cleanVersion } from "#lib/features/services/types";
  import { t } from "#lib/i18n";

  const isNativeApp = isTauri();
  const pollIntervalMs = 5_000;

  const triggerClass =
    "inline-flex min-h-7 cursor-pointer items-center gap-1.5 whitespace-nowrap rounded border-0 bg-transparent px-1.5 text-[11.5px] text-inherit hover:bg-white/10 data-[state=open]:bg-white/10 disabled:cursor-not-allowed disabled:opacity-60";
  const contentClass =
    "z-100 max-h-96 w-72 overflow-y-auto rounded-lg border border-boulder-200 bg-white p-1.5 shadow-xl outline-none";
  const itemClass =
    "flex min-h-9 cursor-pointer items-center gap-2.5 rounded-md px-2 py-1 text-xs font-medium text-boulder-700 outline-none transition-colors data-[highlighted]:bg-boulder-100 data-[highlighted]:text-boulder-950 data-[disabled]:cursor-not-allowed data-[disabled]:opacity-40";
  const labelClass =
    "flex items-center gap-1.5 px-2 pb-1 pt-2 text-[10.5px] font-bold uppercase tracking-[0.06em] text-boulder-500";
  const separatorClass = "mx-1 my-1.5 h-px border-0 bg-boulder-100";
  const tileClass =
    "flex size-6 shrink-0 items-center justify-center rounded-md bg-boulder-100 text-boulder-600";
  const activeBadgeClass =
    "ml-auto inline-flex items-center gap-1 rounded-full bg-green-100 px-1.5 py-0.5 text-[10px] font-semibold text-green-700";
  const emptyClass = "px-2 py-2 text-xs text-boulder-500";

  let projects = $state<Project[]>([]);
  let activeEnvironmentId = $state<number | null>(null);
  let isPhpRunning = $state(false);
  let phpVersion = $state<string | null>(null);
  let nodeVersion = $state<string | null>(null);
  let phpVersions = $state<string[]>([]);
  let nodeVersions = $state<string[]>([]);
  let isBusy = $state(false);
  let errorMessage = $state("");

  let activeProfileName = $derived.by(() => {
    for (const project of projects) {
      const env = project.environments.find((e) => e.id === activeEnvironmentId);
      if (env) return `${project.name} / ${env.name}`;
    }
    return null;
  });

  function describeError(caught: unknown) {
    return caught instanceof Error ? caught.message : String(caught);
  }

  async function run(action: () => Promise<void>) {
    if (isBusy) return;
    isBusy = true;
    errorMessage = "";
    try {
      await action();
    } catch (caught) {
      errorMessage = describeError(caught);
    } finally {
      isBusy = false;
    }
  }

  async function updateSecretsStatus() {
    if (!isNativeApp) {
      if (projects.length === 0) {
        projects = [
          {
            id: 1,
            name: "General",
            environments: [
              { id: 1, name: "Development", secrets: [] },
              { id: 2, name: "Production", secrets: [] },
            ],
          },
        ];
        activeEnvironmentId = 1;
      }
      return;
    }
    try {
      const config = await loadSecretProfiles();
      projects = config.projects;
      activeEnvironmentId = config.activeEnvironmentId;
    } catch {
      // Ignored in background refresh
    }
  }

  async function updatePhpStatus() {
    if (!isNativeApp) return;
    try {
      isPhpRunning = await getPhpStatus();
    } catch {
      isPhpRunning = false;
    }
  }

  async function updateRuntimes() {
    if (!isNativeApp) {
      phpVersions = ["8.3.17", "8.2.27"];
      nodeVersions = ["22.14.0", "20.18.3"];
      phpVersion ??= phpVersions[0];
      nodeVersion ??= nodeVersions[0];
      return;
    }
    try {
      const [active, php, node] = await Promise.all([
        getActiveRuntimes(),
        getInstalledVersions(RuntimeService.PHP),
        getInstalledVersions(RuntimeService.Node),
      ]);
      phpVersions = php;
      nodeVersions = node;
      phpVersion = active.php ?? null;
      nodeVersion = active.nodejs ?? null;
    } catch {
      // Ignored in background refresh
    }
  }

  async function refreshAll() {
    await Promise.all([updateSecretsStatus(), updatePhpStatus(), updateRuntimes()]);
  }

  function selectEnvironment(environmentId: number) {
    if (environmentId === activeEnvironmentId) return;
    void run(async () => {
      if (isNativeApp) {
        await activateSecretProfile(environmentId);
        await updateSecretsStatus();
        await emit("secrets-updated");
      } else activeEnvironmentId = environmentId;
    });
  }

  function deactivateEnvironment() {
    void run(async () => {
      if (isNativeApp) {
        await deactivateSecretEnvironment();
        await updateSecretsStatus();
        await emit("secrets-updated");
      } else activeEnvironmentId = null;
    });
  }

  function togglePhp() {
    void run(async () => {
      if (!isNativeApp) {
        isPhpRunning = !isPhpRunning;
        return;
      }
      if (isPhpRunning) await stopPhp();
      else if (phpVersion) await startPhp(phpVersion);
      await updatePhpStatus();
    });
  }

  function selectPhpVersion(version: string) {
    if (cleanVersion(version) === phpVersion) return;
    void run(async () => {
      if (isNativeApp) {
        await activateCatalogVersion(RuntimeService.PHP, version);
        if (isPhpRunning) await startPhp(cleanVersion(version));
        await updatePhpStatus();
      }
      phpVersion = cleanVersion(version);
    });
  }

  function selectNodeVersion(version: string) {
    if (cleanVersion(version) === nodeVersion) return;
    void run(async () => {
      if (isNativeApp) await activateCatalogVersion(RuntimeService.Node, version);
      nodeVersion = cleanVersion(version);
    });
  }

  onMount(() => {
    void refreshAll();

    const handleFocus = () => void refreshAll();
    window.addEventListener("focus", handleFocus);

    let unlisten: (() => void) | undefined;
    if (isNativeApp) {
      listen("secrets-updated", () => {
        void updateSecretsStatus();
      }).then((fn) => {
        unlisten = fn;
      });
    }

    const intervalHandle = window.setInterval(updatePhpStatus, pollIntervalMs);

    return () => {
      window.removeEventListener("focus", handleFocus);
      window.clearInterval(intervalHandle);
      if (unlisten) unlisten();
    };
  });
</script>

{#snippet menuHeader(Icon: typeof KeyRound, title: string, subtitle: string, running?: boolean)}
  <div class="mb-1 flex items-center gap-2.5 rounded-md bg-boulder-50 px-2.5 py-2">
    <span
      class="flex size-8 shrink-0 items-center justify-center rounded-lg bg-east-bay-950 text-white"
    >
      <Icon size={16} aria-hidden="true" />
    </span>
    <div class="min-w-0">
      <div class="truncate text-xs font-semibold text-boulder-950">{title}</div>
      <div class="flex items-center gap-1.5 truncate text-[11px] text-boulder-500">
        {#if running !== undefined}
          <span class={`size-1.5 rounded-full ${running ? "bg-green-500" : "bg-boulder-300"}`}
          ></span>
        {/if}
        {subtitle}
      </div>
    </div>
  </div>
{/snippet}

<footer
  class="flex min-h-7 shrink-0 select-none items-center justify-between bg-east-bay-950 px-3 text-[11.5px] text-east-bay-100 max-sm:gap-2 max-sm:px-2"
  aria-label={t("status.application")}
>
  <div class="flex min-w-0 items-center">
    <Menu.Root
      positioning={{ placement: "top-start", gutter: 6 }}
      onOpenChange={(details) => details.open && void updateSecretsStatus()}
    >
      <Menu.Trigger class={triggerClass} title={t("status.activeProfile")} disabled={isBusy}>
        <KeyRound
          size={13}
          strokeWidth={2.2}
          class="shrink-0 text-east-bay-300"
          aria-hidden="true"
        />
        {#if activeProfileName}
          <strong class="truncate font-semibold text-white">{activeProfileName}</strong>
        {:else}
          <span class="text-east-bay-400">{t("status.noEnvironment")}</span>
        {/if}
        <ChevronUp size={12} class="shrink-0 text-east-bay-400" aria-hidden="true" />
      </Menu.Trigger>
      <Portal>
        <Menu.Positioner>
          <Menu.Content class={contentClass}>
            {@render menuHeader(
              KeyRound,
              t("status.activeProfile"),
              activeProfileName ?? t("status.noEnvironment"),
            )}
            {#each projects as project (project.id)}
              <Menu.ItemGroup>
                <Menu.ItemGroupLabel class={labelClass}>
                  <Folder size={12} aria-hidden="true" />{project.name}
                </Menu.ItemGroupLabel>
                {#each project.environments as environment (environment.id)}
                  <Menu.Item
                    value={`env-${environment.id}`}
                    class={itemClass}
                    onSelect={() => selectEnvironment(environment.id)}
                  >
                    <span class={tileClass}><KeyRound size={13} /></span>
                    <span class="truncate">{environment.name}</span>
                    {#if environment.id === activeEnvironmentId}
                      <span class={activeBadgeClass}
                        ><Check size={10} strokeWidth={3} />{t("status.active")}</span
                      >
                    {/if}
                  </Menu.Item>
                {/each}
              </Menu.ItemGroup>
            {/each}
            {#if projects.length === 0}
              <div class={emptyClass}>{t("status.noEnvironment")}</div>
            {/if}
            <Menu.Separator class={separatorClass} />
            {#if activeEnvironmentId !== null}
              <Menu.Item
                value="deactivate"
                class={`${itemClass} text-red-600 data-[highlighted]:bg-red-50 data-[highlighted]:text-red-700`}
                onSelect={deactivateEnvironment}
              >
                <span class="flex size-6 shrink-0 items-center justify-center rounded-md bg-red-50"
                  ><PowerOff size={13} /></span
                >
                <span>{t("status.deactivateEnvironment")}</span>
              </Menu.Item>
            {/if}
            <Menu.Item value="manage" class={itemClass} onSelect={() => void goto("/secrets")}>
              <span class={tileClass}><Settings2 size={13} /></span>
              <span>{t("status.manageSecrets")}</span>
              <ArrowUpRight size={13} class="ml-auto text-boulder-400" />
            </Menu.Item>
          </Menu.Content>
        </Menu.Positioner>
      </Portal>
    </Menu.Root>
  </div>

  {#if errorMessage}
    <span class="mx-2 min-w-0 truncate text-red-300" role="alert" title={errorMessage}
      >{errorMessage}</span
    >
  {/if}

  <div class="flex min-w-0 items-center gap-1">
    <Menu.Root
      positioning={{ placement: "top-end", gutter: 6 }}
      onOpenChange={(details) => details.open && void updateRuntimes()}
    >
      <Menu.Trigger
        class={`${triggerClass} max-sm:hidden`}
        title={t("status.nodeVersion")}
        disabled={isBusy}
      >
        <Terminal size={13} strokeWidth={2} class="shrink-0 text-east-bay-300" aria-hidden="true" />
        <span class="font-medium text-east-bay-300">Node</span>
        {#if nodeVersion}
          <strong class="font-semibold text-white">v{nodeVersion}</strong>
        {:else}
          <span class="text-east-bay-400">{t("common.none")}</span>
        {/if}
        <ChevronUp size={12} class="shrink-0 text-east-bay-400" aria-hidden="true" />
      </Menu.Trigger>
      <Portal>
        <Menu.Positioner>
          <Menu.Content class={contentClass}>
            {@render menuHeader(
              Terminal,
              "Node.js",
              nodeVersion ? `v${nodeVersion}` : t("common.none"),
            )}
            <Menu.ItemGroup>
              <Menu.ItemGroupLabel class={labelClass}>{t("status.versions")}</Menu.ItemGroupLabel>
              {#each nodeVersions as version (version)}
                <Menu.Item
                  value={`node-${version}`}
                  class={itemClass}
                  onSelect={() => selectNodeVersion(version)}
                >
                  <span class={tileClass}><Hexagon size={13} /></span>
                  <span>v{cleanVersion(version)}</span>
                  {#if cleanVersion(version) === nodeVersion}
                    <span class={activeBadgeClass}
                      ><Check size={10} strokeWidth={3} />{t("status.active")}</span
                    >
                  {/if}
                </Menu.Item>
              {:else}
                <div class={emptyClass}>{t("status.noVersions")}</div>
              {/each}
            </Menu.ItemGroup>
            <Menu.Separator class={separatorClass} />
            <Menu.Item value="manage" class={itemClass} onSelect={() => void goto("/services")}>
              <span class={tileClass}><Settings2 size={13} /></span>
              <span>{t("status.manageServices")}</span>
              <ArrowUpRight size={13} class="ml-auto text-boulder-400" />
            </Menu.Item>
          </Menu.Content>
        </Menu.Positioner>
      </Portal>
    </Menu.Root>

    <span class="h-3 w-px bg-east-bay-800 max-sm:hidden" aria-hidden="true"></span>

    <Menu.Root
      positioning={{ placement: "top-end", gutter: 6 }}
      onOpenChange={(details) => details.open && void updateRuntimes()}
    >
      <Menu.Trigger class={triggerClass} title={t("status.phpService")} disabled={isBusy}>
        <span
          class={`size-1.75 rounded-full ${isPhpRunning ? "bg-green-500 shadow-[0_0_6px_rgb(34_197_94/60%)]" : "bg-east-bay-400"}`}
          aria-hidden="true"
        ></span>
        <Server size={13} strokeWidth={2} class="shrink-0 text-east-bay-300" aria-hidden="true" />
        <span class="font-medium text-east-bay-300">PHP</span>
        {#if phpVersion}<strong class="font-semibold text-white">v{phpVersion}</strong>{/if}
        <span class="text-east-bay-300" aria-live="polite"
          >{isPhpRunning ? "127.0.0.1:9070" : t("common.stopped")}</span
        >
        <ChevronUp size={12} class="shrink-0 text-east-bay-400" aria-hidden="true" />
      </Menu.Trigger>
      <Portal>
        <Menu.Positioner>
          <Menu.Content class={contentClass}>
            {@render menuHeader(
              FileCode2,
              "PHP FastCGI",
              isPhpRunning ? `${t("status.running")} · 127.0.0.1:9070` : t("common.stopped"),
              isPhpRunning,
            )}
            <Menu.Item
              value="toggle"
              class={`${itemClass} ${isPhpRunning ? "text-red-600 data-[highlighted]:bg-red-50 data-[highlighted]:text-red-700" : "text-green-700 data-[highlighted]:bg-green-50 data-[highlighted]:text-green-800"}`}
              disabled={!isPhpRunning && !phpVersion}
              onSelect={togglePhp}
            >
              <span
                class={`flex size-6 shrink-0 items-center justify-center rounded-md ${isPhpRunning ? "bg-red-50" : "bg-green-50"}`}
              >
                {#if isPhpRunning}
                  <Square size={12} fill="currentColor" />
                {:else}
                  <Play size={12} fill="currentColor" />
                {/if}
              </span>
              <span>{isPhpRunning ? t("status.stopPhp") : t("status.startPhp")}</span>
            </Menu.Item>
            <Menu.Separator class={separatorClass} />
            <Menu.ItemGroup>
              <Menu.ItemGroupLabel class={labelClass}>{t("status.versions")}</Menu.ItemGroupLabel>
              {#each phpVersions as version (version)}
                <Menu.Item
                  value={`php-${version}`}
                  class={itemClass}
                  onSelect={() => selectPhpVersion(version)}
                >
                  <span class={tileClass}><FileCode2 size={13} /></span>
                  <span>v{cleanVersion(version)}</span>
                  {#if cleanVersion(version) === phpVersion}
                    <span class={activeBadgeClass}
                      ><Check size={10} strokeWidth={3} />{t("status.active")}</span
                    >
                  {/if}
                </Menu.Item>
              {:else}
                <div class={emptyClass}>{t("status.noVersions")}</div>
              {/each}
            </Menu.ItemGroup>
            <Menu.Separator class={separatorClass} />
            <Menu.Item value="manage" class={itemClass} onSelect={() => void goto("/services")}>
              <span class={tileClass}><Settings2 size={13} /></span>
              <span>{t("status.manageServices")}</span>
              <ArrowUpRight size={13} class="ml-auto text-boulder-400" />
            </Menu.Item>
          </Menu.Content>
        </Menu.Positioner>
      </Portal>
    </Menu.Root>
  </div>
</footer>
