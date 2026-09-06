<script lang="ts">
  import { Button, Dialog } from "bits-ui";
  import { t } from "$lib/i18n";

  type Props = {
    open: boolean;
    initialName: string;
    onSave: (name: string) => void;
  };

  let { open = $bindable(), initialName, onSave }: Props = $props();
  let draftName = $state("");

  $effect(() => {
    if (open) draftName = initialName;
  });

  function saveProject() {
    const name = draftName.trim();
    if (!name) return;
    onSave(name);
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="rename-backdrop" />
    <Dialog.Content class="rename-dialog" aria-describedby="rename-project-description">
      <Dialog.Title class="rename-title">{t("secrets.renameProject")}</Dialog.Title>
      <Dialog.Description id="rename-project-description" class="sr-only">
        {t("secrets.renameProject")}
      </Dialog.Description>
      <form
        onsubmit={(event) => {
          event.preventDefault();
          saveProject();
        }}
      >
        <label class="sr-only" for="project-name">{t("secrets.renameProject")}</label>
        <input id="project-name" class="input" bind:value={draftName} />
        <div class="dialog-actions">
          <Dialog.Close class="dialog-button">{t("common.cancel")}</Dialog.Close>
          <Button.Root class="dialog-button save-button" type="submit">
            {t("common.save")}
          </Button.Root>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  :global(.rename-backdrop) {
    background: rgb(0 0 0 / 35%);
    inset: 0;
    position: fixed;
    z-index: 100;
  }
  :global(.rename-dialog) {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-shadow: 0 15px 40px rgb(0 0 0 / 15%);
    left: 50%;
    padding: 18px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(360px, calc(100vw - 32px));
    z-index: 101;
  }
  :global(.rename-title) {
    color: var(--color-boulder-900);
    font-size: 15px;
    margin: 0 0 12px;
  }
  .input {
    background: #fff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 12px;
    height: 32px;
    outline: none;
    padding: 0 8px;
    width: 100%;
  }
  .input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 15%);
  }
  .dialog-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 12px;
  }
  :global(.dialog-button) {
    background: var(--color-boulder-100);
    border: 0;
    border-radius: 5px;
    color: var(--color-boulder-700);
    cursor: pointer;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    height: 32px;
    padding: 0 10px;
  }
  :global(.save-button) {
    background: var(--color-east-bay-900);
    color: #fff;
  }
  .sr-only {
    clip: rect(0 0 0 0);
    height: 1px;
    margin: -1px;
    overflow: hidden;
    position: absolute;
    width: 1px;
  }
</style>
