<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Check, Copy, Eye, EyeOff, Plus, ShieldAlert, Terminal, Trash2 } from "@lucide/svelte";
  import { onMount } from "svelte";

  type Secret = {
    id: number;
    key: string;
    value: string;
  };

  type Profile = {
    id: number;
    name: string;
    isProduction: boolean;
    secrets: Secret[];
  };

  type SecretsConfiguration = {
    profiles: Profile[];
    activeProfileId: number | null;
  };

  const starterProfiles: Profile[] = [
    {
      id: 1,
      name: "Pruebas",
      isProduction: false,
      secrets: [
        { id: 1, key: "API_URL", value: "https://api-pruebas.example.test" }
      ]
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

  let nextProfileId = 3;
  let nextSecretId = 4;
  let profiles = $state<Profile[]>([]);
  let activeProfileId = $state<number | null>(null);
  let selectedProfileId = $state<number | null>(null);
  let revealedSecretIds = $state<number[]>([]);
  let pendingProductionProfileId = $state<number | null>(null);
  let copiedSecretId = $state<number | null>(null);
  let persistenceError = $state("");
  let isLoading = $state(true);
  let isSaving = $state(false);
  let isActivating = $state(false);
  let saveTimer: number | undefined;

  let selectedProfile = $derived(profiles.find((profile) => profile.id === selectedProfileId));

  onMount(() => {
    void loadConfiguration();
    return () => window.clearTimeout(saveTimer);
  });

  async function loadConfiguration() {
    try {
      const configuration = await invoke<SecretsConfiguration>("load_secret_profiles");
      profiles = configuration.profiles.length > 0 ? configuration.profiles : structuredClone(starterProfiles);
      activeProfileId = configuration.activeProfileId ?? profiles[0]?.id ?? null;
      selectedProfileId = activeProfileId ?? profiles[0]?.id ?? null;
      nextProfileId = Math.max(0, ...profiles.map((profile) => profile.id)) + 1;
      nextSecretId = Math.max(0, ...profiles.flatMap((profile) => profile.secrets.map((secret) => secret.id))) + 1;
    } catch (error) {
      persistenceError = error instanceof Error ? error.message : String(error);
    } finally {
      isLoading = false;
    }
  }

  function scheduleSave() {
    window.clearTimeout(saveTimer);
    saveTimer = window.setTimeout(() => void saveConfiguration(), 450);
  }

  async function saveConfiguration() {
    isSaving = true;
    persistenceError = "";
    try {
      await invoke("save_secret_profiles", {
        configuration: { profiles, activeProfileId }
      });
    } catch (error) {
      persistenceError = error instanceof Error ? error.message : String(error);
    } finally {
      isSaving = false;
    }
  }

  function selectProfile(profileId: number) {
    selectedProfileId = profileId;
    revealedSecretIds = [];
  }

  function createProfile() {
    const profile: Profile = {
      id: nextProfileId++,
      name: "New profile",
      isProduction: false,
      secrets: []
    };
    profiles = [...profiles, profile];
    selectProfile(profile.id);
    scheduleSave();
  }

  function updateSelectedProfile(update: (profile: Profile) => Profile) {
    profiles = profiles.map((profile) => profile.id === selectedProfileId ? update(profile) : profile);
    scheduleSave();
  }

  function updateProfileName(name: string) {
    updateSelectedProfile((profile) => ({ ...profile, name }));
  }

  function updateProductionStatus(isProduction: boolean) {
    updateSelectedProfile((profile) => ({ ...profile, isProduction }));
  }

  function addSecret() {
    updateSelectedProfile((profile) => ({
      ...profile,
      secrets: [...profile.secrets, { id: nextSecretId++, key: "", value: "" }]
    }));
  }

  function updateSecret(secretId: number, field: "key" | "value", value: string) {
    updateSelectedProfile((profile) => ({
      ...profile,
      secrets: profile.secrets.map((secret) => secret.id === secretId ? { ...secret, [field]: value } : secret)
    }));
  }

  function removeSecret(secretId: number) {
    updateSelectedProfile((profile) => ({
      ...profile,
      secrets: profile.secrets.filter((secret) => secret.id !== secretId)
    }));
    revealedSecretIds = revealedSecretIds.filter((id) => id !== secretId);
  }

  function removeSelectedProfile() {
    if (profiles.length === 1 || !selectedProfile) return;

    const remainingProfiles = profiles.filter((profile) => profile.id !== selectedProfile.id);
    profiles = remainingProfiles;
    const nextSelectedProfile = remainingProfiles[0];
    selectedProfileId = nextSelectedProfile.id;
    if (activeProfileId === selectedProfile.id) activeProfileId = nextSelectedProfile.id;
    revealedSecretIds = [];
    scheduleSave();
  }

  function toggleSecretVisibility(secretId: number) {
    revealedSecretIds = revealedSecretIds.includes(secretId)
      ? revealedSecretIds.filter((id) => id !== secretId)
      : [...revealedSecretIds, secretId];
  }

  async function copySecret(secret: Secret) {
    try {
      await navigator.clipboard.writeText(secret.value);
      copiedSecretId = secret.id;
      window.setTimeout(() => {
        if (copiedSecretId === secret.id) copiedSecretId = null;
      }, 1500);
    } catch {
      copiedSecretId = null;
    }
  }

  async function requestActivation(profile: Profile) {
    if (profile.isProduction) {
      pendingProductionProfileId = profile.id;
      return;
    }
    const previousProfileId = activeProfileId;
    activeProfileId = profile.id;
    const activated = await activateProfileForPowerShell(profile.id);
    if (!activated) activeProfileId = previousProfileId;
  }

  async function confirmProductionActivation() {
    const profileId = pendingProductionProfileId;
    const previousProfileId = activeProfileId;
    if (profileId !== null) activeProfileId = profileId;
    pendingProductionProfileId = null;
    if (profileId !== null && !await activateProfileForPowerShell(profileId)) activeProfileId = previousProfileId;
  }

  async function activateProfileForPowerShell(profileId: number): Promise<boolean> {
    window.clearTimeout(saveTimer);
    isActivating = true;
    persistenceError = "";
    try {
      await saveConfiguration();
      if (persistenceError) return false;
      await invoke("activate_secret_profile_for_powershell", { profileId });
      return true;
    } catch (error) {
      persistenceError = error instanceof Error ? error.message : String(error);
      return false;
    } finally {
      isActivating = false;
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
    <button class="primary-button" type="button" onclick={createProfile}>
      <Plus size={17} strokeWidth={2.2} aria-hidden="true" />
      <span>Add profile</span>
    </button>
  </header>

  {#if persistenceError}
    <p class="persistence-error" role="alert">{persistenceError}</p>
  {/if}

  {#if isLoading}
    <div class="loading-state" role="status">Loading secure profiles...</div>
  {:else}
  <div class="secrets-workspace">
    <aside class="profiles-panel" aria-label="Environment profiles">
      <div class="panel-heading">
        <h2>Profiles</h2>
        <span>{profiles.length}</span>
      </div>
      <div class="profile-list">
        {#each profiles as profile (profile.id)}
          <button
            class:active={profile.id === selectedProfileId}
            class="profile-item"
            type="button"
            onclick={() => selectProfile(profile.id)}
          >
            <span class="profile-item-copy">
              <span class="profile-name-row">
                <span class="profile-name">{profile.name || "Untitled profile"}</span>
                {#if profile.isProduction}<span class="production-tag">Production</span>{/if}
              </span>
              <span class="profile-meta">{profile.secrets.length} {profile.secrets.length === 1 ? "variable" : "variables"}</span>
            </span>
            {#if activeProfileId === profile.id}<span class="active-tag">Active</span>{/if}
          </button>
        {/each}
      </div>
      <button class="secondary-button new-profile-button" type="button" onclick={createProfile}>
        <Plus size={16} strokeWidth={2.2} aria-hidden="true" />
        <span>New profile</span>
      </button>
    </aside>

    {#if selectedProfile}
      <section class="editor-panel" aria-labelledby="profile-editor-title">
        <div class="editor-header">
          <div>
            <p class="eyebrow">Selected profile</p>
            <h2 id="profile-editor-title">{selectedProfile.name || "Untitled profile"}</h2>
          </div>
          <button class="danger-button" type="button" disabled={profiles.length === 1} onclick={removeSelectedProfile}>
            <Trash2 size={16} strokeWidth={2} aria-hidden="true" />
            <span>Delete profile</span>
          </button>
        </div>

        <div class="profile-settings">
          <label class="field-label" for="profile-name">Profile name</label>
          <input
            id="profile-name"
            class="text-input"
            value={selectedProfile.name}
            oninput={(event) => updateProfileName(event.currentTarget.value)}
          />

          <label class="production-setting" for="production-profile">
            <span>
              <strong>Production profile</strong>
              <small>Requires confirmation before it can become active.</small>
            </span>
            <input
              id="production-profile"
              type="checkbox"
              checked={selectedProfile.isProduction}
              onchange={(event) => updateProductionStatus(event.currentTarget.checked)}
            />
          </label>
        </div>

        <div class="variables-header">
          <div>
            <h3>Environment variables</h3>
            <p>Values remain hidden until you choose to reveal them.</p>
          </div>
          <button class="secondary-button" type="button" onclick={addSecret}>
            <Plus size={16} strokeWidth={2.2} aria-hidden="true" />
            <span>Add variable</span>
          </button>
        </div>

        <div class="variables-table" aria-label="Environment variables">
          <div class="variable-table-header" aria-hidden="true">
            <span>Key</span>
            <span>Value</span>
            <span>Actions</span>
          </div>
          {#each selectedProfile.secrets as secret (secret.id)}
            <div class="variable-row">
              <input
                class="text-input secret-key"
                aria-label="Variable key"
                placeholder="VARIABLE_NAME"
                value={secret.key}
                oninput={(event) => updateSecret(secret.id, "key", event.currentTarget.value)}
              />
              <div class="secret-value-field">
                <input
                  class="text-input secret-value"
                  aria-label={`Value for ${secret.key || "environment variable"}`}
                  placeholder="Value"
                  type={revealedSecretIds.includes(secret.id) ? "text" : "password"}
                  value={secret.value}
                  oninput={(event) => updateSecret(secret.id, "value", event.currentTarget.value)}
                />
                <button class="icon-button" type="button" aria-label={revealedSecretIds.includes(secret.id) ? "Hide value" : "Reveal value"} onclick={() => toggleSecretVisibility(secret.id)}>
                  {#if revealedSecretIds.includes(secret.id)}
                    <EyeOff size={16} strokeWidth={2} aria-hidden="true" />
                  {:else}
                    <Eye size={16} strokeWidth={2} aria-hidden="true" />
                  {/if}
                </button>
              </div>
              <div class="row-actions">
                <button class="icon-button" type="button" aria-label={`Copy ${secret.key || "variable"}`} onclick={() => void copySecret(secret)}>
                  {#if copiedSecretId === secret.id}
                    <Check size={16} strokeWidth={2} aria-hidden="true" />
                  {:else}
                    <Copy size={16} strokeWidth={2} aria-hidden="true" />
                  {/if}
                </button>
                <button class="icon-button destructive" type="button" aria-label={`Delete ${secret.key || "variable"}`} onclick={() => removeSecret(secret.id)}>
                  <Trash2 size={16} strokeWidth={2} aria-hidden="true" />
                </button>
              </div>
            </div>
          {:else}
            <div class="empty-variables">No variables in this profile yet.</div>
          {/each}
        </div>

        <div class="editor-footer">
          <span>{isSaving ? "Saving securely..." : `${selectedProfile.secrets.length} ${selectedProfile.secrets.length === 1 ? "variable" : "variables"} configured`}</span>
          {#if activeProfileId === selectedProfile.id}
            <div class="active-profile-actions">
              <span class="active-profile-message"><Check size={16} strokeWidth={2.2} aria-hidden="true" /> Active profile</span>
              <button class="secondary-button" type="button" disabled={isActivating} onclick={() => void activateProfileForPowerShell(selectedProfile.id)}>
                <Terminal size={16} strokeWidth={2} aria-hidden="true" />
                <span>{isActivating ? "Activating..." : "Apply to new terminals"}</span>
              </button>
            </div>
          {:else}
            <button class="primary-button" type="button" onclick={() => void requestActivation(selectedProfile)}>Use this profile</button>
          {/if}
        </div>
        {#if activeProfileId === selectedProfile.id}
          <p class="terminal-status"><Terminal size={15} strokeWidth={2} aria-hidden="true" /> New PowerShell windows load this profile automatically and show its Harbor status.</p>
        {/if}
      </section>
    {/if}
  </div>
  {/if}
</main>

{#if pendingProductionProfileId !== null}
  <div class="modal-backdrop">
    <div class="confirmation-dialog" role="dialog" aria-modal="true" aria-labelledby="production-dialog-title">
      <div class="warning-icon" aria-hidden="true"><ShieldAlert size={22} strokeWidth={2} /></div>
      <h2 id="production-dialog-title">Activate production profile?</h2>
      <p>This may connect your development tools to production services. Confirm only when this is intentional.</p>
      <div class="dialog-actions">
        <button class="secondary-button" type="button" onclick={() => (pendingProductionProfileId = null)}>Cancel</button>
        <button class="primary-button warning-button" type="button" onclick={() => void confirmProductionActivation()}>Activate production</button>
      </div>
    </div>
  </div>
{/if}

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

  .page-header,
  .editor-header,
  .variables-header,
  .editor-footer,
  .panel-heading,
  .profile-name-row,
  .production-setting,
  .row-actions,
  .dialog-actions {
    align-items: center;
    display: flex;
  }

  .page-header,
  .editor-header,
  .variables-header,
  .editor-footer {
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
  h2,
  h3,
  p {
    margin: 0;
  }

  h1 {
    color: var(--color-boulder-950);
    font-size: 30px;
    font-weight: 650;
    line-height: 1.2;
  }

  h2 {
    color: var(--color-boulder-950);
    font-size: 19px;
    font-weight: 650;
  }

  h3 {
    color: var(--color-boulder-900);
    font-size: 14px;
    font-weight: 650;
  }

  .page-description,
  .variables-header p {
    color: var(--color-boulder-600);
    line-height: 1.5;
  }

  .page-description {
    font-size: 15px;
    margin-top: 8px;
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

  .variables-header p {
    font-size: 13px;
    margin-top: 4px;
  }

  .primary-button,
  .secondary-button,
  .danger-button,
  .icon-button {
    align-items: center;
    border: 0;
    border-radius: 6px;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-weight: 650;
    gap: 8px;
    justify-content: center;
  }

  .primary-button {
    background: var(--color-east-bay-900);
    color: #ffffff;
    min-height: 38px;
    padding: 0 14px;
  }

  .primary-button:hover {
    background: var(--color-east-bay-950);
  }

  .secondary-button {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
    min-height: 36px;
    padding: 0 12px;
  }

  .secondary-button:hover,
  .icon-button:hover {
    background: var(--color-boulder-200);
  }

  .danger-button {
    background: transparent;
    color: #a13f32;
    min-height: 34px;
    padding: 0 8px;
  }

  .danger-button:hover:not(:disabled) {
    background: #fff3f1;
  }

  .danger-button:disabled {
    cursor: not-allowed;
    opacity: 0.45;
  }

  .secrets-workspace {
    display: grid;
    flex: 1;
    gap: 20px;
    grid-template-columns: minmax(230px, 280px) minmax(0, 1fr);
    margin-top: 30px;
    min-height: 0;
  }

  .profiles-panel,
  .editor-panel {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
  }

  .profiles-panel {
    display: flex;
    flex-direction: column;
    min-height: 0;
    padding: 16px;
  }

  .panel-heading {
    color: var(--color-boulder-700);
    justify-content: space-between;
    margin-bottom: 12px;
    padding: 0 4px;
  }

  .panel-heading h2 {
    font-size: 13px;
  }

  .panel-heading span {
    background: var(--color-boulder-100);
    border-radius: 999px;
    color: var(--color-boulder-600);
    font-size: 11px;
    font-weight: 700;
    min-width: 22px;
    padding: 3px 6px;
    text-align: center;
  }

  .profile-list {
    display: grid;
    gap: 4px;
    overflow: auto;
  }

  .profile-item {
    align-items: center;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    color: inherit;
    cursor: pointer;
    display: flex;
    gap: 10px;
    justify-content: space-between;
    min-height: 62px;
    padding: 10px;
    text-align: left;
  }

  .profile-item:hover {
    background: var(--color-boulder-50);
  }

  .profile-item.active {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-200);
  }

  .profile-item-copy {
    display: grid;
    gap: 5px;
    min-width: 0;
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

  .new-profile-button {
    margin-top: auto;
    width: 100%;
  }

  .editor-panel {
    display: flex;
    flex-direction: column;
    min-width: 0;
    padding: 24px;
  }

  .profile-settings {
    border-bottom: 1px solid var(--color-boulder-200);
    display: grid;
    gap: 10px;
    grid-template-columns: minmax(0, 1fr) minmax(260px, 0.75fr);
    margin-top: 24px;
    padding-bottom: 24px;
  }

  .field-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 650;
    grid-column: 1;
  }

  .text-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 13px;
    min-height: 38px;
    min-width: 0;
    outline: none;
    padding: 0 10px;
    width: 100%;
  }

  .text-input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 16%);
  }

  #profile-name {
    grid-column: 1;
  }

  .production-setting {
    align-self: end;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    gap: 16px;
    grid-column: 2;
    justify-content: space-between;
    min-height: 38px;
    padding: 0 10px;
  }

  .production-setting span {
    display: grid;
    gap: 2px;
  }

  .production-setting strong {
    color: var(--color-boulder-800);
    font-size: 12px;
  }

  .production-setting small {
    color: var(--color-boulder-500);
    font-size: 11px;
  }

  .production-setting input {
    accent-color: var(--color-east-bay-700);
    cursor: pointer;
    height: 16px;
    width: 16px;
  }

  .variables-header {
    margin-top: 24px;
  }

  .variables-table {
    display: grid;
    gap: 8px;
    margin-top: 16px;
  }

  .variable-table-header,
  .variable-row {
    display: grid;
    gap: 10px;
    grid-template-columns: minmax(160px, 0.8fr) minmax(220px, 1.2fr) 76px;
  }

  .variable-table-header {
    color: var(--color-boulder-500);
    font-size: 11px;
    font-weight: 700;
    padding: 0 4px;
    text-transform: uppercase;
  }

  .variable-row {
    align-items: center;
  }

  .secret-key {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  }

  .secret-value-field {
    min-width: 0;
    position: relative;
  }

  .secret-value {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    padding-right: 40px;
  }

  .icon-button {
    background: transparent;
    color: var(--color-boulder-600);
    height: 34px;
    padding: 0;
    width: 34px;
  }

  .secret-value-field .icon-button {
    position: absolute;
    right: 2px;
    top: 2px;
  }

  .icon-button.destructive {
    color: #a13f32;
  }

  .icon-button.destructive:hover {
    background: #fff3f1;
  }

  .row-actions {
    gap: 4px;
  }

  .empty-variables {
    border: 1px dashed var(--color-boulder-300);
    border-radius: 6px;
    color: var(--color-boulder-500);
    font-size: 13px;
    padding: 28px;
    text-align: center;
  }

  .editor-footer {
    border-top: 1px solid var(--color-boulder-200);
    color: var(--color-boulder-500);
    font-size: 12px;
    margin-top: auto;
    padding-top: 22px;
  }

  .active-profile-message {
    align-items: center;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font-weight: 650;
    gap: 6px;
  }

  .active-profile-actions {
    align-items: center;
    display: flex;
    gap: 12px;
  }

  .terminal-status {
    align-items: center;
    color: var(--color-boulder-600);
    display: flex;
    font-size: 12px;
    gap: 6px;
    line-height: 1.4;
    margin: 16px 0 0;
  }

  .terminal-status :global(svg) {
    color: var(--color-east-bay-700);
    flex-shrink: 0;
  }

  .modal-backdrop {
    align-items: center;
    background: rgb(11 11 11 / 35%);
    display: flex;
    inset: 0;
    justify-content: center;
    padding: 24px;
    position: fixed;
    z-index: 10;
  }

  .confirmation-dialog {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 20px 60px rgb(11 11 11 / 20%);
    max-width: 420px;
    padding: 24px;
  }

  .warning-icon {
    align-items: center;
    background: #fff1dc;
    border-radius: 6px;
    color: #965d00;
    display: flex;
    height: 42px;
    justify-content: center;
    width: 42px;
  }

  .confirmation-dialog h2 {
    margin-top: 18px;
  }

  .confirmation-dialog p {
    color: var(--color-boulder-600);
    font-size: 14px;
    line-height: 1.5;
    margin-top: 10px;
  }

  .dialog-actions {
    gap: 10px;
    justify-content: flex-end;
    margin-top: 24px;
  }

  .warning-button {
    background: #9b5c00;
  }

  .warning-button:hover {
    background: #764600;
  }

  @media (max-width: 820px) {
    .secrets-workspace {
      grid-template-columns: 1fr;
    }

    .profiles-panel {
      min-height: auto;
    }

    .profile-list {
      grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
    }

    .new-profile-button {
      margin-top: 12px;
    }
  }

  @media (max-width: 640px) {
    .secrets-page {
      padding: 24px 16px;
    }

    .page-header,
    .editor-header,
    .variables-header,
    .editor-footer {
      align-items: flex-start;
      flex-direction: column;
      gap: 14px;
    }

    .profile-settings,
    .variable-table-header,
    .variable-row {
      grid-template-columns: 1fr;
    }

    .field-label,
    #profile-name,
    .production-setting {
      grid-column: 1;
    }

    .variable-table-header {
      display: none;
    }

    .variable-row {
      background: var(--color-boulder-50);
      border: 1px solid var(--color-boulder-200);
      border-radius: 6px;
      padding: 10px;
    }

    .row-actions {
      justify-content: flex-end;
    }

    .editor-panel {
      padding: 18px;
    }

    .active-profile-actions {
      align-items: flex-start;
      flex-direction: column;
    }
  }
</style>
