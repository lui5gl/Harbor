<script lang="ts">
  import { Check } from "@lucide/svelte";
  import { Button, Dialog, Switch } from "bits-ui";

  type ProfileSettingsDialogProps = {
    open: boolean;
    name: string;
    isProduction: boolean;
    onSave: (name: string, isProduction: boolean) => void;
  };

  let { open = $bindable(), name, isProduction, onSave }: ProfileSettingsDialogProps = $props();
  let draftName = $state("");
  let draftIsProduction = $state(false);

  $effect(() => {
    if (!open) return;
    draftName = name;
    draftIsProduction = isProduction;
  });

  function saveSettings() {
    onSave(draftName, draftIsProduction);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="profile-settings-dialog" aria-describedby="profile-settings-description">
      <Dialog.Title class="profile-settings-title">Profile settings</Dialog.Title>
      <Dialog.Description id="profile-settings-description" class="profile-settings-description">Adjust the profile name and its production protection.</Dialog.Description>

      <label class="field-label" for="settings-profile-name">Profile name</label>
      <input id="settings-profile-name" class="text-input" bind:value={draftName} />

      <div class="production-setting">
        <span>
          <strong>Production profile</strong>
          <small>Requires confirmation before becoming active.</small>
        </span>
        <Switch.Root class="production-switch" bind:checked={draftIsProduction} aria-label="Production profile">
          <Switch.Thumb class="production-switch-thumb" />
        </Switch.Root>
      </div>

      <div class="dialog-actions">
        <Dialog.Close class="secondary-button">Cancel</Dialog.Close>
        <Button.Root class="primary-button" type="button" onclick={saveSettings}>
          <Check size={16} strokeWidth={2.2} aria-hidden="true" />
          <span>Save changes</span>
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.modal-backdrop) {
    align-items: center;
    background: rgb(11 11 11 / 35%);
    display: flex;
    inset: 0;
    justify-content: center;
    padding: 24px;
    position: fixed;
    z-index: 10;
  }

  :global(.profile-settings-dialog) {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 20px 60px rgb(11 11 11 / 20%);
    display: grid;
    gap: 12px;
    left: 50%;
    max-width: 460px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(100%, 460px);
    z-index: 11;
  }

  :global(.profile-settings-title) {
    color: var(--color-boulder-950);
    font-size: 19px;
    font-weight: 650;
  }

  :global(.profile-settings-description) {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.5;
    margin-bottom: 10px;
  }

  .field-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 650;
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
    outline: none;
    padding: 0 10px;
    width: 100%;
  }

  .text-input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 16%);
  }

  .production-setting {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    display: flex;
    gap: 16px;
    justify-content: space-between;
    margin-top: 6px;
    min-height: 46px;
    padding: 8px 10px;
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

  :global(.production-switch) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 999px;
    cursor: pointer;
    display: inline-flex;
    height: 20px;
    padding: 2px;
    width: 36px;
  }

  :global(.production-switch[data-state="checked"]) {
    background: var(--color-east-bay-700);
    border-color: var(--color-east-bay-700);
  }

  :global(.production-switch-thumb) {
    background: #ffffff;
    border-radius: 999px;
    display: block;
    height: 14px;
    transform: translateX(0);
    transition: transform 150ms ease;
    width: 14px;
  }

  :global(.production-switch[data-state="checked"] .production-switch-thumb) {
    transform: translateX(16px);
  }

  .dialog-actions {
    align-items: center;
    display: flex;
    gap: 10px;
    justify-content: flex-end;
    margin-top: 12px;
  }

  :global(.primary-button),
  :global(.secondary-button) {
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

  :global(.secondary-button) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
    min-height: 36px;
    padding: 0 12px;
  }
</style>
