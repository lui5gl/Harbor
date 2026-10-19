<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Download, Loader2, X } from "@lucide/svelte";
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
  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!isInstalling) onSubmit();
  }
</script>

<Dialog.Root
  bind:open
  onOpenChange={(details) => {
    if (isInstalling && !details.open) open = true;
  }}
>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/40 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="w-full max-w-lg rounded-xl border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
    >
      <form class="flex flex-col gap-4.5" onsubmit={handleSubmit}>
        <div class="flex items-start justify-between gap-3">
          <div>
            <Dialog.Title class="text-base font-semibold text-boulder-950"
              >{editingProfile ? "Editar Perfil de Stack" : "Nuevo Perfil de Entorno"}</Dialog.Title
            ><Dialog.Description class="mt-1 text-[13px] leading-relaxed text-boulder-600"
              >Configura el par de PHP y Apache para este stack. Las versiones no instaladas se
              descargarán al guardar.</Dialog.Description
            >
          </div>
          <button
            class="inline-flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-md text-boulder-400 hover:bg-boulder-100 hover:text-boulder-800 disabled:opacity-40"
            type="button"
            disabled={isInstalling}
            onclick={() => (open = false)}
            aria-label="Cerrar"><X size={17} /></button
          >
        </div>
        {#if error}<div
            class="rounded-md border border-red-200 bg-red-50 px-3 py-2 text-[12.5px] text-red-700"
            role="alert"
          >
            {error}
          </div>{/if}
        <div class="flex flex-col gap-3.5">
          <label class="flex flex-col gap-1.5"
            ><span class="text-xs font-semibold text-boulder-700">Nombre del Perfil</span><input
              id="profile-name-input"
              type="text"
              class="h-9 w-full rounded-md border border-boulder-200 bg-white px-2.5 text-[13px] text-boulder-900 outline-none focus:border-east-bay-600 focus:ring-2 focus:ring-east-bay-600/10"
              placeholder="ej. Laravel 11 Project, Legacy PHP 7.4"
              bind:value={name}
            /></label
          >
          <div class="grid gap-3 sm:grid-cols-2">
            <label class="flex flex-col gap-1.5"
              ><span
                class="flex items-center justify-between text-xs font-semibold text-boulder-700"
                ><span>Versión de PHP</span>{#if isPhpMissing}<span
                    class="rounded bg-emerald-50 px-1.5 py-0.5 text-[10.5px] text-emerald-800"
                    >Se descargará</span
                  >{/if}</span
              ><select
                class="h-9 w-full rounded-md border border-boulder-200 bg-white px-2.5 text-[13px] text-boulder-900 outline-none focus:border-east-bay-600"
                bind:value={phpVersion}
                aria-label="Seleccionar versión de PHP"
                ><option value="" disabled>Seleccionar PHP</option
                >{#if installedPhpVersions.length}<optgroup label="Versiones instaladas"
                    >{#each installedPhpVersions as version}<option value={version}
                        >PHP {version} (Instalada)</option
                      >{/each}</optgroup
                  >{/if}{#if phpDownloadOptions.length}<optgroup label="Descargar e instalar"
                    >{#each phpDownloadOptions as option}<option value={option.version}
                        >{option.label}</option
                      >{/each}</optgroup
                  >{/if}</select
              ></label
            >
            <label class="flex flex-col gap-1.5"
              ><span
                class="flex items-center justify-between text-xs font-semibold text-boulder-700"
                ><span>Versión de Apache</span>{#if isApacheMissing}<span
                    class="rounded bg-emerald-50 px-1.5 py-0.5 text-[10.5px] text-emerald-800"
                    >Se descargará</span
                  >{/if}</span
              ><select
                class="h-9 w-full rounded-md border border-boulder-200 bg-white px-2.5 text-[13px] text-boulder-900 outline-none focus:border-east-bay-600"
                bind:value={apacheVersion}
                aria-label="Seleccionar versión de Apache"
                ><option value="" disabled>Seleccionar Apache</option
                >{#if installedApacheVersions.length}<optgroup label="Versiones instaladas"
                    >{#each installedApacheVersions as version}<option value={version}
                        >Apache {version} (Instalada)</option
                      >{/each}</optgroup
                  >{/if}{#if apacheDownloadOptions.length}<optgroup label="Descargar e instalar"
                    >{#each apacheDownloadOptions as option}<option value={option.version}
                        >{option.label}</option
                      >{/each}</optgroup
                  >{/if}</select
              ></label
            >
          </div>
          {#if isPhpMissing || isApacheMissing}<div
              class="flex items-center gap-2 rounded-md border border-emerald-200 bg-emerald-50 px-3 py-2 text-xs text-emerald-800"
            >
              <Download size={14} class="shrink-0" /><span
                >Las versiones marcadas se descargarán e instalarán automáticamente al guardar.</span
              >
            </div>{/if}
        </div>
        <div class="flex justify-end gap-2.5 pt-1.5">
          <button
            type="button"
            class="h-9 cursor-pointer rounded-md border border-boulder-200 bg-white px-3.5 text-[13px] font-semibold text-boulder-700 hover:bg-boulder-100 disabled:cursor-not-allowed disabled:opacity-50"
            disabled={isInstalling}
            onclick={() => (open = false)}>Cancelar</button
          ><button
            type="submit"
            class="inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3.5 text-[13px] font-semibold text-white hover:bg-east-bay-950 disabled:cursor-not-allowed disabled:opacity-50"
            disabled={isInstalling}
            >{#if isInstalling}<Loader2 size={14} class="animate-spin" /><span
                >Instalando... {installProgress > 0 ? `${installProgress}%` : ""}</span
              >{:else}<span>{editingProfile ? "Guardar Cambios" : "Crear Perfil"}</span
              >{/if}</button
          >
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
