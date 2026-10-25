<script lang="ts">
  import { isTauri } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { KeyRound, Server } from "@lucide/svelte";
  import type { SecretsConfiguration } from "#lib/features/secrets/types";
  import { loadSecretProfiles } from "#lib/features/secrets/infrastructure/secretsRepository";
  import { getPhpStatus } from "#lib/features/services/infrastructure/runtimesRepository";
  import { t } from "#lib/i18n";

  const isNativeApp = isTauri();
  const pollIntervalMs = 5_000;

  let activeProfileName = $state<string | null>(null);
  let isPhpRunning = $state(false);

  async function updateSecretsStatus() {
    if (!isNativeApp) {
      activeProfileName = "General / Development";
      return;
    }

    try {
      const config = await loadSecretProfiles();
      let activeEnv: { name: string; projectName: string } | null = null;
      for (const project of config.projects) {
        const env = project.environments.find((e) => e.id === config.activeEnvironmentId);
        if (env) {
          activeEnv = { name: env.name, projectName: project.name };
          break;
        }
      }
      activeProfileName = activeEnv ? `${activeEnv.projectName} / ${activeEnv.name}` : null;
    } catch {
      // Ignored in background refresh
    }
  }

  async function updatePhpStatus() {
    if (!isNativeApp) {
      isPhpRunning = false;
      return;
    }
    try {
      isPhpRunning = await getPhpStatus();
    } catch {
      isPhpRunning = false;
    }
  }

  onMount(() => {
    void updateSecretsStatus();
    void updatePhpStatus();

    const handleFocus = () => {
      void updateSecretsStatus();
      void updatePhpStatus();
    };
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
  <div class="flex min-w-0 items-center gap-2">
    <a
      href="/secrets"
      class="inline-flex min-h-7 items-center gap-1.5 whitespace-nowrap rounded px-1 text-inherit no-underline hover:bg-white/8"
      title={t("status.activeProfile")}
    >
      <KeyRound size={13} strokeWidth={2.2} class="text-east-bay-300" aria-hidden="true" />
      <span class="font-medium text-east-bay-300">{t("status.environment")}</span>
      {#if activeProfileName}
        <strong class="font-semibold text-white">{activeProfileName}</strong>
      {:else}
        <span class="text-east-bay-400">{t("common.none")}</span>
      {/if}
    </a>
  </div>

  <div class="flex min-w-0 items-center gap-2">
    <a
      href="/services"
      class="inline-flex min-h-7 items-center gap-1.5 whitespace-nowrap rounded px-1 text-inherit no-underline hover:bg-white/8"
      title={t("status.phpService")}
    >
      <span
        class={`size-1.75 rounded-full ${isPhpRunning ? "bg-green-500 shadow-[0_0_6px_rgb(34_197_94/60%)]" : "bg-east-bay-600"}`}
        aria-hidden="true"
      ></span>
      <Server size={13} strokeWidth={2} class="text-east-bay-300" aria-hidden="true" />
      <span class="font-medium text-east-bay-300">FastCGI:</span>
      <strong class="font-semibold text-white"
        >{isPhpRunning ? "127.0.0.1:9070" : t("common.stopped")}</strong
      >
    </a>

    <span class="h-3 w-px bg-east-bay-800 max-sm:hidden" aria-hidden="true"></span>

    <span
      class="inline-flex min-h-7 items-center gap-1.5 whitespace-nowrap"
      title={isNativeApp ? t("status.tauriRuntime") : t("status.webPreview")}
    >
      <span class="text-[10.5px] font-semibold text-east-bay-400 max-sm:hidden"
        >{isNativeApp ? t("common.native") : t("common.web")}</span
      >
    </span>
  </div>
</footer>
