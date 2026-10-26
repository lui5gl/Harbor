<script lang="ts">
  import { Check, Pencil, Plus, Trash2 } from "@lucide/svelte";
  import { cleanVersion, type PhpStackProfile } from "../types";
  type Props = {
    profiles: readonly PhpStackProfile[];
    activePhpVersion: string | null;
    onCreate: () => void;
    onSelect: (profile: PhpStackProfile) => void;
    onEdit: (profile: PhpStackProfile, event: MouseEvent) => void;
    onDelete: (id: string, event: MouseEvent) => void;
  };
  let { profiles, activePhpVersion, onCreate, onSelect, onEdit, onDelete }: Props = $props();
  let cleanActivePhp = $derived(activePhpVersion ? cleanVersion(activePhpVersion) : null);
</script>

<section
  class="flex flex-col gap-4 border-t border-boulder-100 pt-5"
  aria-labelledby="profiles-title"
>
  <div class="flex items-start justify-between gap-3">
    <div>
      <h3 id="profiles-title" class="text-[15px] font-semibold text-boulder-950">
        Perfiles de Entorno Web
      </h3>
      <p class="mt-1 text-[13px] leading-snug text-boulder-600">
        Configura pares de PHP y Apache para alternar de stack según el proyecto.
      </p>
    </div>
    <button
      class="inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-[12.5px] font-semibold text-white"
      type="button"
      onclick={onCreate}><Plus size={14} /><span>Nuevo Perfil</span></button
    >
  </div>
  {#if profiles.length === 0}<div
      class="flex flex-col items-center gap-2.5 rounded-lg border border-dashed border-boulder-300 p-6 text-center"
    >
      <p class="text-[13px] text-boulder-500">No tienes perfiles configurados.</p>
      <button
        class="inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3 text-[12.5px] font-semibold text-white"
        type="button"
        onclick={onCreate}><Plus size={14} /><span>Crear Primer Perfil</span></button
      >
    </div>{:else}
    <div
      class="flex flex-col divide-y divide-boulder-100 overflow-hidden rounded-lg border border-boulder-200"
    >
      {#each profiles as profile (profile.id)}{@const isActive =
          profile.phpVersion === cleanActivePhp}
        <div
          class={`flex items-center transition-colors ${isActive ? "bg-east-bay-50" : "hover:bg-boulder-50"}`}
        >
          <button
            type="button"
            class="flex min-w-0 flex-1 cursor-pointer items-center gap-3.5 px-4 py-3 text-left"
            onclick={() => onSelect(profile)}
          >
            <span
              class={`size-4 shrink-0 rounded-full border-2 ${isActive ? "border-east-bay-900 bg-east-bay-900 inset-ring-2 inset-ring-white" : "border-boulder-300"}`}
            ></span>
            <span class="flex min-w-0 flex-1 flex-col gap-1.5">
              <div class="flex items-center gap-2.5">
                <span class="text-sm font-semibold text-boulder-950">{profile.name}</span
                >{#if isActive}<span
                    class="inline-flex items-center gap-1 rounded-full bg-east-bay-100 px-2 py-0.5 text-[11px] font-bold text-east-bay-800"
                    ><Check size={11} /><span>Activo · CLI & Web</span></span
                  >{/if}
              </div>
              <span class="font-mono text-xs text-boulder-600"
                >PHP {profile.phpVersion} · Apache {profile.apacheVersion}</span
              >
            </span>
          </button>
          <div class="flex gap-1 pr-3">
            <button
              class="inline-flex size-7 cursor-pointer items-center justify-center rounded text-boulder-500 hover:bg-boulder-100 hover:text-boulder-800"
              type="button"
              title="Editar perfil"
              aria-label={`Editar perfil ${profile.name}`}
              onclick={(event) => onEdit(profile, event)}><Pencil size={14} /></button
            >{#if profiles.length > 1}<button
                class="inline-flex size-7 cursor-pointer items-center justify-center rounded text-boulder-500 hover:bg-red-50 hover:text-red-700"
                type="button"
                title="Eliminar perfil"
                aria-label={`Eliminar perfil ${profile.name}`}
                onclick={(event) => onDelete(profile.id, event)}><Trash2 size={14} /></button
              >{/if}
          </div>
        </div>{/each}
    </div>
  {/if}
</section>
