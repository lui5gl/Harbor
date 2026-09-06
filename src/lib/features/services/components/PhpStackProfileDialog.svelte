<script lang="ts">
  import { Check, ChevronDown, Download, Loader2, X } from "@lucide/svelte";
  import { Button, Dialog, Select, Separator } from "bits-ui";
  import type { PhpStackProfile } from "../types";

  type VersionOption = { version: string; label: string };
  type Props = {
    open: boolean;
    editingProfile: PhpStackProfile | null;
    name: string;
    phpVersion: string;
    apacheVersion: string;
    installedPhpVersions: readonly string[];
    installedApacheVersions: readonly string[];
    phpDownloadOptions: readonly VersionOption[];
    apacheDownloadOptions: readonly VersionOption[];
    isPhpMissing: boolean;
    isApacheMissing: boolean;
    isInstalling: boolean;
    installProgress: number;
    error: string;
    onSubmit: () => void;
  };

  let {
    open = $bindable(),
    editingProfile,
    name = $bindable(),
    phpVersion = $bindable(),
    apacheVersion = $bindable(),
    installedPhpVersions,
    installedApacheVersions,
    phpDownloadOptions,
    apacheDownloadOptions,
    isPhpMissing,
    isApacheMissing,
    isInstalling,
    installProgress,
    error,
    onSubmit,
  }: Props = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-overlay" />
    <Dialog.Content class="modal-content" aria-labelledby="profile-modal-title">
      <div class="modal-header">
        <div>
          <Dialog.Title id="profile-modal-title" class="modal-title">
            {editingProfile ? "Editar Perfil de Stack" : "Nuevo Perfil de Entorno"}
          </Dialog.Title>
          <Dialog.Description class="modal-desc">
            Configura el par de PHP y Apache para este stack. Si eliges una versión no descargada,
            se instalará automáticamente.
          </Dialog.Description>
        </div>
        <Dialog.Close class="modal-close-btn" aria-label="Cerrar"
          ><X size={16} strokeWidth={2} /></Dialog.Close
        >
      </div>

      {#if error}<div class="form-error-banner"><span>{error}</span></div>{/if}

      <div class="modal-form">
        <div class="form-group">
          <label for="profile-name-input" class="form-label">Nombre del Perfil</label>
          <input
            id="profile-name-input"
            type="text"
            class="form-input"
            placeholder="ej. Laravel 11 Project, Legacy PHP 7.4"
            bind:value={name}
          />
        </div>
        <div class="form-grid-two">
          <div class="form-group">
            <div class="field-label-row">
              <span class="form-label">Versión de PHP</span>{#if isPhpMissing}<span
                  class="auto-download-tag">Se descargará</span
                >{/if}
            </div>
            <Select.Root type="single" bind:value={phpVersion}>
              <Select.Trigger class="bits-select-trigger" aria-label="Seleccionar versión de PHP"
                ><span class="select-value-text"
                  >{phpVersion ? `PHP ${phpVersion}` : "Seleccionar PHP"}</span
                ><ChevronDown size={14} strokeWidth={2.2} class="select-chevron" /></Select.Trigger
              >
              <Select.Portal
                ><Select.Content class="bits-select-content" sideOffset={5} align="start"
                  ><Select.Viewport class="bits-select-viewport">
                    {#if installedPhpVersions.length > 0}<Select.Group
                        ><Select.GroupHeading class="bits-select-group-label"
                          >Versiones Instaladas</Select.GroupHeading
                        >{#each installedPhpVersions as version}<Select.Item
                            class="bits-select-item"
                            value={version}
                            label={`PHP ${version}`}
                            >{#snippet children({ selected })}<span class="item-label"
                                >PHP {version} (Instalada)</span
                              >{#if selected}<Check
                                  size={14}
                                  strokeWidth={2.4}
                                  class="item-check"
                                />{/if}{/snippet}</Select.Item
                          >{/each}</Select.Group
                      >{/if}
                    {#if phpDownloadOptions.length > 0}<Separator.Root
                        class="bits-select-separator"
                      /><Select.Group
                        ><Select.GroupHeading class="bits-select-group-label"
                          >Descargar e Instalar</Select.GroupHeading
                        >{#each phpDownloadOptions as option}<Select.Item
                            class="bits-select-item download-item"
                            value={option.version}
                            label={option.label}
                            >{#snippet children({ selected })}<span class="item-label"
                                >{option.label}</span
                              >{#if selected}<Check
                                  size={14}
                                  strokeWidth={2.4}
                                  class="item-check"
                                />{/if}{/snippet}</Select.Item
                          >{/each}</Select.Group
                      >{/if}
                  </Select.Viewport></Select.Content
                ></Select.Portal
              >
            </Select.Root>
          </div>
          <div class="form-group">
            <div class="field-label-row">
              <span class="form-label">Versión de Apache</span>{#if isApacheMissing}<span
                  class="auto-download-tag">Se descargará</span
                >{/if}
            </div>
            <Select.Root type="single" bind:value={apacheVersion}>
              <Select.Trigger class="bits-select-trigger" aria-label="Seleccionar versión de Apache"
                ><span class="select-value-text"
                  >{apacheVersion ? `Apache ${apacheVersion}` : "Seleccionar Apache"}</span
                ><ChevronDown size={14} strokeWidth={2.2} class="select-chevron" /></Select.Trigger
              >
              <Select.Portal
                ><Select.Content class="bits-select-content" sideOffset={5} align="start"
                  ><Select.Viewport class="bits-select-viewport">
                    {#if installedApacheVersions.length > 0}<Select.Group
                        ><Select.GroupHeading class="bits-select-group-label"
                          >Versiones Instaladas</Select.GroupHeading
                        >{#each installedApacheVersions as version}<Select.Item
                            class="bits-select-item"
                            value={version}
                            label={`Apache ${version}`}
                            >{#snippet children({ selected })}<span class="item-label"
                                >Apache {version} (Instalada)</span
                              >{#if selected}<Check
                                  size={14}
                                  strokeWidth={2.4}
                                  class="item-check"
                                />{/if}{/snippet}</Select.Item
                          >{/each}</Select.Group
                      >{/if}
                    {#if apacheDownloadOptions.length > 0}<Separator.Root
                        class="bits-select-separator"
                      /><Select.Group
                        ><Select.GroupHeading class="bits-select-group-label"
                          >Descargar e Instalar</Select.GroupHeading
                        >{#each apacheDownloadOptions as option}<Select.Item
                            class="bits-select-item download-item"
                            value={option.version}
                            label={option.label}
                            >{#snippet children({ selected })}<span class="item-label"
                                >{option.label}</span
                              >{#if selected}<Check
                                  size={14}
                                  strokeWidth={2.4}
                                  class="item-check"
                                />{/if}{/snippet}</Select.Item
                          >{/each}</Select.Group
                      >{/if}
                  </Select.Viewport></Select.Content
                ></Select.Portal
              >
            </Select.Root>
          </div>
        </div>
        {#if isPhpMissing || isApacheMissing}<div class="download-notice">
            <Download size={14} strokeWidth={2} class="download-notice-icon" /><span
              >Las versiones marcadas se descargarán e instalarán automáticamente al guardar.</span
            >
          </div>{/if}
      </div>

      <div class="modal-footer">
        <Button.Root
          type="button"
          class="secondary-button"
          disabled={isInstalling}
          onclick={() => (open = false)}>Cancelar</Button.Root
        >
        <Button.Root
          type="button"
          class="primary-button-sm"
          disabled={isInstalling}
          onclick={onSubmit}
          >{#if isInstalling}<Loader2 size={14} class="spin" /><span
              >Instalando... {installProgress > 0 ? `${installProgress}%` : ""}</span
            >{:else}<span>{editingProfile ? "Guardar Cambios" : "Crear Perfil"}</span
            >{/if}</Button.Root
        >
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.modal-overlay) {
    background: rgb(0 0 0 / 40%);
    inset: 0;
    position: fixed;
    z-index: 50;
  }

  :global(.modal-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    display: flex;
    flex-direction: column;
    gap: 18px;
    left: 50%;
    max-width: 480px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: calc(100vw - 32px);
    z-index: 51;
  }

  .modal-header {
    align-items: flex-start;
    display: flex;
    justify-content: space-between;
    gap: 12px;
  }

  :global(.modal-title) {
    color: var(--color-boulder-950);
    font-size: 16px;
    font-weight: 650;
    margin: 0;
  }

  :global(.modal-desc) {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.4;
    margin: 4px 0 0;
  }

  :global(.modal-close-btn) {
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: flex;
    padding: 4px;
    transition: color 150ms ease;
  }

  :global(.modal-close-btn:hover) {
    color: var(--color-boulder-800);
  }

  .form-error-banner {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 6px;
    color: #b91c1c;
    font-size: 12.5px;
    padding: 8px 12px;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .field-label-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }
  .auto-download-tag {
    background: #ecfdf5;
    border-radius: 4px;
    color: #065f46;
    font-size: 10.5px;
    font-weight: 600;
    padding: 1px 6px;
  }
  .form-grid-two {
    display: grid;
    gap: 12px;
    grid-template-columns: 1fr 1fr;
  }
  .form-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 600;
  }
  .form-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 13px;
    height: 36px;
    padding: 0 10px;
    transition: border-color 150ms ease;
    width: 100%;
  }
  .form-input:focus {
    border-color: var(--color-east-bay-600);
    outline: none;
  }
  :global(.bits-select-trigger) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    cursor: pointer;
    display: flex;
    font: inherit;
    font-size: 13px;
    height: 36px;
    justify-content: space-between;
    padding: 0 10px;
    transition: border-color 150ms ease;
    width: 100%;
  }
  :global(.bits-select-trigger:focus-visible) {
    border-color: var(--color-east-bay-600);
    outline: none;
  }
  .select-value-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global(.select-chevron) {
    color: var(--color-boulder-500);
    flex-shrink: 0;
  }
  :global(.bits-select-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    max-height: 240px;
    min-width: 220px;
    overflow-y: auto;
    padding: 4px;
    z-index: 80;
  }
  :global(.bits-select-viewport) {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  :global(.bits-select-group-label) {
    color: var(--color-boulder-500);
    font-size: 10.5px;
    font-weight: 700;
    letter-spacing: 0.04em;
    padding: 6px 8px 3px;
    text-transform: uppercase;
  }
  :global(.bits-select-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }
  :global(.bits-select-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: flex;
    font-size: 12.5px;
    justify-content: space-between;
    outline: none;
    padding: 6px 8px;
    transition: background-color 100ms ease;
    user-select: none;
  }
  :global(.bits-select-item[data-highlighted]) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }
  :global(.bits-select-item[data-selected]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-900);
    font-weight: 600;
  }
  :global(.bits-select-item.download-item) {
    color: var(--color-east-bay-700);
  }
  .item-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  :global(.item-check) {
    color: var(--color-east-bay-800);
    flex-shrink: 0;
  }
  .download-notice {
    align-items: center;
    background: #f0fdf4;
    border: 1px solid #bbf7d0;
    border-radius: 6px;
    color: #15803d;
    display: flex;
    font-size: 12px;
    gap: 8px;
    padding: 8px 12px;
  }
  :global(.download-notice-icon) {
    color: #16a34a;
    flex-shrink: 0;
  }
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding-top: 6px;
  }
  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  @media (max-width: 720px) {
    .form-grid-two {
      grid-template-columns: 1fr;
    }
  }
</style>
