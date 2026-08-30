<script lang="ts">
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { emit, listen } from "@tauri-apps/api/event";
  import { Plus } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import { onMount } from "svelte";
  import DeleteConfirmationDialog from "$lib/features/secrets/DeleteConfirmationDialog.svelte";
  import ProductionActivationDialog from "$lib/features/secrets/ProductionActivationDialog.svelte";
  import ProfileEditor from "$lib/features/secrets/ProfileEditor.svelte";
  import ProfilesPanel from "$lib/features/secrets/ProfilesPanel.svelte";
  import type { Profile, SecretsConfiguration } from "$lib/features/secrets/types";

  const starterProfiles: Profile[] = [
    {
      id: 1,
      name: "Pruebas",
      isProduction: false,
      secrets: [{ id: 1, key: "API_URL", value: "https://api-pruebas.example.test" }]
    },
    {
      id: 2,
      name: "Production",
      isProduction: true,
      secrets: [
        { id: 2, key: "API_URL", value: "https://api.example.com" },
        { id: 3, key: "API_TOKEN", value: "replace-with-a-secret" }
      ]
    }
  ];
  const profileLoadTimeoutMs = 8_000;
  const isNativeApp = isTauri();

  let nextProfileId = 3;
  let nextSecretId = 4;
  let profiles = $state<Profile[]>([]);
  let activeProfileId = $state<number | null>(null);
  let selectedProfileId = $state<number | null>(null);
  let pendingProductionProfileId = $state<number | null>(null);
  let pendingProfileDeletionId = $state<number | null>(null);
  let pendingVariableDeletionId = $state<number | null>(null);
  let persistenceError = $state("");
  let isLoading = $state(true);
  let isSaving = $state(false);
  let isProductionDialogOpen = $state(false);
  let isProfileDeletionDialogOpen = $state(false);
  let isVariableDeletionDialogOpen = $state(false);
  let saveTimer: number | undefined;

  let selectedProfile = $derived(profiles.find((profile) => profile.id === selectedProfileId));

  onMount(() => {
    void loadConfiguration();

    const handleFocus = () => {
      void loadConfiguration();
    };
    window.addEventListener("focus", handleFocus);

    let unlisten: (() => void) | undefined;
    if (isNativeApp) {
      listen("secrets-updated", () => {
        void loadConfiguration();
      }).then((fn) => {
        unlisten = fn;
      });
    }

    return () => {
      window.clearTimeout(saveTimer);
      window.removeEventListener("focus", handleFocus);
      if (unlisten) unlisten();
    };
  });

  async function loadConfiguration() {
    if (!isNativeApp) {
      profiles = structuredClone(starterProfiles);
      activeProfileId = profiles[0]?.id ?? null;
      selectedProfileId = activeProfileId;
      isLoading = false;
      return;
    }

    try {
      const configuration = await loadProfilesWithTimeout();
      profiles = configuration.profiles.length > 0 ? configuration.profiles : structuredClone(starterProfiles);
      activeProfileId = configuration.activeProfileId ?? profiles[0]?.id ?? null;
      selectedProfileId = activeProfileId ?? profiles[0]?.id ?? null;
      nextProfileId = Math.max(0, ...profiles.map((profile) => profile.id)) + 1;
      nextSecretId = Math.max(0, ...profiles.flatMap((profile) => profile.secrets.map((secret) => secret.id))) + 1;
    } catch (error) {
      persistenceError = error instanceof Error ? error.message : String(error);
      profiles = structuredClone(starterProfiles);
      activeProfileId = profiles[0]?.id ?? null;
      selectedProfileId = activeProfileId;
    } finally {
      isLoading = false;
    }
  }

  function loadProfilesWithTimeout() {
    return new Promise<SecretsConfiguration>((resolve, reject) => {
      const timeout = window.setTimeout(
        () => reject(new Error("Loading secure profiles timed out. Check that Harbor is running and try again.")),
        profileLoadTimeoutMs
      );
      invoke<SecretsConfiguration>("load_secret_profiles")
        .then(resolve, reject)
        .finally(() => window.clearTimeout(timeout));
    });
  }

  function scheduleSave() {
    window.clearTimeout(saveTimer);
    saveTimer = window.setTimeout(() => void saveConfiguration(), 450);
  }

  async function saveConfiguration() {
    if (!isNativeApp) return;

    isSaving = true;
    persistenceError = "";
    try {
      await invoke("save_secret_profiles", { configuration: { profiles, activeProfileId } });
      await emit("secrets-updated");
    } catch (error) {
      persistenceError = error instanceof Error ? error.message : String(error);
    } finally {
      isSaving = false;
    }
  }

  function selectProfile(profileId: number) {
    selectedProfileId = profileId;
  }

  function reorderProfiles(reorderedProfiles: Profile[]) {
    profiles = reorderedProfiles;
    scheduleSave();
  }

  function createProfile() {
    const profile: Profile = { id: nextProfileId++, name: "New profile", isProduction: false, secrets: [] };
    profiles = [...profiles, profile];
    selectProfile(profile.id);
    scheduleSave();
  }

  function updateSelectedProfile(update: (profile: Profile) => Profile) {
    profiles = profiles.map((profile) => profile.id === selectedProfileId ? update(profile) : profile);
    scheduleSave();
  }

  function removeSelectedProfile() {
    if (profiles.length === 1 || !selectedProfile) return;

    const remainingProfiles = profiles.filter((profile) => profile.id !== selectedProfile.id);
    profiles = remainingProfiles;
    selectedProfileId = remainingProfiles[0].id;
    if (activeProfileId === selectedProfile.id) activeProfileId = remainingProfiles[0].id;
    scheduleSave();
  }

  function requestProfileDeletion() {
    if (!selectedProfile || profiles.length === 1) return;
    pendingProfileDeletionId = selectedProfile.id;
    isProfileDeletionDialogOpen = true;
  }

  function confirmProfileDeletion() {
    if (pendingProfileDeletionId === selectedProfileId) removeSelectedProfile();
    pendingProfileDeletionId = null;
    isProfileDeletionDialogOpen = false;
  }

  function requestVariableDeletion(secretId: number) {
    pendingVariableDeletionId = secretId;
    isVariableDeletionDialogOpen = true;
  }

  function confirmVariableDeletion() {
    const secretId = pendingVariableDeletionId;
    if (secretId !== null) {
      updateSelectedProfile((profile) => ({ ...profile, secrets: profile.secrets.filter((secret) => secret.id !== secretId) }));
    }
    pendingVariableDeletionId = null;
    isVariableDeletionDialogOpen = false;
  }

  async function requestActivation(profile: Profile) {
    if (profile.isProduction) {
      pendingProductionProfileId = profile.id;
      isProductionDialogOpen = true;
      return;
    }
    await activateProfile(profile.id);
  }

  async function confirmProductionActivation() {
    const profileId = pendingProductionProfileId;
    isProductionDialogOpen = false;
    pendingProductionProfileId = null;
    if (profileId !== null) await activateProfile(profileId);
  }

  async function activateProfile(profileId: number) {
    const previousProfileId = activeProfileId;
    activeProfileId = profileId;
    window.clearTimeout(saveTimer);
    persistenceError = "";
    try {
      await saveConfiguration();
      if (persistenceError) {
        activeProfileId = previousProfileId;
        return;
      }
      if (isNativeApp) {
        await invoke("activate_secret_profile_for_powershell", { profileId });
        await emit("secrets-updated");
      }
    } catch (error) {
      persistenceError = error instanceof Error ? error.message : String(error);
      activeProfileId = previousProfileId;
    }
  }
</script>

<svelte:head>
  <title>Harbor | Secrets</title>
  <meta name="description" content="Manage environment variables by Harbor profile." />
</svelte:head>

<main class="secrets-page" aria-labelledby="secrets-title">
  <header class="page-header">
    <div>
      <p class="eyebrow">Environment configuration</p>
      <h1 id="secrets-title">Secrets</h1>
      <p class="page-description">Create profiles for each environment and choose which configuration is active.</p>
    </div>
    <Button.Root class="primary-button" type="button" onclick={createProfile}>
      <Plus size={17} strokeWidth={2.2} aria-hidden="true" />
      <span>Add profile</span>
    </Button.Root>
  </header>

  {#if persistenceError}
    <p class="persistence-error" role="alert">{persistenceError}</p>
  {/if}

  {#if isLoading}
    <div class="loading-state" role="status">Loading secure profiles...</div>
  {:else}
    <div class="secrets-workspace">
      <ProfilesPanel
        {profiles}
        {activeProfileId}
        {selectedProfileId}
        onSelect={selectProfile}
        onReorder={reorderProfiles}
      />
      {#if selectedProfile}
        <ProfileEditor
          profile={selectedProfile}
          profileCount={profiles.length}
          isActive={activeProfileId === selectedProfile.id}
          onSaveSettings={(name, isProduction) => updateSelectedProfile((profile) => ({ ...profile, name, isProduction }))}
          onRequestDeleteProfile={requestProfileDeletion}
          onAddVariable={() => updateSelectedProfile((profile) => ({ ...profile, secrets: [...profile.secrets, { id: nextSecretId++, key: "", value: "" }] }))}
          onUpdateVariable={(secretId, field, value) => updateSelectedProfile((profile) => ({ ...profile, secrets: profile.secrets.map((secret) => secret.id === secretId ? { ...secret, [field]: value } : secret) }))}
          onRequestDeleteVariable={requestVariableDeletion}
          onUseProfile={() => void requestActivation(selectedProfile)}
        />
      {/if}
    </div>
  {/if}
</main>

<ProductionActivationDialog
  bind:open={isProductionDialogOpen}
  onOpenChange={(open) => { if (!open) pendingProductionProfileId = null; }}
  onConfirm={() => void confirmProductionActivation()}
/>

<DeleteConfirmationDialog
  bind:open={isProfileDeletionDialogOpen}
  title="Delete this profile?"
  description={`This removes ${selectedProfile?.name || "the selected profile"} and all of its environment variables.`}
  actionLabel="Delete profile"
  onOpenChange={(open) => { if (!open) pendingProfileDeletionId = null; }}
  onConfirm={confirmProfileDeletion}
/>

<DeleteConfirmationDialog
  bind:open={isVariableDeletionDialogOpen}
  title="Delete this variable?"
  description="This removes the environment variable from the selected profile."
  actionLabel="Delete variable"
  onOpenChange={(open) => { if (!open) pendingVariableDeletionId = null; }}
  onConfirm={confirmVariableDeletion}
/>

<style>
  .secrets-page {
    box-sizing: border-box;
    display: flex;
    flex: 1;
    flex-direction: column;
    margin: 0 auto;
    max-width: 1240px;
    padding: 32px;
    width: 100%;
  }

  .page-header {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }

  .eyebrow {
    color: var(--color-east-bay-700);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.08em;
    margin: 0 0 8px;
    text-transform: uppercase;
  }

  h1,
  p {
    margin: 0;
  }

  h1 {
    color: var(--color-boulder-950);
    font-size: 30px;
    font-weight: 650;
    line-height: 1.2;
  }

  .page-description {
    color: var(--color-boulder-600);
    font-size: 15px;
    line-height: 1.5;
    margin-top: 8px;
  }

  :global(.primary-button) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 6px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-weight: 650;
    gap: 8px;
    justify-content: center;
    min-height: 38px;
    padding: 0 14px;
  }

  :global(.primary-button:hover) {
    background: var(--color-east-bay-950);
  }

  .persistence-error {
    background: #fff3f1;
    border: 1px solid #efb5ad;
    border-radius: 6px;
    color: #913526;
    font-size: 13px;
    margin: 20px 0 0;
    padding: 10px 12px;
  }

  .loading-state {
    align-items: center;
    color: var(--color-boulder-600);
    display: flex;
    flex: 1;
    font-size: 14px;
    justify-content: center;
    min-height: 280px;
  }

  .secrets-workspace {
    align-items: start;
    display: grid;
    gap: 20px;
    grid-template-columns: minmax(230px, 280px) minmax(0, 1fr);
    margin-top: 30px;
  }

  @media (max-width: 820px) {
    .secrets-workspace {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 640px) {
    .secrets-page {
      padding: 24px 16px;
    }

    .page-header {
      align-items: flex-start;
      flex-direction: column;
      gap: 14px;
    }
  }
</style>
