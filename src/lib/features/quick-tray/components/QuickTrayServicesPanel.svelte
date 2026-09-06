<script lang="ts">
  import { Maximize2, Play, Square } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import type { QuickTrayServices } from "$lib/features/quick-tray/application/quickTrayServices.svelte";

  type Props = {
    services: QuickTrayServices;
    onOpenMainWindow: () => void;
  };

  let { services, onOpenMainWindow }: Props = $props();
</script>

<div class="services-tray-view">
  <div class="services-tray-list">
    <div class="service-tray-card">
      <div class="service-tray-header">
        <div class="service-tray-title">
          <span class="service-icon-badge php-badge">PHP</span>
          <div>
            <h3 class="service-name">PHP</h3>
            <p class="service-subtitle">FastCGI & CLI</p>
          </div>
        </div>
        <Button.Root
          class={`service-toggle-btn${services.isPhpRunning ? " running" : ""}`}
          type="button"
          disabled={services.isBusy || services.php.installedVersions.length === 0}
          onclick={() => void services.togglePhp()}
          title={services.isPhpRunning ? "Detener FastCGI" : "Iniciar FastCGI"}
        >
          {#if services.isPhpRunning}<Square size={13} strokeWidth={2} /><span>Detener</span
            >{:else}<Play size={13} strokeWidth={2} /><span>Iniciar</span>{/if}
        </Button.Root>
      </div>
      <div class="service-tray-body">
        <div class="service-field">
          <span class="field-desc">Versión CLI / FastCGI:</span>
          {#if services.php.installedVersions.length > 0}
            <select
              class="service-select"
              value={services.php.selectedVersion}
              disabled={services.isBusy}
              onchange={(event) => void services.selectPhpVersion(event.currentTarget.value)}
            >
              {#each services.php.installedVersions as version}<option value={version}
                  >{version}</option
                >{/each}
            </select>
          {:else}<span class="not-installed-label">No instalado</span>{/if}
        </div>
        <div class="service-status-row">
          <span class={`status-pill ${services.isPhpRunning ? "running" : "stopped"}`}
            ><span class="status-dot-indicator"></span><span
              >{services.isPhpRunning ? "FastCGI 127.0.0.1:9070" : "Detenido"}</span
            ></span
          >
        </div>
      </div>
    </div>

    <div class="service-tray-card">
      <div class="service-tray-header">
        <div class="service-tray-title">
          <span class="service-icon-badge node-badge">Node</span>
          <div>
            <h3 class="service-name">Node.js</h3>
            <p class="service-subtitle">JavaScript Runtime</p>
          </div>
        </div>
      </div>
      <div class="service-tray-body">
        <div class="service-field">
          <span class="field-desc">Versión activa:</span>
          {#if services.node.installedVersions.length > 0}
            <select
              class="service-select"
              value={services.node.selectedVersion}
              disabled={services.isBusy}
              onchange={(event) => void services.selectNodeVersion(event.currentTarget.value)}
            >
              {#each services.node.installedVersions as version}<option value={version}
                  >{version}</option
                >{/each}
            </select>
          {:else}<span class="not-installed-label">No instalado</span>{/if}
        </div>
        <div class="service-status-row">
          <span class="status-pill ready"
            ><span class="status-dot-indicator"></span><span
              >{services.node.selectedVersion
                ? `v${services.node.selectedVersion} listo`
                : "Sin versión"}</span
            ></span
          >
        </div>
      </div>
    </div>

    <div class="service-tray-card">
      <div class="service-tray-header">
        <div class="service-tray-title">
          <span class="service-icon-badge apache-badge">HTTP</span>
          <div>
            <h3 class="service-name">Apache</h3>
            <p class="service-subtitle">Servidor Web</p>
          </div>
        </div>
      </div>
      <div class="service-tray-body">
        <div class="service-field">
          <span class="field-desc">Versión instalada:</span
          >{#if services.apache.installedVersions.length > 0}<span class="service-static-ver"
              >{services.apache.installedVersions[0]}</span
            >{:else}<span class="not-installed-label">No instalado</span>{/if}
        </div>
      </div>
    </div>
  </div>

  <footer class="tray-footer">
    <div class="footer-status">
      <span class="footer-indicator"></span><span
        >{services.php.installedVersions.length +
          services.node.installedVersions.length +
          services.apache.installedVersions.length} instalados</span
      >
    </div>
    <Button.Root class="footer-open-btn" type="button" onclick={onOpenMainWindow}
      ><span>Descargar más en Harbor</span><Maximize2 size={11} strokeWidth={2.4} /></Button.Root
    >
  </footer>
</div>

<style>
  .services-tray-view,
  .services-tray-list,
  .service-tray-card,
  .service-tray-body {
    display: flex;
    flex-direction: column;
  }
  .services-tray-view {
    flex: 1;
    min-height: 0;
  }
  .services-tray-list {
    flex: 1;
    gap: 10px;
    overflow-y: auto;
    padding: 14px;
  }
  .service-tray-card {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    gap: 10px;
    padding: 12px 14px;
  }
  .service-tray-header,
  .service-field {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }
  .service-tray-title {
    align-items: center;
    display: flex;
    gap: 10px;
  }
  .service-icon-badge {
    align-items: center;
    border-radius: 6px;
    display: inline-flex;
    font-size: 10px;
    font-weight: 800;
    height: 28px;
    justify-content: center;
    width: 32px;
  }
  .php-badge {
    background: #ede9fe;
    color: #6d28d9;
  }
  .node-badge {
    background: #ecfdf5;
    color: #047857;
  }
  .apache-badge {
    background: #fef3c7;
    color: #b45309;
  }
  .service-name {
    color: var(--color-boulder-950);
    font-size: 13.5px;
    font-weight: 700;
    margin: 0;
  }
  .service-subtitle {
    color: var(--color-boulder-500);
    font-size: 11px;
    margin: 0;
  }
  .service-tray-body {
    border-top: 1px solid var(--color-boulder-100);
    gap: 8px;
    padding-top: 12px;
  }
  .field-desc {
    color: var(--color-boulder-600);
    font-size: 12px;
  }
  .service-select {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    color: var(--color-boulder-900);
    font: 600 12px var(--font-sans);
    height: 26px;
    padding: 0 6px;
  }
  .service-static-ver {
    color: var(--color-boulder-800);
    font-size: 12px;
    font-weight: 600;
  }
  .not-installed-label {
    color: var(--color-boulder-400);
    font-size: 11.5px;
    font-style: italic;
  }
  .service-status-row {
    align-items: center;
    display: flex;
  }
  .status-pill {
    align-items: center;
    border-radius: 12px;
    display: inline-flex;
    font-size: 10.5px;
    font-weight: 600;
    gap: 5px;
    padding: 2px 8px;
  }
  .status-pill.running {
    background: #ecfdf5;
    color: #047857;
  }
  .status-pill.stopped {
    background: var(--color-boulder-100);
    color: var(--color-boulder-600);
  }
  .status-pill.ready {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-800);
  }
  .status-dot-indicator {
    background: currentColor;
    border-radius: 50%;
    height: 6px;
    width: 6px;
  }
  .tray-footer {
    align-items: center;
    background: #fff;
    border-top: 1px solid var(--color-boulder-200);
    display: flex;
    height: 38px;
    justify-content: space-between;
    padding: 0 14px;
  }
  .footer-status {
    align-items: center;
    color: var(--color-boulder-500);
    display: flex;
    font-size: 11.5px;
    gap: 6px;
  }
  .footer-indicator {
    background: var(--color-boulder-400);
    border-radius: 50%;
    height: 5px;
    width: 5px;
  }
  :global(.service-toggle-btn) {
    align-items: center;
    background: var(--color-east-bay-900);
    border-radius: 5px;
    color: #fff;
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
    height: 28px;
    padding: 0 10px;
  }
  :global(.service-toggle-btn.running) {
    background: #e11d48;
  }
  :global(.footer-open-btn) {
    align-items: center;
    color: var(--color-east-bay-700);
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
  }
</style>
