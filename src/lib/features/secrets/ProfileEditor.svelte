<script lang="ts">
  import { Check, Copy, Ellipsis, Eye, EyeOff, Pencil, Plus, Trash2 } from "@lucide/svelte";
  import { Button, DropdownMenu, ScrollArea, Tooltip } from "bits-ui";
  import ProfileSettingsDialog from "./ProfileSettingsDialog.svelte";
  import type { Profile, Secret } from "./types";

  type ProfileEditorProps = {
    profile: Profile;
    profileCount: number;
    isActive: boolean;
    onSaveSettings: (name: string, isProduction: boolean) => void;
    onRequestDeleteProfile: () => void;
    onAddVariable: () => void;
    onUpdateVariable: (secretId: number, field: "key" | "value", value: string) => void;
    onRequestDeleteVariable: (secretId: number) => void;
    onUseProfile: () => void;
  };

  let {
    profile,
    profileCount,
    isActive,
    onSaveSettings,
    onRequestDeleteProfile,
    onAddVariable,
    onUpdateVariable,
    onRequestDeleteVariable,
    onUseProfile
  }: ProfileEditorProps = $props();

  let revealedSecretIds = $state<number[]>([]);
  let copiedSecretId = $state<number | null>(null);
  let isProfileActionsOpen = $state(false);
  let isSettingsDialogOpen = $state(false);

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

  function splitPastedAssignment(secret: Secret, event: ClipboardEvent) {
    const pastedText = event.clipboardData?.getData("text") ?? "";
    const separatorIndex = pastedText.indexOf("=");
    if (separatorIndex <= 0) return;

    const key = pastedText.slice(0, separatorIndex).trim();
    if (!key) return;

    event.preventDefault();
    onUpdateVariable(secret.id, "key", key);
    onUpdateVariable(secret.id, "value", pastedText.slice(separatorIndex + 1));
  }
</script>

<section class="editor-panel" aria-labelledby="profile-editor-title">
  <div class="editor-header">
    <div>
      <p class="eyebrow">Selected profile</p>
      <h2 id="profile-editor-title">{profile.name || "Untitled profile"}</h2>
    </div>
    <div class="profile-actions">
      <DropdownMenu.Root bind:open={isProfileActionsOpen}>
        <DropdownMenu.Trigger class="icon-button" type="button" aria-label="Profile actions">
          <Ellipsis size={18} strokeWidth={2} aria-hidden="true" />
        </DropdownMenu.Trigger>
        <DropdownMenu.Portal>
          <DropdownMenu.Content class="profile-actions-menu" sideOffset={6} align="end">
            <DropdownMenu.Item class="profile-actions-menu-item" onclick={() => (isSettingsDialogOpen = true)}>
              <Pencil size={15} strokeWidth={2} aria-hidden="true" />
              <span>Edit settings</span>
            </DropdownMenu.Item>
            <DropdownMenu.Separator class="profile-actions-menu-separator" />
            <DropdownMenu.Item class="profile-actions-menu-item destructive" disabled={profileCount === 1} onclick={onRequestDeleteProfile}>
              <Trash2 size={15} strokeWidth={2} aria-hidden="true" />
              <span>Delete profile</span>
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Portal>
      </DropdownMenu.Root>
      <ProfileSettingsDialog bind:open={isSettingsDialogOpen} name={profile.name} isProduction={profile.isProduction} onSave={onSaveSettings} />
    </div>
  </div>

  <div class="variables-header">
    <div>
      <h3>Environment variables</h3>
      <p>Values remain hidden until you choose to reveal them.</p>
    </div>
    <Button.Root class="secondary-button" type="button" onclick={onAddVariable}>
      <Plus size={16} strokeWidth={2.2} aria-hidden="true" />
      <span>Add variable</span>
    </Button.Root>
  </div>

  <ScrollArea.Root class="variables-scroll-area" type="auto">
    <ScrollArea.Viewport class="variables-table" aria-label="Environment variables">
      <div class="variable-table-header" aria-hidden="true">
        <span>Key</span>
        <span>Value</span>
        <span>Actions</span>
      </div>
      {#each profile.secrets as secret (secret.id)}
        <div class="variable-row">
          <input
            class="text-input secret-key"
            aria-label="Variable key"
            placeholder="VARIABLE_NAME"
            value={secret.key}
            oninput={(event) => onUpdateVariable(secret.id, "key", event.currentTarget.value)}
            onpaste={(event) => splitPastedAssignment(secret, event)}
          />
          <div class="secret-value-field">
            <input
              class="text-input secret-value"
              aria-label={`Value for ${secret.key || "environment variable"}`}
              placeholder="Value"
              type={revealedSecretIds.includes(secret.id) ? "text" : "password"}
              value={secret.value}
              oninput={(event) => onUpdateVariable(secret.id, "value", event.currentTarget.value)}
            />
            <Tooltip.Root>
              <Tooltip.Trigger class="icon-button" type="button" aria-label={revealedSecretIds.includes(secret.id) ? "Hide value" : "Reveal value"} onclick={() => toggleSecretVisibility(secret.id)}>
                {#if revealedSecretIds.includes(secret.id)}
                  <EyeOff size={16} strokeWidth={2} aria-hidden="true" />
                {:else}
                  <Eye size={16} strokeWidth={2} aria-hidden="true" />
                {/if}
              </Tooltip.Trigger>
              <Tooltip.Portal><Tooltip.Content class="tooltip-content" sideOffset={6}>{revealedSecretIds.includes(secret.id) ? "Hide value" : "Reveal value"}</Tooltip.Content></Tooltip.Portal>
            </Tooltip.Root>
          </div>
          <div class="row-actions">
            <DropdownMenu.Root>
              <DropdownMenu.Trigger class="icon-button" type="button" aria-label={`Actions for ${secret.key || "variable"}`}>
                <Ellipsis size={18} strokeWidth={2} aria-hidden="true" />
              </DropdownMenu.Trigger>
              <DropdownMenu.Portal>
                <DropdownMenu.Content class="profile-actions-menu" sideOffset={6} align="end">
                  <DropdownMenu.Item class="profile-actions-menu-item" onclick={() => void copySecret(secret)}>
                    {#if copiedSecretId === secret.id}
                      <Check size={15} strokeWidth={2} aria-hidden="true" />
                      <span>Copied</span>
                    {:else}
                      <Copy size={15} strokeWidth={2} aria-hidden="true" />
                      <span>Copy value</span>
                    {/if}
                  </DropdownMenu.Item>
                  <DropdownMenu.Separator class="profile-actions-menu-separator" />
                  <DropdownMenu.Item class="profile-actions-menu-item destructive" onclick={() => onRequestDeleteVariable(secret.id)}>
                    <Trash2 size={15} strokeWidth={2} aria-hidden="true" />
                    <span>Delete variable</span>
                  </DropdownMenu.Item>
                </DropdownMenu.Content>
              </DropdownMenu.Portal>
            </DropdownMenu.Root>
          </div>
        </div>
      {:else}
        <div class="empty-variables">No variables in this profile yet.</div>
      {/each}
    </ScrollArea.Viewport>
    <ScrollArea.Scrollbar class="variables-scrollbar" orientation="vertical">
      <ScrollArea.Thumb class="variables-scrollbar-thumb" />
    </ScrollArea.Scrollbar>
  </ScrollArea.Root>

  {#if !isActive}
    <div class="editor-footer">
      <Button.Root class="primary-button" type="button" onclick={onUseProfile}>Use this profile</Button.Root>
    </div>
  {/if}
</section>

<style>
  .editor-panel {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    min-width: 0;
    padding: 24px;
    width: 100%;
  }

  .editor-header,
  .variables-header,
  .editor-footer,
  .profile-actions,
  .row-actions {
    align-items: center;
    display: flex;
  }

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

  h2,
  h3,
  p {
    margin: 0;
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

  :global(.primary-button),
  :global(.secondary-button),
  :global(.danger-button),
  :global(.icon-button) {
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

  :global(.primary-button) {
    background: var(--color-east-bay-900);
    color: #ffffff;
    min-height: 38px;
    padding: 0 14px;
  }

  :global(.primary-button:hover) {
    background: var(--color-east-bay-950);
  }

  :global(.secondary-button) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
    min-height: 36px;
    padding: 0 12px;
  }

  :global(.secondary-button:hover),
  :global(.icon-button:hover) {
    background: var(--color-boulder-200);
  }

  :global(.danger-button) {
    background: transparent;
    color: #a13f32;
    min-height: 34px;
    padding: 0 8px;
  }

  :global(.danger-button:hover:not(:disabled)) {
    background: #fff3f1;
  }

  :global(.danger-button:disabled) {
    cursor: not-allowed;
    opacity: 0.45;
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

  .variables-header {
    border-top: 1px solid var(--color-boulder-200);
    margin-top: 24px;
    padding-top: 24px;
  }

  .variables-header p {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.5;
    margin-top: 4px;
  }

  :global(.variables-table) {
    display: grid;
    gap: 0;
    margin-top: 16px;
    padding-right: 4px;
  }

  :global(.variables-scroll-area) {
    flex: 1;
    min-height: 0;
  }

  :global(.variables-scrollbar) {
    display: flex;
    padding: 2px;
    width: 8px;
  }

  :global(.variables-scrollbar-thumb) {
    background: var(--color-boulder-300);
    border-radius: 999px;
    flex: 1;
  }

  .variable-table-header,
  .variable-row {
    display: grid;
    column-gap: 12px;
    grid-template-columns: minmax(180px, 0.8fr) minmax(260px, 1.2fr) 34px;
  }

  .variable-table-header {
    color: var(--color-boulder-500);
    font-size: 11px;
    font-weight: 700;
    padding: 0 4px 8px;
    text-transform: uppercase;
  }

  .variable-row {
    align-items: center;
    border-top: 1px solid var(--color-boulder-200);
    min-height: 62px;
    padding: 10px 4px;
  }

  .secret-key,
  .secret-value {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  }

  .secret-value-field {
    min-width: 0;
    position: relative;
  }

  .secret-value {
    padding-right: 40px;
  }

  :global(.icon-button) {
    background: transparent;
    color: var(--color-boulder-600);
    height: 34px;
    padding: 0;
    width: 34px;
  }

  :global(.profile-actions-menu) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 12px 28px rgb(11 11 11 / 14%);
    min-width: 168px;
    overflow: hidden;
    padding: 4px;
    z-index: 5;
  }

  :global(.profile-actions-menu-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: flex;
    font-size: 12px;
    font-weight: 600;
    gap: 8px;
    min-height: 34px;
    outline: none;
    padding: 0 8px;
  }

  :global(.profile-actions-menu-item[data-highlighted]) {
    background: var(--color-boulder-100);
  }

  :global(.profile-actions-menu-item[data-disabled]) {
    cursor: not-allowed;
    opacity: 0.45;
  }

  :global(.profile-actions-menu-item.destructive) {
    color: #a13f32;
  }

  :global(.profile-actions-menu-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }

  .secret-value-field :global(.icon-button) {
    position: absolute;
    right: 2px;
    top: 2px;
  }

  :global(.icon-button.destructive) {
    color: #a13f32;
  }

  :global(.icon-button.destructive:hover) {
    background: #fff3f1;
  }

  @media (max-width: 640px) {
    .editor-panel {
      padding: 18px;
    }

    .variable-table-header {
      display: none;
    }

    .variable-row {
      gap: 10px;
      grid-template-columns: 1fr 34px;
      padding: 12px 0;
    }

    .secret-key,
    .secret-value-field {
      grid-column: 1 / -1;
    }

    .row-actions {
      grid-column: 2;
      grid-row: 1;
      justify-self: end;
    }
  }

  :global(.tooltip-content) {
    background: var(--color-boulder-900);
    border-radius: 4px;
    color: #ffffff;
    font-size: 11px;
    padding: 5px 7px;
    z-index: 10;
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
    display: flex;
    justify-content: flex-end;
    margin-top: 24px;
    padding-top: 22px;
  }

  @media (max-width: 640px) {
    .editor-header,
    .variables-header,
    .editor-footer {
      align-items: flex-start;
      flex-direction: column;
      gap: 14px;
    }

    .variable-table-header,
    .variable-row {
      grid-template-columns: 1fr;
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

  }
</style>
