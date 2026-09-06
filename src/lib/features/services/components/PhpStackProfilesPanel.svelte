<script lang="ts">
  import { Check, Layers, Pencil, Plus, Server, Terminal, Trash2 } from "@lucide/svelte";
  import { Button, Tooltip } from "bits-ui";
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

<section class="profiles-card" aria-labelledby="profiles-title">
  <div class="card-header">
    <div class="card-title-group">
      <div class="card-icon layers-icon" aria-hidden="true">
        <Layers size={18} strokeWidth={2.2} />
      </div>
      <div>
        <h3 id="profiles-title">Perfiles de Entorno Web</h3>
        <p>Configura pares de PHP y Apache para alternar de stack según el proyecto.</p>
      </div>
    </div>
    <Button.Root class="primary-button-sm" type="button" onclick={onCreate}
      ><Plus size={14} strokeWidth={2.2} aria-hidden="true" /><span>Nuevo Perfil</span></Button.Root
    >
  </div>

  {#if profiles.length === 0}
    <div class="empty-runtime-box">
      <p>No tienes perfiles configurados.</p>
      <Button.Root class="primary-button-sm" type="button" onclick={onCreate}
        ><Plus size={14} strokeWidth={2.2} aria-hidden="true" /><span>Crear Primer Perfil</span
        ></Button.Root
      >
    </div>
  {:else}
    <div class="profiles-list">
      {#each profiles as profile (profile.id)}
        {@const isActive = profile.phpVersion === cleanActivePhp}
        <div
          role="button"
          tabindex="0"
          class={`profile-row${isActive ? " is-active" : ""}`}
          onclick={() => onSelect(profile)}
          onkeydown={(event) => {
            if (event.key === "Enter" || event.key === " ") {
              event.preventDefault();
              onSelect(profile);
            }
          }}
        >
          <div class="profile-radio" aria-hidden="true">
            <span class={`radio-circle${isActive ? " checked" : ""}`}></span>
          </div>
          <div class="profile-main-info">
            <div class="profile-title-row">
              <span class="profile-name">{profile.name}</span>{#if isActive}<span
                  class="active-badge"
                  ><Check size={11} strokeWidth={2.8} aria-hidden="true" /><span
                    >Activo · CLI & Web</span
                  ></span
                >{/if}
            </div>
            <div class="profile-stack-badges">
              <span class="stack-badge php"
                ><Terminal size={12} strokeWidth={2} /><span>PHP {profile.phpVersion}</span></span
              ><span class="stack-badge apache"
                ><Server size={12} strokeWidth={2} /><span>Apache {profile.apacheVersion}</span
                ></span
              >
            </div>
          </div>
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div class="profile-actions" onclick={(event) => event.stopPropagation()}>
            <Tooltip.Root
              ><Tooltip.Trigger
                class="action-btn edit-btn"
                type="button"
                aria-label={`Editar perfil ${profile.name}`}
                onclick={(event) => onEdit(profile, event)}
                ><Pencil size={13.5} strokeWidth={2.2} aria-hidden="true" /></Tooltip.Trigger
              ><Tooltip.Portal
                ><Tooltip.Content class="tooltip-content" sideOffset={6}
                  >Editar perfil</Tooltip.Content
                ></Tooltip.Portal
              ></Tooltip.Root
            >
            {#if profiles.length > 1}<Tooltip.Root
                ><Tooltip.Trigger
                  class="action-btn delete-btn"
                  type="button"
                  aria-label={`Eliminar perfil ${profile.name}`}
                  onclick={(event) => onDelete(profile.id, event)}
                  ><Trash2 size={13.5} strokeWidth={2.2} aria-hidden="true" /></Tooltip.Trigger
                ><Tooltip.Portal
                  ><Tooltip.Content class="tooltip-content" sideOffset={6}
                    >Eliminar perfil</Tooltip.Content
                  ></Tooltip.Portal
                ></Tooltip.Root
              >{/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</section>

<style>
  .profiles-card {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 18px 20px;
  }

  .card-header {
    align-items: flex-start;
    display: flex;
    gap: 12px;
    justify-content: space-between;
  }
  .card-title-group {
    align-items: flex-start;
    display: flex;
    gap: 12px;
  }
  .card-icon {
    align-items: center;
    border-radius: 8px;
    display: flex;
    flex-shrink: 0;
    height: 36px;
    justify-content: center;
    width: 36px;
  }
  .layers-icon {
    background: var(--color-east-bay-100);
    color: var(--color-east-bay-800);
  }
  .card-title-group h3 {
    color: var(--color-boulder-950);
    font-size: 15px;
    font-weight: 650;
    margin: 0;
  }
  .card-title-group p {
    color: var(--color-boulder-600);
    font-size: 12.5px;
    line-height: 1.4;
    margin: 3px 0 0;
  }
  .empty-runtime-box {
    align-items: center;
    background: #fff;
    border: 1px dashed var(--color-boulder-300);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 22px;
    text-align: center;
  }
  .empty-runtime-box p {
    color: var(--color-boulder-500);
    font-size: 13px;
    margin: 0;
  }
  .profiles-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .profile-row {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    gap: 14px;
    padding: 12px 16px;
    position: relative;
    transition:
      border-color 150ms ease,
      box-shadow 150ms ease,
      background-color 150ms ease;
  }
  .profile-row:hover {
    background: #fafafa;
    border-color: var(--color-boulder-300);
  }
  .profile-row.is-active {
    background: #fff;
    border-color: var(--color-east-bay-500);
    box-shadow: 0 1px 4px rgb(15 23 42 / 6%);
  }
  .profile-radio {
    display: flex;
    flex-shrink: 0;
  }
  .radio-circle {
    border: 2px solid var(--color-boulder-300);
    border-radius: 50%;
    box-sizing: border-box;
    display: block;
    height: 16px;
    position: relative;
    transition:
      border-color 150ms ease,
      background-color 150ms ease;
    width: 16px;
  }
  .profile-row:hover .radio-circle {
    border-color: var(--color-boulder-400);
  }
  .radio-circle.checked {
    background-color: var(--color-east-bay-900);
    border-color: var(--color-east-bay-900);
  }
  .radio-circle.checked::after {
    background: #fff;
    border-radius: 50%;
    content: "";
    height: 6px;
    left: 3px;
    position: absolute;
    top: 3px;
    width: 6px;
  }
  .profile-main-info {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 5px;
    min-width: 0;
  }
  .profile-title-row {
    align-items: center;
    display: flex;
    gap: 10px;
  }
  .profile-name {
    color: var(--color-boulder-950);
    font-size: 14px;
    font-weight: 650;
  }
  .active-badge {
    align-items: center;
    background: var(--color-east-bay-100);
    border-radius: 999px;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font-size: 11px;
    font-weight: 700;
    gap: 4px;
    letter-spacing: 0.02em;
    padding: 2px 8px;
  }
  .profile-stack-badges {
    align-items: center;
    display: flex;
    gap: 8px;
  }
  .stack-badge {
    align-items: center;
    border-radius: 4px;
    display: inline-flex;
    font-family: var(--font-mono);
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
    padding: 2px 7px;
  }
  .stack-badge.php {
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    color: var(--color-east-bay-800);
  }
  .stack-badge.apache {
    background: #fefce8;
    border: 1px solid #fef08a;
    color: #854d0e;
  }
  .profile-actions {
    align-items: center;
    display: flex;
    gap: 6px;
    margin-left: auto;
    opacity: 0.85;
    transition: opacity 150ms ease;
  }
  .profile-row:hover .profile-actions {
    opacity: 1;
  }
  :global(.action-btn) {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-600);
    cursor: pointer;
    display: inline-flex;
    height: 30px;
    justify-content: center;
    padding: 0;
    transition:
      background-color 150ms ease,
      border-color 150ms ease,
      color 150ms ease,
      transform 100ms ease;
    width: 30px;
  }
  :global(.action-btn:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-300);
    color: var(--color-boulder-900);
  }
  :global(.action-btn.delete-btn:hover) {
    background: #fef2f2;
    border-color: #fecaca;
    color: #dc2626;
  }
  :global(.action-btn:active) {
    transform: scale(0.95);
  }
</style>
