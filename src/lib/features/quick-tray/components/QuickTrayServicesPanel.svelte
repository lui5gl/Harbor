<script lang="ts">
  import { Maximize2, Play, Square } from "@lucide/svelte";
  import type { QuickTrayServices } from "#lib/features/quick-tray/application/quickTrayServices.svelte";

  type Props = {
    services: QuickTrayServices;
    onOpenMainWindow: () => void;
  };

  let { services, onOpenMainWindow }: Props = $props();
</script>

<div class="flex min-h-0 flex-1 flex-col">
  <div class="flex min-h-0 flex-1 flex-col gap-2.5 overflow-y-auto p-3.5">
    <section
      class="flex flex-col gap-2.5 rounded-lg border border-boulder-200 bg-white px-3.5 py-3"
    >
      <header class="flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <span
            class="inline-flex h-7 w-8 items-center justify-center rounded-md bg-violet-100 text-[10px] font-extrabold text-violet-700"
            >PHP</span
          >
          <div>
            <h3 class="text-sm font-bold text-boulder-950">PHP</h3>
            <p class="text-[11px] text-boulder-500">FastCGI & CLI</p>
          </div>
        </div>
        <button
          class={`inline-flex h-7 cursor-pointer items-center gap-1 rounded-md px-2.5 text-[11.5px] font-semibold text-white transition-colors disabled:cursor-not-allowed disabled:opacity-50 ${services.isPhpRunning ? "bg-rose-600 hover:bg-rose-700" : "bg-east-bay-900 hover:bg-east-bay-950"}`}
          type="button"
          disabled={services.isBusy || services.php.installedVersions.length === 0}
          onclick={() => void services.togglePhp()}
          title={services.isPhpRunning ? "Detener FastCGI" : "Iniciar FastCGI"}
        >
          {#if services.isPhpRunning}<Square size={13} strokeWidth={2} /><span>Detener</span
            >{:else}<Play size={13} strokeWidth={2} /><span>Iniciar</span>{/if}
        </button>
      </header>
      <div class="flex flex-col gap-2 border-t border-boulder-100 pt-3">
        <div class="flex items-center justify-between text-xs">
          <span class="text-boulder-600">Versión CLI / FastCGI:</span>
          {#if services.php.installedVersions.length > 0}
            <select
              class="h-6.5 rounded-md border border-boulder-300 bg-boulder-50 px-1.5 text-xs font-semibold text-boulder-900 outline-none focus:border-east-bay-500"
              value={services.php.selectedVersion}
              disabled={services.isBusy}
              onchange={(event) => void services.selectPhpVersion(event.currentTarget.value)}
            >
              {#each services.php.installedVersions as version}<option value={version}
                  >{version}</option
                >{/each}
            </select>
          {:else}
            <span class="text-[11.5px] italic text-boulder-400">No instalado</span>
          {/if}
        </div>
        <div class="flex items-center">
          <span
            class={`inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10.5px] font-semibold ${services.isPhpRunning ? "bg-emerald-50 text-emerald-700" : "bg-boulder-100 text-boulder-600"}`}
          >
            <span class="size-1.5 rounded-full bg-current"></span>
            <span>{services.isPhpRunning ? "FastCGI 127.0.0.1:9070" : "Detenido"}</span>
          </span>
        </div>
      </div>
    </section>

    <section
      class="flex flex-col gap-2.5 rounded-lg border border-boulder-200 bg-white px-3.5 py-3"
    >
      <header class="flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <span
            class="inline-flex h-7 w-8 items-center justify-center rounded-md bg-emerald-50 text-[10px] font-extrabold text-emerald-700"
            >Node</span
          >
          <div>
            <h3 class="text-sm font-bold text-boulder-950">Node.js</h3>
            <p class="text-[11px] text-boulder-500">JavaScript Runtime</p>
          </div>
        </div>
      </header>
      <div class="flex flex-col gap-2 border-t border-boulder-100 pt-3">
        <div class="flex items-center justify-between text-xs">
          <span class="text-boulder-600">Versión activa:</span>
          {#if services.node.installedVersions.length > 0}
            <select
              class="h-6.5 rounded-md border border-boulder-300 bg-boulder-50 px-1.5 text-xs font-semibold text-boulder-900 outline-none focus:border-east-bay-500"
              value={services.node.selectedVersion}
              disabled={services.isBusy}
              onchange={(event) => void services.selectNodeVersion(event.currentTarget.value)}
            >
              {#each services.node.installedVersions as version}<option value={version}
                  >{version}</option
                >{/each}
            </select>
          {:else}
            <span class="text-[11.5px] italic text-boulder-400">No instalado</span>
          {/if}
        </div>
        <div class="flex items-center">
          <span
            class="inline-flex items-center gap-1 rounded-full bg-east-bay-50 px-2 py-0.5 text-[10.5px] font-semibold text-east-bay-800"
          >
            <span class="size-1.5 rounded-full bg-current"></span>
            <span
              >{services.node.selectedVersion
                ? `v${services.node.selectedVersion} listo`
                : "Sin versión"}</span
            >
          </span>
        </div>
      </div>
    </section>

    <section
      class="flex flex-col gap-2.5 rounded-lg border border-boulder-200 bg-white px-3.5 py-3"
    >
      <header class="flex items-center justify-between">
        <div class="flex items-center gap-2.5">
          <span
            class="inline-flex h-7 w-8 items-center justify-center rounded-md bg-amber-100 text-[10px] font-extrabold text-amber-700"
            >HTTP</span
          >
          <div>
            <h3 class="text-sm font-bold text-boulder-950">Apache</h3>
            <p class="text-[11px] text-boulder-500">Servidor Web</p>
          </div>
        </div>
      </header>
      <div class="border-t border-boulder-100 pt-3">
        <div class="flex items-center justify-between text-xs">
          <span class="text-boulder-600">Versión instalada:</span>
          {#if services.apache.installedVersions.length > 0}
            <span class="font-semibold text-boulder-800"
              >{services.apache.installedVersions[0]}</span
            >
          {:else}
            <span class="text-[11.5px] italic text-boulder-400">No instalado</span>
          {/if}
        </div>
      </div>
    </section>
  </div>

  <footer
    class="flex h-9.5 shrink-0 items-center justify-between border-t border-boulder-200 bg-white px-3.5"
  >
    <div class="flex items-center gap-1.5 text-[11.5px] text-boulder-500">
      <span class="size-1 rounded-full bg-boulder-400"></span>
      <span
        >{services.php.installedVersions.length +
          services.node.installedVersions.length +
          services.apache.installedVersions.length} instalados</span
      >
    </div>
    <button
      class="inline-flex cursor-pointer items-center gap-1.5 text-[11.5px] font-semibold text-east-bay-700 transition-colors hover:text-east-bay-950"
      type="button"
      onclick={onOpenMainWindow}
    >
      <span>Descargar más en Harbor</span>
      <Maximize2 size={11} strokeWidth={2.4} />
    </button>
  </footer>
</div>
