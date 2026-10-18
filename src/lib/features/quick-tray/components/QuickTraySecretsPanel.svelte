<script lang="ts">
  import { t } from "#lib/i18n";
  import {
    Check,
    ChevronDown,
    Copy,
    Ellipsis,
    Eye,
    EyeOff,
    KeyRound,
    Maximize2,
    Pencil,
    Play,
    Power,
    Plus,
    Search,
    Trash2,
    X,
  } from "@lucide/svelte";
  import { Button, DropdownMenu, ScrollArea, Select, Tooltip } from "bits-ui";
  import type { Environment, Project, Secret } from "#lib/features/secrets/types";
  import type { createSecretsStore } from "#lib/features/secrets/application/secretsStore.svelte";

  type SecretsStore = ReturnType<typeof createSecretsStore>;
  type FlatEnvironment = Environment & {
    displayName: string;
    projectName: string;
    projectId: number;
  };
  type Props = {
    secrets: SecretsStore;
    onCreateEnvironment: () => void;
    onActivate: () => void;
    onOpenAddVariable: () => void;
    onOpenEditVariable: (secret: Secret) => void;
    onDeleteVariable: (id: number) => void;
    onCopy: (text: string, identifier: string) => void;
    onOpenMainWindow: () => void;
    copiedId: string | null;
    revealedSecretIds: number[];
    isActivating: boolean;
  };

  let {
    secrets,
    onCreateEnvironment,
    onActivate,
    onOpenAddVariable,
    onOpenEditVariable,
    onDeleteVariable,
    onCopy,
    onOpenMainWindow,
    copiedId,
    revealedSecretIds,
    isActivating,
  }: Props = $props();

  let selectedEnvironmentIdStr = $state("");
  let searchQuery = $state("");
  let flatEnvironments = $derived.by<FlatEnvironment[]>(() =>
    secrets.projects.flatMap((project: Project) =>
      project.environments.map((environment) => ({
        ...environment,
        displayName: `${project.name} / ${environment.name}`,
        projectName: project.name,
        projectId: project.id,
      })),
    ),
  );
  let selectedEnvironment = $derived(
    flatEnvironments.find((environment) => environment.id === secrets.selectedEnvironmentId),
  );
  let isActive = $derived(selectedEnvironment?.id === secrets.activeEnvironmentId);
  let filteredSecrets = $derived.by(() => {
    if (!selectedEnvironment) return [];
    const query = searchQuery.trim().toLowerCase();
    return query
      ? selectedEnvironment.secrets.filter((secret) => secret.key.toLowerCase().includes(query))
      : selectedEnvironment.secrets;
  });

  $effect(() => {
    if (secrets.selectedEnvironmentId !== null)
      selectedEnvironmentIdStr = String(secrets.selectedEnvironmentId);
  });

  function selectEnvironment(value: string) {
    const environmentId = Number(value);
    const environment = flatEnvironments.find((item) => item.id === environmentId);
    if (environment) secrets.selectEnvironment(environment.projectId, environment.id);
  }

  function toggleReveal(id: number) {
    revealedSecretIds = revealedSecretIds.includes(id)
      ? revealedSecretIds.filter((item) => item !== id)
      : [...revealedSecretIds, id];
  }
</script>

<section class="profile-toolbar">
  <div class="profile-select-group">
    <Select.Root
      type="single"
      bind:value={selectedEnvironmentIdStr}
      onValueChange={selectEnvironment}
    >
      <Select.Trigger class="profile-select-trigger" aria-label={t("secrets.quickTraySelectEnv")}>
        <div class="select-label-wrapper">
          <span class={`status-dot-mini${isActive ? " active" : ""}`}></span>
          <span class="select-profile-name">
            {selectedEnvironment?.displayName || t("secrets.quickTraySelectEnv")}
          </span>
          {#if selectedEnvironment?.isProduction}
            <span class="prod-badge-mini">PROD</span>
          {/if}
        </div>
        <ChevronDown size={13} strokeWidth={2.2} class="select-chevron" />
      </Select.Trigger>
      <Select.Portal>
        <Select.Content class="profile-select-content" sideOffset={5} align="start">
          <Select.Viewport class="profile-select-viewport">
            {#each flatEnvironments as environment (environment.id)}
              <Select.Item
                class="profile-select-item"
                value={String(environment.id)}
                label={environment.displayName}
              >
                {#snippet children({ selected })}
                  <div class="item-left">
                    <span
                      class={environment.id === secrets.activeEnvironmentId
                        ? "active-dot-select"
                        : "dot-placeholder"}
                    ></span>
                    <span class="item-name">{environment.displayName}</span>
                    {#if environment.isProduction}
                      <span class="prod-badge-mini">PROD</span>
                    {/if}
                  </div>
                  {#if selected}<Check size={13} strokeWidth={2.4} class="item-check" />{/if}
                {/snippet}
              </Select.Item>
            {/each}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
    <Button.Root class="new-profile-btn" type="button" onclick={onCreateEnvironment}>
      <Plus size={13} strokeWidth={2.4} />
      <span>{t("secrets.quickTrayNew")}</span>
    </Button.Root>
  </div>
  <div class="activation-row">
    {#if isActive}
      <div class="active-status-tag">
        <div class="active-badge-group">
          <span class="active-pulse-dot"></span>
          <span>{t("secrets.quickTrayActive")}</span>
        </div>
        <Button.Root
          class="deactivate-tray-btn"
          type="button"
          title={t("secrets.quickTrayDeactivate")}
          aria-label={t("secrets.quickTrayDeactivate")}
          disabled={secrets.isBusy}
          onclick={() => void secrets.deactivateEnvironment()}
        >
          <Power size={11} strokeWidth={2.4} />
          <span>{t("secrets.quickTrayDeactivate")}</span>
        </Button.Root>
      </div>
    {:else}
      <div class="inactive-status-row">
        <span class="inactive-label">{t("secrets.quickTrayInactive")}</span>
        <Button.Root
          class="activate-action-btn"
          type="button"
          disabled={isActivating || !secrets.canWrite}
          onclick={onActivate}
        >
          <Play size={10} strokeWidth={2.4} />
          <span
            >{isActivating
              ? t("secrets.quickTrayActivating")
              : t("secrets.quickTrayActivate")}</span
          >
        </Button.Root>
      </div>
    {/if}
  </div>
</section>

<section class="search-section">
  <div class="search-input-wrapper">
    <Search size={13.5} strokeWidth={2} class="search-icon" />
    <input
      type="search"
      class="tray-search-input"
      placeholder={t("secrets.quickTrayFilter")}
      bind:value={searchQuery}
    />
    {#if searchQuery}
      <button
        class="clear-search-btn"
        type="button"
        onclick={() => (searchQuery = "")}
        aria-label={t("secrets.clearFilter")}
      >
        <X size={11} strokeWidth={2.4} />
      </button>
    {/if}
  </div>
  <Button.Root
    class="add-var-btn"
    type="button"
    disabled={isActive || !secrets.canWrite}
    onclick={onOpenAddVariable}
  >
    <Plus size={13} strokeWidth={2.4} />
    <span>{t("secrets.quickTrayAddVar")}</span>
  </Button.Root>
</section>

<div class="secrets-container">
  {#if !selectedEnvironment || selectedEnvironment.secrets.length === 0}
    <div class="empty-state">
      <div class="empty-icon"><KeyRound size={22} strokeWidth={1.8} /></div>
      <p class="empty-title">{t("secrets.quickTrayNoVars")}</p>
      <p class="empty-subtitle">{t("secrets.quickTrayNoVarsDesc")}</p>
      <Button.Root class="empty-add-btn" type="button" onclick={onOpenAddVariable}>
        <Plus size={13} strokeWidth={2.4} />
        <span>{t("secrets.addFirstVariable")}</span>
      </Button.Root>
    </div>
  {:else if filteredSecrets.length === 0}
    <div class="empty-state">
      <p class="empty-title">{t("secrets.quickTrayNoResults")}</p>
      <p class="empty-subtitle">{t("secrets.noMatchFor")} "{searchQuery}".</p>
    </div>
  {:else}
    <ScrollArea.Root class="variables-scroll-area" type="auto"
      ><ScrollArea.Viewport class="variables-viewport">
        {#each filteredSecrets as secret (secret.id)}
          {@const isRevealed = revealedSecretIds.includes(secret.id)}
          {@const isCopied = copiedId === `val-${secret.id}`}
          {@const isKeyCopied = copiedId === `key-${secret.id}`}
          <div class="secret-card">
            <div class="secret-info">
              <div class="secret-key-line">
                <span class="secret-key-text">{secret.key}</span><button
                  class={`copy-key-btn${isKeyCopied ? " copied" : ""}`}
                  type="button"
                  onclick={() => onCopy(secret.key, `key-${secret.id}`)}
                  title="Copiar nombre de variable"
                  aria-label="Copiar nombre de variable"
                  >{#if isKeyCopied}<Check size={11} strokeWidth={2.4} />{:else}<Copy
                      size={11}
                      strokeWidth={2}
                    />{/if}</button
                >
              </div>
              <div class="secret-val-line">
                {#if isRevealed}<span class="secret-val-text">{secret.value || "<vacío>"}</span
                  >{:else}<span class="secret-val-masked">••••••••••••</span>{/if}
              </div>
            </div>
            <div class="secret-card-actions">
              <Tooltip.Root
                ><Tooltip.Trigger
                  class="action-icon-btn"
                  type="button"
                  aria-label={isRevealed ? "Ocultar valor" : "Ver valor"}
                  onclick={() => toggleReveal(secret.id)}
                  >{#if isRevealed}<EyeOff size={14} strokeWidth={2} />{:else}<Eye
                      size={14}
                      strokeWidth={2}
                    />{/if}</Tooltip.Trigger
                ><Tooltip.Portal
                  ><Tooltip.Content class="tooltip-content" sideOffset={6}
                    >{isRevealed ? "Ocultar" : "Mostrar"}</Tooltip.Content
                  ></Tooltip.Portal
                ></Tooltip.Root
              ><Tooltip.Root
                ><Tooltip.Trigger
                  class={`action-icon-btn${isCopied ? " copied" : ""}`}
                  type="button"
                  aria-label="Copiar valor"
                  onclick={() => onCopy(secret.value, `val-${secret.id}`)}
                  >{#if isCopied}<Check size={14} strokeWidth={2.4} />{:else}<Copy
                      size={14}
                      strokeWidth={2}
                    />{/if}</Tooltip.Trigger
                ><Tooltip.Portal
                  ><Tooltip.Content class="tooltip-content" sideOffset={6}
                    >{isCopied ? "Copiado" : "Copiar valor"}</Tooltip.Content
                  ></Tooltip.Portal
                ></Tooltip.Root
              ><DropdownMenu.Root
                ><DropdownMenu.Trigger class="action-icon-btn" type="button" aria-label="Opciones"
                  ><Ellipsis size={15} strokeWidth={2} /></DropdownMenu.Trigger
                ><DropdownMenu.Portal
                  ><DropdownMenu.Content class="dropdown-content" sideOffset={4} align="end"
                    ><DropdownMenu.Item
                      class="dropdown-item"
                      onclick={() => onOpenEditVariable(secret)}
                      ><Pencil size={14} strokeWidth={2} /><span>Editar variable</span
                      ></DropdownMenu.Item
                    ><DropdownMenu.Separator class="dropdown-separator" /><DropdownMenu.Item
                      class="dropdown-item destructive"
                      onclick={() => onDeleteVariable(secret.id)}
                      ><Trash2 size={14} strokeWidth={2} /><span>Eliminar variable</span
                      ></DropdownMenu.Item
                    ></DropdownMenu.Content
                  ></DropdownMenu.Portal
                ></DropdownMenu.Root
              >
            </div>
          </div>
        {/each}
      </ScrollArea.Viewport><ScrollArea.Scrollbar class="variables-scrollbar" orientation="vertical"
        ><ScrollArea.Thumb class="variables-scrollbar-thumb" /></ScrollArea.Scrollbar
      ></ScrollArea.Root
    >
  {/if}
</div>

<footer class="tray-footer">
  <div class="footer-status">
    <span class="footer-indicator"></span>
    <span>
      {selectedEnvironment?.secrets.length ?? 0}
      {selectedEnvironment?.secrets.length === 1 ? t("secrets.variable") : t("secrets.variables")}
    </span>
  </div>
  <Button.Root class="footer-open-btn" type="button" onclick={onOpenMainWindow}>
    <span>{t("secrets.quickTrayOpenFull")}</span>
    <Maximize2 size={11} strokeWidth={2.4} />
  </Button.Root>
</footer>

<style>
  .profile-toolbar {
    background: #ffffff;
    border-bottom: 1px solid var(--color-boulder-200);
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 10px 14px;
    flex-shrink: 0;
  }

  .profile-select-group {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
  }

  :global(.profile-select-trigger) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-300) !important;
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    cursor: pointer;
    display: flex;
    flex: 1;
    height: 32px;
    justify-content: space-between;
    min-width: 0;
    outline: none;
    padding: 0 10px;
    transition: all 0.15s ease;
  }

  :global(.profile-select-trigger:hover) {
    background: var(--color-boulder-50);
    border-color: var(--color-boulder-400) !important;
  }

  .select-label-wrapper {
    align-items: center;
    display: flex;
    gap: 7px;
    min-width: 0;
    overflow: hidden;
  }

  .status-dot-mini {
    background: var(--color-boulder-400);
    border-radius: 50%;
    flex-shrink: 0;
    height: 6px;
    width: 6px;
  }

  .status-dot-mini.active {
    background: #10b981;
    box-shadow: 0 0 0 2px rgb(16 185 129 / 25%);
  }

  .select-profile-name {
    color: var(--color-boulder-950);
    font-size: 12px;
    font-weight: 650;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .prod-badge-mini {
    background: #fffbeb;
    border: 1px solid #fef3c7;
    border-radius: 3px;
    color: #92400e;
    font-size: 8.5px;
    font-weight: 750;
    letter-spacing: 0.04em;
    padding: 0.5px 4px;
    text-transform: uppercase;
  }

  :global(.select-chevron) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
  }

  :global(.new-profile-btn) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-300) !important;
    border-radius: 6px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: inline-flex;
    flex-shrink: 0;
    font: inherit;
    font-size: 12px;
    font-weight: 600;
    gap: 5px;
    height: 32px;
    outline: none;
    padding: 0 11px;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  :global(.new-profile-btn:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-400) !important;
  }

  :global(.profile-select-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-shadow: 0 12px 32px rgb(11 11 11 / 14%);
    min-width: 220px;
    overflow: hidden;
    padding: 4px;
    z-index: 100;
  }

  :global(.profile-select-viewport) {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  :global(.profile-select-item) {
    align-items: center;
    border-radius: 5px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: flex;
    font-size: 12px;
    font-weight: 550;
    justify-content: space-between;
    height: 30px;
    outline: none;
    padding: 0 10px;
    transition: background 0.1s ease;
  }

  :global(.profile-select-item[data-highlighted]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-900);
  }

  .item-left {
    align-items: center;
    display: flex;
    gap: 7px;
  }

  .active-dot-select {
    background: #10b981;
    border-radius: 50%;
    height: 6px;
    width: 6px;
  }

  .dot-placeholder {
    height: 6px;
    width: 6px;
  }

  .item-name {
    font-weight: 600;
  }

  :global(.item-check) {
    color: var(--color-east-bay-700);
  }

  .activation-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
    min-height: 28px;
  }

  .active-status-tag {
    align-items: center;
    background: #f0fdf4;
    border: 1px solid #bbf7d0;
    border-radius: 6px;
    box-sizing: border-box;
    color: #15803d;
    display: flex;
    font-size: 11.5px;
    font-weight: 600;
    justify-content: space-between;
    padding: 4px 10px;
    width: 100%;
  }

  .active-badge-group {
    align-items: center;
    display: flex;
    gap: 7px;
  }

  .active-pulse-dot {
    background: #10b981;
    border-radius: 50%;
    box-shadow: 0 0 0 2px rgb(16 185 129 / 25%);
    height: 7px;
    width: 7px;
  }

  :global(.deactivate-tray-btn) {
    align-items: center;
    background: #ffffff !important;
    border: 1px solid #86efac !important;
    border-radius: 4px;
    color: #15803d;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 10.5px;
    font-weight: 650;
    gap: 4px;
    height: 22px;
    outline: none;
    padding: 0 7px;
    transition: all 0.12s ease;
  }

  :global(.deactivate-tray-btn:hover) {
    background: #fef2f2 !important;
    border-color: #fca5a5 !important;
    color: #b91c1c;
  }

  .inactive-status-row {
    align-items: center;
    box-sizing: border-box;
    display: flex;
    justify-content: space-between;
    width: 100%;
  }

  .inactive-label {
    color: var(--color-boulder-500);
    font-size: 12px;
  }

  :global(.activate-action-btn) {
    align-items: center;
    background: #166534 !important;
    border: 0 !important;
    border-radius: 5px;
    box-shadow: 0 1px 2px rgb(0 0 0 / 10%);
    color: #ffffff !important;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 11px;
    font-weight: 650;
    gap: 5px;
    height: 26px;
    outline: none;
    padding: 0 10px;
    transition: opacity 0.12s ease;
  }

  :global(.activate-action-btn:hover) {
    opacity: 0.92;
  }

  :global(.activate-action-btn:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .search-section {
    align-items: center;
    background: #ffffff;
    border-bottom: 1px solid var(--color-boulder-200);
    box-sizing: border-box;
    display: flex;
    flex-shrink: 0;
    gap: 8px;
    padding: 8px 14px;
  }

  .search-input-wrapper {
    align-items: center;
    background: var(--color-boulder-100);
    border: 1px solid transparent;
    border-radius: 6px;
    box-sizing: border-box;
    display: flex;
    flex: 1;
    height: 30px;
    padding: 0 8px;
    transition: all 0.15s ease;
  }

  .search-input-wrapper:focus-within {
    background: #ffffff;
    border-color: var(--color-east-bay-400);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 14%);
  }

  :global(.search-icon) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
    margin-right: 6px;
  }

  .tray-search-input {
    background: transparent !important;
    border: 0 !important;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 12px;
    height: 100%;
    outline: none !important;
    padding: 0;
    width: 100%;
  }

  .clear-search-btn {
    align-items: center;
    background: transparent;
    border: 0;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: inline-flex;
    height: 18px;
    justify-content: center;
    outline: none;
    padding: 0;
    width: 18px;
  }

  .clear-search-btn:hover {
    color: var(--color-boulder-700);
  }

  :global(.add-var-btn) {
    align-items: center;
    background: var(--color-east-bay-900) !important;
    border: 0 !important;
    border-radius: 6px;
    color: #ffffff !important;
    cursor: pointer;
    display: inline-flex;
    flex-shrink: 0;
    font: inherit;
    font-size: 11.5px;
    font-weight: 650;
    gap: 4px;
    height: 30px;
    outline: none;
    padding: 0 10px;
    transition: opacity 0.15s ease;
    white-space: nowrap;
  }

  :global(.add-var-btn:hover) {
    opacity: 0.92;
  }

  :global(.add-var-btn:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .secrets-container {
    background: var(--color-boulder-50);
    display: flex;
    flex: 1;
    flex-direction: column;
    min-height: 0;
    overflow: hidden;
  }

  :global(.variables-scroll-area) {
    flex: 1;
    height: 100%;
    min-height: 0;
  }

  :global(.variables-viewport) {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 10px 14px;
  }

  .secret-card {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 7px;
    box-shadow: 0 1px 2px rgb(0 0 0 / 3%);
    display: flex;
    justify-content: space-between;
    padding: 8px 10px;
    transition: all 0.12s ease;
  }

  .secret-card:hover {
    border-color: var(--color-boulder-300);
    box-shadow: 0 1px 4px rgb(0 0 0 / 5%);
  }

  .secret-info {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .secret-key-line {
    align-items: center;
    display: flex;
    gap: 5px;
  }

  .secret-key-text {
    color: var(--color-boulder-950);
    font-family: var(--font-mono);
    font-size: 12px;
    font-weight: 700;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .copy-key-btn {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 3px;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: inline-flex;
    height: 18px;
    justify-content: center;
    outline: none;
    padding: 0;
    transition: all 0.1s ease;
    width: 18px;
  }

  .copy-key-btn:hover {
    background: var(--color-boulder-150, #ebebeb);
    color: var(--color-boulder-700);
  }

  .copy-key-btn.copied {
    color: #10b981;
  }

  .secret-val-line {
    min-width: 0;
  }

  .secret-val-text {
    color: var(--color-boulder-600);
    display: block;
    font-family: var(--font-mono);
    font-size: 11px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .secret-val-masked {
    color: var(--color-boulder-400);
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 2px;
  }

  .secret-card-actions {
    align-items: center;
    display: flex;
    flex-shrink: 0;
    gap: 3px;
    margin-left: 8px;
  }

  :global(.action-icon-btn) {
    align-items: center;
    background: transparent !important;
    border: 0 !important;
    border-radius: 4px;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: inline-flex;
    height: 24px;
    justify-content: center;
    outline: none;
    padding: 0;
    transition: all 0.1s ease;
    width: 24px;
  }

  :global(.action-icon-btn:hover) {
    background: var(--color-boulder-100) !important;
    color: var(--color-boulder-800);
  }

  :global(.action-icon-btn.copied) {
    color: #10b981;
    background: #dcfce7 !important;
  }

  :global(.dropdown-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    min-width: 140px;
    padding: 4px;
    z-index: 100;
  }

  :global(.dropdown-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: flex;
    font-size: 11.5px;
    font-weight: 550;
    gap: 7px;
    height: 28px;
    outline: none;
    padding: 0 8px;
    transition: background 0.1s ease;
  }

  :global(.dropdown-item:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  :global(.dropdown-item.destructive) {
    color: #b91c1c;
  }

  :global(.dropdown-item.destructive:hover) {
    background: #fef2f2;
    color: #dc2626;
  }

  :global(.dropdown-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 3px 0;
  }

  .empty-state {
    align-items: center;
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin: auto 0;
    padding: 36px 20px;
    text-align: center;
  }

  .empty-icon {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    border-radius: 50%;
    color: var(--color-east-bay-700);
    display: flex;
    height: 48px;
    justify-content: center;
    margin-bottom: 12px;
    width: 48px;
  }

  .empty-title {
    color: var(--color-boulder-950);
    font-size: 13.5px;
    font-weight: 650;
    margin: 0 0 4px;
  }

  .empty-subtitle {
    color: var(--color-boulder-500);
    font-size: 12px;
    line-height: 1.4;
    margin: 0 0 16px;
    max-width: 240px;
  }

  :global(.empty-add-btn) {
    align-items: center;
    background: var(--color-east-bay-900) !important;
    border: 0 !important;
    border-radius: 6px;
    color: #ffffff !important;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    gap: 5px;
    height: 30px;
    outline: none;
    padding: 0 12px;
  }

  .tray-footer {
    align-items: center;
    background: #ffffff;
    border-top: 1px solid var(--color-boulder-200);
    box-sizing: border-box;
    display: flex;
    height: 38px;
    justify-content: space-between;
    padding: 0 14px;
    flex-shrink: 0;
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

  :global(.footer-open-btn) {
    align-items: center;
    background: transparent !important;
    border: 0 !important;
    color: var(--color-east-bay-700);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
    outline: none;
    padding: 0;
    transition: color 0.12s ease;
  }

  :global(.footer-open-btn:hover) {
    color: var(--color-east-bay-950);
  }

  :global(.variables-scrollbar) {
    display: flex;
    padding: 2px;
    width: 6px;
  }

  :global(.variables-scrollbar-thumb) {
    background: var(--color-boulder-300);
    border-radius: 99px;
    flex: 1;
  }

  :global(.tooltip-content) {
    background: var(--color-boulder-950);
    border-radius: 5px;
    color: #ffffff;
    font-size: 11px;
    font-weight: 550;
    padding: 4px 8px;
    z-index: 150;
  }
</style>
