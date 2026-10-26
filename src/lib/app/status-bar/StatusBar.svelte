<script lang="ts">
  import { isTauri } from "@tauri-apps/api/core";
  import { emit, listen } from "@tauri-apps/api/event";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { Menu } from "@ark-ui/svelte/menu";
  import { Portal } from "@ark-ui/svelte/portal";
  import {
    Check,
    ChevronUp,
    KeyRound,
    Play,
    PowerOff,
    Server,
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
    "z-100 max-h-72 min-w-56 overflow-y-auto rounded-md border border-boulder-200 bg-white p-1.5 shadow-lg outline-none";
  const itemClass =
    "flex h-8 cursor-pointer items-center gap-2 rounded px-2 text-xs font-medium text-boulder-700 outline-none transition-colors data-[highlighted]:bg-boulder-100 data-[highlighted]:text-boulder-950 data-[disabled]:cursor-not-allowed data-[disabled]:opacity-40";
  const labelClass =
    "px-2 py-1 text-[10.5px] font-bold uppercase tracking-[0.04em] text-boulder-500";
  const separatorClass = "mx-2 my-1 h-px border-0 bg-boulder-100";

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
            {#each projects as project (project.id)}
              <Menu.ItemGroup>
                <Menu.ItemGroupLabel class={labelClass}>{project.name}</Menu.ItemGroupLabel>
                {#each project.environments as environment (environment.id)}
                  <Menu.Item
                    value={`env-${environment.id}`}
                    class={itemClass}
                    onSelect={() => selectEnvironment(environment.id)}
                  >
                    <span class="flex size-3.5 shrink-0 items-center justify-center">
                      {#if environment.id === activeEnvironmentId}<Check size={14} />{/if}
                    </span>
                    <span class="truncate">{environment.name}</span>
                  </Menu.Item>
                {/each}
              </Menu.ItemGroup>
            {/each}
            {#if projects.length === 0}
              <div class="px-2 py-1.5 text-xs text-boulder-500">{t("status.noEnvironment")}</div>
            {/if}
            <Menu.Separator class={separatorClass} />
            {#if activeEnvironmentId !== null}
              <Menu.Item value="deactivate" class={itemClass} onSelect={deactivateEnvironment}>
                <PowerOff size={14} /><span>{t("status.deactivateEnvironment")}</span>
              </Menu.Item>
            {/if}
            <Menu.Item value="manage" class={itemClass} onSelect={() => void goto("/secrets")}>
              <KeyRound size={14} /><span>{t("status.manageSecrets")}</span>
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
            <Menu.ItemGroup>
              <Menu.ItemGroupLabel class={labelClass}>Node.js</Menu.ItemGroupLabel>
              {#each nodeVersions as version (version)}
                <Menu.Item
                  value={`node-${version}`}
                  class={itemClass}
                  onSelect={() => selectNodeVersion(version)}
                >
                  <span class="flex size-3.5 shrink-0 items-center justify-center">
                    {#if cleanVersion(version) === nodeVersion}<Check size={14} />{/if}
                  </span>
                  <span>v{cleanVersion(version)}</span>
                </Menu.Item>
              {:else}
                <div class="px-2 py-1.5 text-xs text-boulder-500">{t("status.noVersions")}</div>
              {/each}
            </Menu.ItemGroup>
            <Menu.Separator class={separatorClass} />
            <Menu.Item value="manage" class={itemClass} onSelect={() => void goto("/services")}>
              <Server size={14} /><span>{t("status.manageServices")}</span>
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
            <Menu.Item
              value="toggle"
              class={itemClass}
              disabled={!isPhpRunning && !phpVersion}
              onSelect={togglePhp}
            >
              {#if isPhpRunning}
                <Square size={13} fill="currentColor" /><span>{t("status.stopPhp")}</span>
              {:else}
                <Play size={13} fill="currentColor" /><span>{t("status.startPhp")}</span>
              {/if}
            </Menu.Item>
            <Menu.Separator class={separatorClass} />
            <Menu.ItemGroup>
              <Menu.ItemGroupLabel class={labelClass}>PHP</Menu.ItemGroupLabel>
              {#each phpVersions as version (version)}
                <Menu.Item
                  value={`php-${version}`}
                  class={itemClass}
                  onSelect={() => selectPhpVersion(version)}
                >
                  <span class="flex size-3.5 shrink-0 items-center justify-center">
                    {#if cleanVersion(version) === phpVersion}<Check size={14} />{/if}
                  </span>
                  <span>v{cleanVersion(version)}</span>
                </Menu.Item>
              {:else}
                <div class="px-2 py-1.5 text-xs text-boulder-500">{t("status.noVersions")}</div>
              {/each}
            </Menu.ItemGroup>
            <Menu.Separator class={separatorClass} />
            <Menu.Item value="manage" class={itemClass} onSelect={() => void goto("/services")}>
              <Server size={14} /><span>{t("status.manageServices")}</span>
            </Menu.Item>
          </Menu.Content>
        </Menu.Positioner>
      </Portal>
    </Menu.Root>
  </div>
</footer>
