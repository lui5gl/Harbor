<script lang="ts">
  import { Button, ScrollArea } from "bits-ui";
  import type { Profile } from "./types";

  type ProfilesPanelProps = {
    profiles: Profile[];
    selectedProfileId: number | null;
    activeProfileId: number | null;
    onSelect: (profileId: number) => void;
    onReorder?: (profiles: Profile[]) => void;
  };

  let { profiles, selectedProfileId, activeProfileId, onSelect }: ProfilesPanelProps = $props();

  const sortByName = (a: Profile, b: Profile) =>
    a.name.localeCompare(b.name, undefined, { sensitivity: "base" });

  let productionProfiles = $derived(
    profiles.filter((p) => p.isProduction).sort(sortByName)
  );

  let developmentProfiles = $derived(
    profiles.filter((p) => !p.isProduction).sort(sortByName)
  );
</script>

<aside class="profiles-panel" aria-label="Environment profiles">
  <div class="panel-header">
    <div class="header-title-row">
      <h2>Profiles</h2>
      <span class="profile-count" title="{profiles.length} profiles">{profiles.length}</span>
    </div>
  </div>

  <ScrollArea.Root class="profile-scroll-area" type="auto">
    <ScrollArea.Viewport class="profile-list-viewport">
      {#if profiles.length === 0}
        <div class="empty-state">No profiles found</div>
      {:else}
        <div class="profile-sections">
          {#if productionProfiles.length > 0}
            <section class="section-group" aria-label="Production profiles">
              <div class="section-heading">
                <span class="section-title">Production</span>
                <span class="section-badge">{productionProfiles.length}</span>
              </div>
              <div class="profile-list">
                {#each productionProfiles as profile (profile.id)}
                  {@render profileItem(profile)}
                {/each}
              </div>
            </section>
          {/if}

          {#if developmentProfiles.length > 0}
            <section class="section-group" aria-label="Development profiles">
              <div class="section-heading">
                <span class="section-title">Development & Testing</span>
                <span class="section-badge">{developmentProfiles.length}</span>
              </div>
              <div class="profile-list">
                {#each developmentProfiles as profile (profile.id)}
                  {@render profileItem(profile)}
                {/each}
              </div>
            </section>
          {/if}
        </div>
      {/if}
    </ScrollArea.Viewport>
    <ScrollArea.Scrollbar class="profile-scrollbar" orientation="vertical">
      <ScrollArea.Thumb class="profile-scrollbar-thumb" />
    </ScrollArea.Scrollbar>
  </ScrollArea.Root>
</aside>

{#snippet profileItem(profile: Profile)}
  {@const isSelected = profile.id === selectedProfileId}
  {@const isActive = profile.id === activeProfileId}

  <Button.Root
    class={`profile-item${isSelected ? " selected" : ""}${isActive ? " is-active" : ""}`}
    type="button"
    onclick={() => onSelect(profile.id)}
  >
    <div class="profile-item-body">
      <div class="profile-primary-row">
        <span class="profile-name" title={profile.name || "Untitled profile"}>
          {profile.name || "Untitled profile"}
        </span>
        {#if isActive}
          <span class="active-badge">Active</span>
        {/if}
      </div>

      <div class="profile-secondary-row">
        <span class="profile-meta">
          {profile.secrets.length} {profile.secrets.length === 1 ? "variable" : "variables"}
        </span>
      </div>
    </div>
  </Button.Root>
{/snippet}

<style>
  .profiles-panel {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    height: 100%;
    max-height: 560px;
    padding: 14px;
  }

  .panel-header {
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
    display: flex;
    justify-content: space-between;
    margin-bottom: 12px;
    padding-bottom: 10px;
  }

  .header-title-row {
    align-items: center;
    display: flex;
    gap: 8px;
  }

  .panel-header h2 {
    color: var(--color-boulder-950);
    font-size: 13px;
    font-weight: 700;
    letter-spacing: -0.01em;
    margin: 0;
  }

  .profile-count {
    background: var(--color-boulder-100);
    border-radius: 999px;
    color: var(--color-boulder-600);
    font-size: 11px;
    font-weight: 700;
    line-height: 1;
    padding: 2px 6px;
  }

  :global(.profile-scroll-area) {
    flex: 1;
    min-height: 0;
  }

  :global(.profile-list-viewport) {
    height: 100%;
    width: 100%;
  }

  .profile-sections {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-right: 2px;
  }

  .section-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .section-heading {
    align-items: center;
    display: flex;
    justify-content: space-between;
    padding: 0 4px;
  }

  .section-title {
    color: var(--color-boulder-500);
    font-size: 10.5px;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .section-badge {
    color: var(--color-boulder-400);
    font-size: 10px;
    font-weight: 600;
  }

  .profile-list {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .empty-state {
    color: var(--color-boulder-400);
    font-size: 12px;
    padding: 24px 8px;
    text-align: center;
  }

  :global(.profile-item) {
    align-items: center;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    box-sizing: border-box;
    color: inherit;
    cursor: pointer;
    display: flex;
    padding: 8px 10px;
    text-align: left;
    transition: background 0.12s ease, border-color 0.12s ease;
    width: 100%;
  }

  :global(.profile-item:hover) {
    background: var(--color-boulder-50);
    border-color: var(--color-boulder-200);
  }

  :global(.profile-item.selected) {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-300);
  }

  :global(.profile-item.selected .profile-name) {
    color: var(--color-east-bay-950);
  }

  .profile-item-body {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .profile-primary-row {
    align-items: center;
    display: flex;
    gap: 6px;
    justify-content: space-between;
    min-width: 0;
  }

  .profile-name {
    color: var(--color-boulder-900);
    font-size: 12.5px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .profile-secondary-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }

  .profile-meta {
    color: var(--color-boulder-500);
    font-size: 11px;
    font-weight: 500;
  }

  .active-badge {
    background: var(--color-east-bay-100);
    border: 1px solid var(--color-east-bay-200);
    border-radius: 4px;
    color: var(--color-east-bay-800);
    flex-shrink: 0;
    font-size: 9.5px;
    font-weight: 700;
    letter-spacing: 0.02em;
    line-height: 1;
    padding: 2px 5px;
    text-transform: uppercase;
  }

  :global(.profile-scrollbar) {
    display: flex;
    padding: 2px;
    user-select: none;
    width: 6px;
  }

  :global(.profile-scrollbar-thumb) {
    background: var(--color-boulder-300);
    border-radius: 999px;
    flex: 1;
    transition: background 0.15s ease;
  }

  :global(.profile-scrollbar-thumb:hover) {
    background: var(--color-boulder-400);
  }
</style>
