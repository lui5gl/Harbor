<script lang="ts">
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
    Plus,
    Search,
    Trash2,
    X,
  } from "@lucide/svelte";
  import { Button, DropdownMenu, ScrollArea, Select, Tooltip } from "bits-ui";
  import type { Environment, Project, Secret } from "$lib/features/secrets/types";
  import type { createSecretsStore } from "$lib/features/secrets/application/secretsStore.svelte";

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
      ? selectedEnvironment.secrets.filter(
          (secret) =>
            secret.key.toLowerCase().includes(query) || secret.value.toLowerCase().includes(query),
        )
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
      <Select.Trigger class="profile-select-trigger" aria-label="Seleccionar entorno">
        <div class="select-label-wrapper">
          {#if isActive}<span class="active-dot" title="Entorno activo en el sistema"></span>{/if}
          <span class="select-profile-name"
            >{selectedEnvironment?.displayName || "Seleccionar entorno"}</span
          >
          {#if selectedEnvironment?.isProduction}<span class="prod-tag">Prod</span>{/if}
        </div>
        <ChevronDown size={14} strokeWidth={2.2} class="select-chevron" />
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
                        ? "active-dot"
                        : "dot-placeholder"}
                    ></span><span class="item-name">{environment.displayName}</span
                    >{#if environment.isProduction}<span class="prod-tag">Prod</span>{/if}
                  </div>
                  {#if selected}<Check size={14} strokeWidth={2.4} class="item-check" />{/if}
                {/snippet}
              </Select.Item>
            {/each}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
    <Button.Root
      class="secondary-button new-profile-btn"
      type="button"
      onclick={onCreateEnvironment}
      ><Plus size={14} strokeWidth={2.2} /><span>Nuevo</span></Button.Root
    >
  </div>
  <div class="activation-row">
    {#if isActive}<div class="active-status-tag">
        <span class="active-pulse"></span><span>Entorno activo en el sistema</span>
      </div>
    {:else}<div class="inactive-status-row">
        <span class="inactive-label">Este entorno no está activo</span><Button.Root
          class="activate-action-btn"
          type="button"
          disabled={isActivating}
          onclick={onActivate}
          ><Play size={11} strokeWidth={2.2} /><span
            >{isActivating ? "Activando..." : "Activar en shell"}</span
          ></Button.Root
        >
      </div>{/if}
  </div>
</section>

<section class="search-section">
  <div class="search-input-wrapper">
    <Search size={14} strokeWidth={2} class="search-icon" /><input
      type="text"
      placeholder="Buscar variable..."
      bind:value={searchQuery}
    />{#if searchQuery}<button
        class="clear-search-btn"
        type="button"
        onclick={() => (searchQuery = "")}
        aria-label="Limpiar búsqueda"><X size={12} strokeWidth={2.4} /></button
      >{/if}
  </div>
  <Button.Root class="add-var-btn" type="button" onclick={onOpenAddVariable}
    ><Plus size={13} strokeWidth={2.2} /><span>Variable</span></Button.Root
  >
</section>

<div class="secrets-container">
  {#if !selectedEnvironment || selectedEnvironment.secrets.length === 0}
    <div class="empty-state">
      <div class="empty-icon"><KeyRound size={22} strokeWidth={1.8} /></div>
      <p class="empty-title">Sin variables de entorno</p>
      <p class="empty-subtitle">
        Agrega variables a este entorno para administrarlas en el sistema.
      </p>
      <Button.Root class="secondary-button" type="button" onclick={onOpenAddVariable}
        ><Plus size={14} strokeWidth={2.2} /><span>Agregar primera variable</span></Button.Root
      >
    </div>
  {:else if filteredSecrets.length === 0}
    <div class="empty-state">
      <p class="empty-title">No hay resultados</p>
      <p class="empty-subtitle">No se encontraron variables coincidentes con "{searchQuery}".</p>
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
    <span class="footer-indicator"></span><span
      >{selectedEnvironment?.secrets.length ?? 0}
      {selectedEnvironment?.secrets.length === 1 ? "variable" : "variables"}</span
    >
  </div>
  <Button.Root class="footer-open-btn" type="button" onclick={onOpenMainWindow}
    ><span>Abrir Harbor completo</span><Maximize2 size={11} strokeWidth={2.4} /></Button.Root
  >
</footer>
