<script lang="ts">
  import { ExternalLink, Folder, FolderOpen, Globe, Layers } from "@lucide/svelte";

  type Props = {
    localUrl: string;
    documentRoot: string;
    activeProfileName: string;
    onOpenUrl: (url: string) => void;
    onOpenPath: (path: string) => void;
  };

  let { localUrl, documentRoot, activeProfileName, onOpenUrl, onOpenPath }: Props = $props();
</script>

<div class="env-overview-card">
  <div class="overview-item">
    <div class="overview-label-row">
      <Globe size={14} strokeWidth={2} class="overview-icon" />
      <span class="overview-label">URL Local</span>
    </div>
    <button
      type="button"
      class="overview-link-button"
      onclick={() => onOpenUrl(localUrl)}
      title={`Abrir ${localUrl} en el navegador`}
    >
      <span class="overview-value">{localUrl}</span>
      <ExternalLink size={13} strokeWidth={2} class="overview-action-icon" aria-hidden="true" />
    </button>
  </div>

  <div class="overview-divider" aria-hidden="true"></div>

  <div class="overview-item">
    <div class="overview-label-row">
      <Folder size={14} strokeWidth={2} class="overview-icon" />
      <span class="overview-label">Carpeta Web (Document Root)</span>
    </div>
    <button
      type="button"
      class="overview-link-button"
      onclick={() => onOpenPath(documentRoot)}
      title={`Abrir ${documentRoot} en el explorador de archivos`}
    >
      <span class="overview-value">{documentRoot}</span>
      <FolderOpen size={13} strokeWidth={2} class="overview-action-icon" aria-hidden="true" />
    </button>
  </div>

  <div class="overview-divider" aria-hidden="true"></div>

  <div class="overview-item">
    <div class="overview-label-row">
      <Layers size={14} strokeWidth={2} class="overview-icon" />
      <span class="overview-label">Perfil Activo</span>
    </div>
    <span class="overview-value highlight">{activeProfileName}</span>
  </div>
</div>

<style>
  .env-overview-card {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    gap: 16px;
    padding: 14px 18px;
  }
  .overview-item {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }
  .overview-label-row {
    align-items: center;
    color: var(--color-boulder-500);
    display: flex;
    gap: 6px;
  }
  :global(.overview-icon) {
    color: var(--color-boulder-400);
  }
  .overview-label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }
  .overview-link-button {
    align-items: center;
    appearance: none;
    -webkit-appearance: none;
    background: transparent;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    display: inline-flex;
    gap: 6px;
    margin: 0;
    max-width: 100%;
    outline: none;
    padding: 0;
    text-align: left;
  }
  .overview-link-button:hover .overview-value {
    color: var(--color-east-bay-700);
    text-decoration: underline;
  }
  .overview-link-button:hover :global(.overview-action-icon) {
    color: var(--color-east-bay-700);
    transform: translate(1px, -1px);
  }
  .overview-link-button:focus-visible {
    outline: 2px solid var(--color-east-bay-400);
    outline-offset: 2px;
  }
  :global(.overview-action-icon) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
    transition:
      color 0.15s ease,
      transform 0.15s ease;
  }
  .overview-value {
    color: var(--color-boulder-900);
    font-family: var(--font-mono);
    font-size: 13px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .overview-value.highlight {
    color: var(--color-east-bay-900);
    font-family: inherit;
    font-weight: 700;
  }
  .overview-divider {
    background: var(--color-boulder-200);
    height: 32px;
    width: 1px;
  }

  @media (max-width: 720px) {
    .env-overview-card {
      align-items: stretch;
      flex-direction: column;
    }
    .overview-divider {
      display: none;
    }
  }
</style>
