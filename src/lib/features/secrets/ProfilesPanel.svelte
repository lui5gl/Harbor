<script lang="ts">
  import { Check, ChevronDown, GripVertical, ListFilter, ListOrdered } from "@lucide/svelte";
  import { Button, ScrollArea, Select } from "bits-ui";
  import type { Profile, ProfileSort } from "./types";

  type ProfilesPanelProps = {
    profiles: Profile[];
    selectedProfileId: number | null;
    activeProfileId: number | null;
    onSelect: (profileId: number) => void;
    onReorder: (profiles: Profile[]) => void;
  };

  let { profiles, selectedProfileId, activeProfileId, onSelect, onReorder }: ProfilesPanelProps = $props();
  let profileSort = $state<ProfileSort>("manual");
  let isProfileSortMenuOpen = $state(false);
  let profileSortAnchor = $state<HTMLDivElement | null>(null);
  let draggedProfileId = $state<number | null>(null);
  let isReorderingProfiles = $state(false);

  const profileSortOptions: { value: ProfileSort; label: string }[] = [
    { value: "manual", label: "Manual" },
    { value: "production", label: "Production first" },
    { value: "name", label: "Name A-Z" }
  ];

  let sortedProfiles = $derived([...profiles].sort((left, right) => {
    if (profileSort === "production" && left.isProduction !== right.isProduction) {
      return left.isProduction ? -1 : 1;
    }
    if (profileSort === "manual") return 0;
    return left.name.localeCompare(right.name, undefined, { sensitivity: "base" });
  }));

  $effect(() => {
    if (profileSort !== "manual") isReorderingProfiles = false;
  });

  function toggleReordering() {
    isReorderingProfiles = !isReorderingProfiles;
    draggedProfileId = null;
  }

  function startProfileDrag(profileId: number, event: DragEvent) {
    if (!isReorderingProfiles) return;
    draggedProfileId = profileId;
    event.dataTransfer?.setData("text/plain", String(profileId));
    if (event.dataTransfer) event.dataTransfer.effectAllowed = "move";
  }

  function reorderProfile(targetProfileId: number, event: DragEvent) {
    event.preventDefault();
    const sourceProfileId = draggedProfileId ?? Number(event.dataTransfer?.getData("text/plain"));
    draggedProfileId = null;
    if (!isReorderingProfiles || !sourceProfileId || sourceProfileId === targetProfileId) return;

    const sourceIndex = profiles.findIndex((profile) => profile.id === sourceProfileId);
    const targetIndex = profiles.findIndex((profile) => profile.id === targetProfileId);
    if (sourceIndex < 0 || targetIndex < 0) return;

    const reorderedProfiles = [...profiles];
    const [movedProfile] = reorderedProfiles.splice(sourceIndex, 1);
    reorderedProfiles.splice(targetIndex, 0, movedProfile);
    onReorder(reorderedProfiles);
  }
</script>

<aside class="profiles-panel" aria-label="Environment profiles">
  <div class="panel-heading">
    <h2>Profiles</h2>
    <div class="profile-list-controls">
      <div class="profile-sort-anchor" bind:this={profileSortAnchor}>
        <Select.Root
          type="single"
          items={profileSortOptions}
          bind:value={profileSort}
          bind:open={isProfileSortMenuOpen}
        >
          <Select.Trigger class={`profile-sort-control${isProfileSortMenuOpen ? " open" : ""}`} aria-label="Order profiles">
            <ListFilter size={14} strokeWidth={2.2} aria-hidden="true" />
            <Select.Value>{profileSortOptions.find((option) => option.value === profileSort)?.label}</Select.Value>
            <ChevronDown size={14} strokeWidth={2} aria-hidden="true" />
          </Select.Trigger>
          <Select.Portal>
            <Select.Content class="profile-sort-content" customAnchor={profileSortAnchor} sideOffset={6}>
              <Select.Viewport>
                {#each profileSortOptions as option (option.value)}
                  <Select.Item class="profile-sort-item" value={option.value} label={option.label}>
                    {#snippet children({ selected })}
                      <span>{option.label}</span>
                      {#if selected}<Check size={15} strokeWidth={2.2} aria-hidden="true" />{/if}
                    {/snippet}
                  </Select.Item>
                {/each}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>
      {#if profileSort === "manual"}
        <button
          class={`reorder-toggle${isReorderingProfiles ? " active" : ""}`}
          type="button"
          aria-label={isReorderingProfiles ? "Finish reordering profiles" : "Reorder profiles"}
          aria-pressed={isReorderingProfiles}
          onclick={toggleReordering}
        >
          {#if isReorderingProfiles}
            <Check size={15} strokeWidth={2.4} aria-hidden="true" />
          {:else}
            <ListOrdered size={15} strokeWidth={2.1} aria-hidden="true" />
          {/if}
        </button>
      {/if}
      <span class="profile-count">{profiles.length}</span>
    </div>
  </div>

  {#snippet profileItems()}
    {#each sortedProfiles as profile (profile.id)}
      <Button.Root
        class={`profile-item${profile.id === selectedProfileId ? " active" : ""}${profile.id === draggedProfileId ? " dragging" : ""}${isReorderingProfiles ? " reordering" : ""}`}
        type="button"
        draggable={isReorderingProfiles}
        onclick={() => { if (!isReorderingProfiles) onSelect(profile.id); }}
        ondragstart={(event) => startProfileDrag(profile.id, event)}
        ondragend={() => (draggedProfileId = null)}
        ondragover={(event) => { if (isReorderingProfiles) event.preventDefault(); }}
        ondrop={(event) => reorderProfile(profile.id, event)}
      >
        {#if isReorderingProfiles}
          <span class="profile-drag-handle" aria-hidden="true">
            <GripVertical size={16} strokeWidth={2} />
          </span>
        {/if}
        <span class="profile-item-copy">
          <span class="profile-name-row">
            <span class="profile-name">{profile.name || "Untitled profile"}</span>
            {#if profile.isProduction}<span class="production-tag">Production</span>{/if}
          </span>
          <span class="profile-meta">{profile.secrets.length} {profile.secrets.length === 1 ? "variable" : "variables"}</span>
        </span>
        {#if activeProfileId === profile.id}<span class="active-tag">Active</span>{/if}
      </Button.Root>
    {/each}
  {/snippet}

  {#if profiles.length > 6}
    <ScrollArea.Root class="profile-scroll-area" type="auto">
      <ScrollArea.Viewport class="profile-list">
        {@render profileItems()}
      </ScrollArea.Viewport>
      <ScrollArea.Scrollbar class="profile-scrollbar" orientation="vertical">
        <ScrollArea.Thumb class="profile-scrollbar-thumb" />
      </ScrollArea.Scrollbar>
    </ScrollArea.Root>
  {:else}
    <div class="profile-list">
      {@render profileItems()}
    </div>
  {/if}

</aside>

<style>
  .profiles-panel {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    min-height: 0;
    padding: 16px;
  }

  .panel-heading,
  .profile-list-controls,
  .profile-name-row {
    align-items: center;
    display: flex;
  }

  .panel-heading {
    color: var(--color-boulder-700);
    justify-content: space-between;
    margin-bottom: 12px;
    padding: 0 4px;
  }

  .panel-heading h2 {
    color: var(--color-boulder-950);
    font-size: 13px;
    margin: 0;
  }

  .profile-list-controls {
    gap: 8px;
  }

  .reorder-toggle {
    align-items: center;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 5px;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    height: 28px;
    justify-content: center;
    padding: 0;
    width: 28px;
  }

  .reorder-toggle:hover {
    background: var(--color-boulder-100);
    color: var(--color-boulder-700);
  }

  .reorder-toggle.active {
    background: var(--color-east-bay-100);
    border-color: var(--color-east-bay-200);
    color: var(--color-east-bay-800);
  }

  .reorder-toggle:focus-visible {
    outline: 2px solid var(--color-east-bay-400);
    outline-offset: 2px;
  }

  .profile-count {
    background: var(--color-boulder-100);
    border-radius: 999px;
    color: var(--color-boulder-600);
    font-size: 11px;
    font-weight: 700;
    min-width: 22px;
    padding: 3px 6px;
    text-align: center;
  }

  .profile-sort-anchor {
    position: relative;
  }

  :global(.profile-sort-control) {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-600);
    cursor: pointer;
    display: inline-flex;
    gap: 4px;
    height: 28px;
    justify-content: center;
    padding: 0 6px;
  }

  :global(.profile-sort-control:hover) {
    background: var(--color-boulder-100);
  }

  :global(.profile-sort-control:focus-visible),
  :global(.profile-sort-control.open) {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 16%);
  }

  :global(.profile-sort-control span) {
    color: var(--color-boulder-700);
    font: inherit;
    font-size: 11px;
    font-weight: 650;
  }

  :global(.profile-sort-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 12px 28px rgb(11 11 11 / 14%);
    min-width: 166px;
    overflow: hidden;
    padding: 4px;
    z-index: 5;
  }

  :global(.profile-sort-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: flex;
    font-size: 12px;
    font-weight: 600;
    justify-content: space-between;
    min-height: 32px;
    outline: none;
    padding: 0 8px;
  }

  :global(.profile-sort-item[data-highlighted]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-900);
  }

  :global(.profile-list) {
    display: grid;
    gap: 2px;
    padding-right: 4px;
  }

  :global(.profile-scroll-area) {
    flex: 1;
    min-height: 0;
  }

  :global(.profile-scrollbar) {
    display: flex;
    padding: 2px;
    width: 8px;
  }

  :global(.profile-scrollbar-thumb) {
    background: var(--color-boulder-300);
    border-radius: 999px;
    flex: 1;
  }

  :global(.profile-item) {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: inherit;
    cursor: pointer;
    display: grid;
    gap: 8px;
    grid-template-columns: minmax(0, 1fr) auto;
    min-height: 54px;
    padding: 8px 10px;
    text-align: left;
    width: 100%;
  }

  :global(.profile-item.reordering) {
    grid-template-columns: 16px minmax(0, 1fr) auto;
  }

  :global(.profile-item[draggable="true"]) {
    cursor: grab;
  }

  :global(.profile-item.dragging) {
    cursor: grabbing;
    opacity: 0.5;
  }

  :global(.profile-item:hover) {
    background: var(--color-boulder-50);
  }

  :global(.profile-item.active) {
    background: var(--color-east-bay-50);
    box-shadow: inset 3px 0 0 var(--color-east-bay-500);
  }

  .profile-item-copy {
    display: grid;
    gap: 5px;
    min-width: 0;
  }

  .profile-drag-handle {
    align-items: center;
    color: var(--color-boulder-400);
    display: inline-flex;
    height: 20px;
    justify-content: center;
    width: 16px;
  }

  .profile-name-row {
    gap: 7px;
    min-width: 0;
  }

  .profile-name {
    color: var(--color-boulder-900);
    font-size: 13px;
    font-weight: 650;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .profile-meta {
    color: var(--color-boulder-500);
    font-size: 12px;
  }

  .production-tag,
  .active-tag {
    border-radius: 999px;
    font-size: 10px;
    font-weight: 700;
    padding: 3px 6px;
    white-space: nowrap;
  }

  .production-tag {
    background: #fff1dc;
    color: #965d00;
  }

  .active-tag {
    background: var(--color-east-bay-200);
    color: var(--color-east-bay-800);
  }

  @media (max-width: 820px) {
    .profiles-panel {
      min-height: auto;
    }

    :global(.profile-list) {
      grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
    }

  }
</style>
