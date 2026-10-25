<script lang="ts">
  import { t } from "#lib/i18n";
  import { Menu } from "@ark-ui/svelte/menu";
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
    if (secrets.selectedEnvironmentId !== null) {
      selectedEnvironmentIdStr = String(secrets.selectedEnvironmentId);
    }
  });

  function selectEnvironment(value: string) {
    selectedEnvironmentIdStr = value;
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

<section class="flex shrink-0 flex-col gap-2 border-b border-boulder-200 bg-white px-3.5 py-2.5">
  <div class="flex w-full items-center gap-2">
    <div class="relative min-w-0 flex-1">
      <span
        class={`pointer-events-none absolute top-1/2 left-2.5 size-1.5 -translate-y-1/2 rounded-full ${isActive ? "bg-emerald-500 ring-2 ring-emerald-500/25" : "bg-boulder-400"}`}
      ></span>
      <select
        class="h-8 w-full cursor-pointer appearance-none truncate rounded-md border border-boulder-300 bg-white pr-8 pl-6 text-xs font-semibold text-boulder-950 outline-none transition-colors hover:border-boulder-400 hover:bg-boulder-50 focus:border-east-bay-400 focus:ring-2 focus:ring-east-bay-400/20"
        aria-label={t("secrets.quickTraySelectEnv")}
        value={selectedEnvironmentIdStr}
        onchange={(event) => selectEnvironment(event.currentTarget.value)}
      >
        {#if !selectedEnvironment}
          <option value="">{t("secrets.quickTraySelectEnv")}</option>
        {/if}
        {#each flatEnvironments as environment (environment.id)}
          <option value={String(environment.id)}>
            {environment.displayName}
          </option>
        {/each}
      </select>
      <ChevronDown
        size={13}
        strokeWidth={2.2}
        class="pointer-events-none absolute top-1/2 right-2.5 -translate-y-1/2 text-boulder-400"
      />
    </div>
    <button
      class="inline-flex h-8 shrink-0 cursor-pointer items-center gap-1.5 whitespace-nowrap rounded-md border border-boulder-300 bg-white px-2.5 text-xs font-semibold text-boulder-800 transition-colors hover:border-boulder-400 hover:bg-boulder-100"
      type="button"
      onclick={onCreateEnvironment}
    >
      <Plus size={13} strokeWidth={2.4} />
      <span>{t("secrets.quickTrayNew")}</span>
    </button>
  </div>

  <div class="flex min-h-7 items-center justify-between">
    {#if isActive}
      <div
        class="flex w-full items-center justify-between rounded-md border border-emerald-200 bg-emerald-50 px-2.5 py-1 text-[11.5px] font-semibold text-emerald-700"
      >
        <div class="flex items-center gap-2">
          <span class="size-1.5 rounded-full bg-emerald-500 ring-2 ring-emerald-500/25"></span>
          <span>{t("secrets.quickTrayActive")}</span>
        </div>
        <button
          class="inline-flex h-5.5 cursor-pointer items-center gap-1 rounded border border-emerald-300 bg-white px-1.5 text-[10.5px] font-semibold transition-colors hover:border-red-300 hover:bg-red-50 hover:text-red-700 disabled:cursor-not-allowed disabled:opacity-50"
          type="button"
          title={t("secrets.quickTrayDeactivate")}
          aria-label={t("secrets.quickTrayDeactivate")}
          disabled={secrets.isBusy}
          onclick={() => void secrets.deactivateEnvironment()}
        >
          <Power size={11} strokeWidth={2.4} />
          <span>{t("secrets.quickTrayDeactivate")}</span>
        </button>
      </div>
    {:else}
      <div class="flex w-full items-center justify-between">
        <span class="text-xs text-boulder-500">{t("secrets.quickTrayInactive")}</span>
        <button
          class="inline-flex h-6.5 cursor-pointer items-center gap-1 rounded-md bg-emerald-800 px-2.5 text-[11px] font-semibold text-white shadow-sm transition-colors hover:bg-emerald-900 disabled:cursor-not-allowed disabled:opacity-50"
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
        </button>
      </div>
    {/if}
  </div>
</section>

<section class="flex shrink-0 items-center gap-2 border-b border-boulder-200 bg-white px-3.5 py-2">
  <div
    class="flex h-7.5 min-w-0 flex-1 items-center rounded-md border border-transparent bg-boulder-100 px-2 transition-all focus-within:border-east-bay-400 focus-within:bg-white focus-within:ring-2 focus-within:ring-east-bay-400/15"
  >
    <Search size={14} strokeWidth={2} class="mr-1.5 shrink-0 text-boulder-400" />
    <input
      type="search"
      class="h-full min-w-0 flex-1 border-0 bg-transparent p-0 text-xs text-boulder-900 outline-none placeholder:text-boulder-400"
      placeholder={t("secrets.quickTrayFilter")}
      bind:value={searchQuery}
    />
    {#if searchQuery}
      <button
        class="inline-flex size-4.5 cursor-pointer items-center justify-center text-boulder-400 hover:text-boulder-700"
        type="button"
        onclick={() => (searchQuery = "")}
        aria-label={t("secrets.clearFilter")}
      >
        <X size={11} strokeWidth={2.4} />
      </button>
    {/if}
  </div>
  <button
    class="inline-flex h-7.5 shrink-0 cursor-pointer items-center gap-1 rounded-md bg-east-bay-900 px-2.5 text-[11.5px] font-semibold whitespace-nowrap text-white transition-colors hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
    type="button"
    disabled={isActive || !secrets.canWrite}
    onclick={onOpenAddVariable}
  >
    <Plus size={13} strokeWidth={2.4} />
    <span>{t("secrets.quickTrayAddVar")}</span>
  </button>
</section>

<div class="flex min-h-0 flex-1 flex-col overflow-hidden bg-boulder-50">
  {#if !selectedEnvironment || selectedEnvironment.secrets.length === 0}
    <div class="my-auto flex flex-col items-center justify-center px-5 py-9 text-center">
      <div
        class="mb-3 flex size-12 items-center justify-center rounded-full border border-east-bay-200 bg-east-bay-50 text-east-bay-700"
      >
        <KeyRound size={22} strokeWidth={1.8} />
      </div>
      <p class="mb-1 text-sm font-semibold text-boulder-950">{t("secrets.quickTrayNoVars")}</p>
      <p class="mb-4 max-w-60 text-xs leading-relaxed text-boulder-500">
        {t("secrets.quickTrayNoVarsDesc")}
      </p>
      <button
        class="inline-flex h-7.5 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-xs font-semibold text-white transition-colors hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
        type="button"
        disabled={!selectedEnvironment || isActive || !secrets.canWrite}
        onclick={onOpenAddVariable}
      >
        <Plus size={13} strokeWidth={2.4} />
        <span>{t("secrets.addFirstVariable")}</span>
      </button>
    </div>
  {:else if filteredSecrets.length === 0}
    <div class="my-auto flex flex-col items-center justify-center px-5 py-9 text-center">
      <p class="mb-1 text-sm font-semibold text-boulder-950">{t("secrets.quickTrayNoResults")}</p>
      <p class="max-w-60 text-xs leading-relaxed text-boulder-500">
        {t("secrets.noMatchFor")} "{searchQuery}".
      </p>
    </div>
  {:else}
    <div class="flex min-h-0 flex-1 flex-col gap-1.5 overflow-y-auto px-3.5 py-2.5">
      {#each filteredSecrets as secret (secret.id)}
        {@const isRevealed = revealedSecretIds.includes(secret.id)}
        {@const isCopied = copiedId === `val-${secret.id}`}
        {@const isKeyCopied = copiedId === `key-${secret.id}`}
        <div
          class="flex items-center justify-between rounded-md border border-boulder-200 bg-white px-2.5 py-2 shadow-sm transition-all hover:border-boulder-300 hover:shadow"
        >
          <div class="flex min-w-0 flex-1 flex-col gap-0.5">
            <div class="flex items-center gap-1">
              <span class="truncate font-mono text-xs font-bold text-boulder-950">{secret.key}</span
              >
              <button
                class={`inline-flex size-4.5 shrink-0 cursor-pointer items-center justify-center rounded-sm transition-colors ${isKeyCopied ? "bg-emerald-50 text-emerald-600" : "text-boulder-400 hover:bg-boulder-100 hover:text-boulder-700"}`}
                type="button"
                onclick={() => onCopy(secret.key, `key-${secret.id}`)}
                title="Copiar nombre de variable"
                aria-label="Copiar nombre de variable"
              >
                {#if isKeyCopied}<Check size={11} strokeWidth={2.4} />{:else}<Copy
                    size={11}
                    strokeWidth={2}
                  />{/if}
              </button>
            </div>
            <div class="min-w-0">
              {#if isRevealed}
                <span class="block truncate font-mono text-[11px] text-boulder-600"
                  >{secret.value || "<vacío>"}</span
                >
              {:else}
                <span class="font-mono text-[10px] tracking-[2px] text-boulder-400"
                  >••••••••••••</span
                >
              {/if}
            </div>
          </div>

          <div class="ml-2 flex shrink-0 items-center gap-0.5">
            <button
              class="inline-flex size-6 cursor-pointer items-center justify-center rounded text-boulder-400 transition-colors hover:bg-boulder-100 hover:text-boulder-800"
              type="button"
              title={isRevealed ? "Ocultar valor" : "Mostrar valor"}
              aria-label={isRevealed ? "Ocultar valor" : "Mostrar valor"}
              onclick={() => toggleReveal(secret.id)}
            >
              {#if isRevealed}<EyeOff size={14} strokeWidth={2} />{:else}<Eye
                  size={14}
                  strokeWidth={2}
                />{/if}
            </button>
            <button
              class={`inline-flex size-6 cursor-pointer items-center justify-center rounded transition-colors ${isCopied ? "bg-emerald-100 text-emerald-600" : "text-boulder-400 hover:bg-boulder-100 hover:text-boulder-800"}`}
              type="button"
              title={isCopied ? "Copiado" : "Copiar valor"}
              aria-label="Copiar valor"
              onclick={() => onCopy(secret.value, `val-${secret.id}`)}
            >
              {#if isCopied}<Check size={14} strokeWidth={2.4} />{:else}<Copy
                  size={14}
                  strokeWidth={2}
                />{/if}
            </button>
            <Menu.Root positioning={{ placement: "bottom-end", gutter: 4 }}>
              <Menu.Trigger
                class="inline-flex size-6 cursor-pointer items-center justify-center rounded text-boulder-400 transition-colors hover:bg-boulder-100 hover:text-boulder-800 data-open:bg-boulder-100 data-open:text-boulder-800"
                aria-label="Opciones"
              >
                <Ellipsis size={15} strokeWidth={2} />
              </Menu.Trigger>
              <Menu.Positioner>
                <Menu.Content
                  class="z-100 min-w-36 rounded-md border border-boulder-200 bg-white p-1 shadow-xl outline-none"
                >
                  <Menu.Item
                    value="edit"
                    class="flex h-7 cursor-pointer items-center gap-2 rounded px-2 text-[11.5px] font-medium text-boulder-800 outline-none data-highlighted:bg-boulder-100 data-highlighted:text-boulder-950"
                    onSelect={() => onOpenEditVariable(secret)}
                  >
                    <Pencil size={14} strokeWidth={2} />
                    <span>Editar variable</span>
                  </Menu.Item>
                  <Menu.Separator class="my-0.5 h-px bg-boulder-200" />
                  <Menu.Item
                    value="delete"
                    class="flex h-7 cursor-pointer items-center gap-2 rounded px-2 text-[11.5px] font-medium text-red-700 outline-none data-highlighted:bg-red-50 data-highlighted:text-red-600"
                    onSelect={() => onDeleteVariable(secret.id)}
                  >
                    <Trash2 size={14} strokeWidth={2} />
                    <span>Eliminar variable</span>
                  </Menu.Item>
                </Menu.Content>
              </Menu.Positioner>
            </Menu.Root>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<footer
  class="flex h-9.5 shrink-0 items-center justify-between border-t border-boulder-200 bg-white px-3.5"
>
  <div class="flex items-center gap-1.5 text-[11.5px] text-boulder-500">
    <span class="size-1 rounded-full bg-boulder-400"></span>
    <span>
      {selectedEnvironment?.secrets.length ?? 0}
      {selectedEnvironment?.secrets.length === 1 ? t("secrets.variable") : t("secrets.variables")}
    </span>
  </div>
  <button
    class="inline-flex cursor-pointer items-center gap-1.5 text-[11.5px] font-semibold text-east-bay-700 transition-colors hover:text-east-bay-950"
    type="button"
    onclick={onOpenMainWindow}
  >
    <span>{t("secrets.quickTrayOpenFull")}</span>
    <Maximize2 size={11} strokeWidth={2.4} />
  </button>
</footer>
