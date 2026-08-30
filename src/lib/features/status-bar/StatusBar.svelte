<script lang="ts">
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { KeyRound, Server, ShieldAlert } from "@lucide/svelte";
  import type { SecretsConfiguration } from "$lib/features/secrets/types";

  const isNativeApp = isTauri();
  const pollIntervalMs = 5_000;

  let activeProfileName = $state<string | null>(null);
  let isProduction = $state(false);
  let isPhpRunning = $state(false);

  async function updateSecretsStatus() {
    if (!isNativeApp) {
      activeProfileName = "Pruebas";
      isProduction = false;
      return;
    }

    try {
      const config = await invoke<SecretsConfiguration>("load_secret_profiles");
      const active = config.profiles.find((p) => p.id === config.activeProfileId);
      if (active) {
        activeProfileName = active.name || "Untitled";
        isProduction = active.isProduction;
      } else {
        activeProfileName = null;
        isProduction = false;
      }
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
      isPhpRunning = await invoke<boolean>("get_php_status");
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

<footer class="status-bar" aria-label="Application status">
  <div class="status-group">
    <a href="/secrets" class="status-item status-link" title="Active environment secrets profile">
      {#if isProduction}
        <ShieldAlert size={13} strokeWidth={2.4} class="alert-icon" aria-hidden="true" />
      {:else}
        <KeyRound size={13} strokeWidth={2.2} aria-hidden="true" />
      {/if}
      <span class="label">Env:</span>
      {#if activeProfileName}
        <strong class={isProduction ? "production-name" : ""}>{activeProfileName}</strong>
        {#if isProduction}
          <span class="prod-badge">PROD</span>
        {/if}
      {:else}
        <span class="muted-text">None</span>
      {/if}
    </a>
  </div>

  <div class="status-group">
    <a href="/services" class="status-item status-link" title="PHP FastCGI service status">
      <span class={`status-dot ${isPhpRunning ? "running" : "stopped"}`} aria-hidden="true"></span>
      <Server size={13} strokeWidth={2} aria-hidden="true" />
      <span class="label">FastCGI:</span>
      <strong>{isPhpRunning ? "127.0.0.1:9070" : "Stopped"}</strong>
    </a>

    <span class="status-divider" aria-hidden="true"></span>

    <span class="status-item" title={isNativeApp ? "Tauri Desktop Runtime" : "Web Preview Mode"}>
      <span class="platform-badge">{isNativeApp ? "Native" : "Web"}</span>
    </span>
  </div>
</footer>

<style>
  .status-bar {
    align-items: center;
    background: var(--color-east-bay-950);
    box-sizing: border-box;
    color: var(--color-east-bay-100);
    display: flex;
    flex-shrink: 0;
    font-size: 11.5px;
    justify-content: space-between;
    min-height: 28px;
    padding: 0 12px;
    user-select: none;
  }

  .status-group {
    align-items: center;
    display: flex;
    gap: 8px;
    min-width: 0;
  }

  .status-item {
    align-items: center;
    color: inherit;
    display: inline-flex;
    gap: 6px;
    min-height: 28px;
    text-decoration: none;
    white-space: nowrap;
  }

  .status-link {
    border-radius: 4px;
    cursor: pointer;
    padding: 0 4px;
    transition: background 0.12s ease;
  }

  .status-link:hover {
    background: rgb(255 255 255 / 8%);
  }

  .status-item :global(svg) {
    color: var(--color-east-bay-300);
  }

  .status-item :global(svg.alert-icon) {
    color: #f59e0b;
  }

  .label {
    color: var(--color-east-bay-300);
    font-weight: 500;
  }

  .status-item strong {
    color: #ffffff;
    font-weight: 600;
  }

  .production-name {
    color: #fbbf24 !important;
  }

  .prod-badge {
    background: #d97706;
    border-radius: 3px;
    color: #ffffff;
    font-size: 9px;
    font-weight: 750;
    letter-spacing: 0.04em;
    line-height: 1;
    padding: 2px 4px;
  }

  .muted-text {
    color: var(--color-east-bay-400);
  }

  .status-dot {
    border-radius: 999px;
    display: inline-block;
    height: 7px;
    width: 7px;
  }

  .status-dot.running {
    background: #22c55e;
    box-shadow: 0 0 6px rgb(34 197 94 / 60%);
  }

  .status-dot.stopped {
    background: var(--color-east-bay-600);
  }

  .status-divider {
    background: var(--color-east-bay-800);
    height: 12px;
    width: 1px;
  }

  .platform-badge {
    color: var(--color-east-bay-400);
    font-size: 10.5px;
    font-weight: 600;
  }

  @media (max-width: 540px) {
    .status-bar {
      gap: 8px;
      padding: 0 8px;
    }

    .platform-badge,
    .status-divider {
      display: none;
    }
  }
</style>
