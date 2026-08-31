<script lang="ts">
  import { Check, FolderPlus } from "@lucide/svelte";
  import { Button, Dialog } from "bits-ui";

  type ProjectDialogProps = {
    open: boolean;
    onCreate: (projectName: string, environmentName: string) => void;
  };

  let { open = $bindable(), onCreate }: ProjectDialogProps = $props();
  let projectName = $state("");
  let environmentName = $state("Development");

  $effect(() => {
    if (!open) {
      projectName = "";
      environmentName = "Development";
    }
  });

  function submit() {
    const normalizedProjectName = projectName.trim();
    const normalizedEnvironmentName = environmentName.trim();
    if (!normalizedProjectName || !normalizedEnvironmentName) return;
    onCreate(normalizedProjectName, normalizedEnvironmentName);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="creation-dialog" aria-describedby="new-project-description">
      <div class="dialog-heading">
        <div class="dialog-icon"><FolderPlus size={19} strokeWidth={2.2} /></div>
        <div>
          <Dialog.Title class="dialog-title">New project</Dialog.Title>
          <Dialog.Description id="new-project-description" class="dialog-description">
            Create an application and its first environment.
          </Dialog.Description>
        </div>
      </div>

      <label class="field-label" for="project-name">Project / application name</label>
      <input id="project-name" class="text-input" placeholder="e.g. Acme Store API" bind:value={projectName} />

      <label class="field-label" for="environment-name">First environment</label>
      <input id="environment-name" class="text-input" placeholder="e.g. Development" bind:value={environmentName} />

      <div class="dialog-actions">
        <Dialog.Close class="secondary-button">Cancel</Dialog.Close>
        <Button.Root class="primary-button" type="button" onclick={submit} disabled={!projectName.trim() || !environmentName.trim()}>
          <Check size={15} strokeWidth={2.2} />
          <span>Create project</span>
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.modal-backdrop) { background: rgb(11 11 11 / 45%); inset: 0; position: fixed; z-index: 100; }
  :global(.creation-dialog) { background: #fff; border: 1px solid var(--color-boulder-200); border-radius: 8px; box-shadow: 0 20px 60px rgb(11 11 11 / 20%); box-sizing: border-box; display: flex; flex-direction: column; gap: 9px; left: 50%; max-width: 430px; padding: 20px; position: fixed; top: 50%; transform: translate(-50%, -50%); width: calc(100vw - 32px); z-index: 101; }
  .dialog-heading { align-items: flex-start; display: flex; gap: 11px; margin-bottom: 5px; }
  .dialog-icon { align-items: center; background: var(--color-east-bay-50); border: 1px solid var(--color-east-bay-200); border-radius: 7px; color: var(--color-east-bay-700); display: flex; flex-shrink: 0; height: 36px; justify-content: center; width: 36px; }
  :global(.dialog-title) { color: var(--color-boulder-950); font-size: 16px; font-weight: 700; }
  :global(.dialog-description) { color: var(--color-boulder-600); font-size: 12.5px; line-height: 1.45; margin: 3px 0 0; }
  .field-label { color: var(--color-boulder-700); font-size: 12px; font-weight: 650; margin-top: 4px; }
  .text-input { border: 1px solid var(--color-boulder-300); border-radius: 6px; box-sizing: border-box; color: var(--color-boulder-900); font: inherit; font-size: 13px; height: 36px; outline: none; padding: 0 10px; width: 100%; }
  .text-input:focus { border-color: var(--color-east-bay-500); box-shadow: 0 0 0 3px rgb(113 132 192 / 16%); }
  .dialog-actions { display: flex; gap: 8px; justify-content: flex-end; margin-top: 10px; }
  :global(.secondary-button), :global(.primary-button) { align-items: center; border: 0; border-radius: 6px; cursor: pointer; display: inline-flex; font: inherit; font-size: 12px; font-weight: 650; gap: 6px; height: 34px; justify-content: center; padding: 0 12px; }
  :global(.secondary-button) { background: var(--color-boulder-100); color: var(--color-boulder-800); }
  :global(.primary-button) { background: var(--color-east-bay-900); color: #fff; }
  :global(.primary-button:disabled) { cursor: not-allowed; opacity: .5; }
</style>