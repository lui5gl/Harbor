<script lang="ts">
  import { Check } from "@lucide/svelte";
  import { Button, Dialog, Switch } from "bits-ui";

  type Props = {
    open: boolean;
    name: string;
    isProduction: boolean;
    onCreate: () => void;
  };

  let {
    open = $bindable(),
    name = $bindable(),
    isProduction = $bindable(),
    onCreate,
  }: Props = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="dialog-content" aria-describedby="profile-dialog-desc">
      <div class="dialog-header">
        <Dialog.Title class="dialog-title">Nuevo entorno</Dialog.Title>
        <Dialog.Description id="profile-dialog-desc" class="dialog-description">
          Crea un entorno aislado para gestionar sus variables.
        </Dialog.Description>
      </div>

      <div class="dialog-form-fields">
        <div class="field-group">
          <label class="field-label" for="dialog-prof-name">Nombre del entorno</label>
          <input
            id="dialog-prof-name"
            class="dialog-text-input"
            placeholder="EJ: Staging, Local, QA"
            bind:value={name}
          />
        </div>
        <div class="production-switch-row">
          <div class="switch-info">
            <span class="switch-title">Entorno de Producción</span>
            <span class="switch-desc">Solicitará confirmación antes de activarse</span>
          </div>
          <Switch.Root class="production-switch" bind:checked={isProduction}>
            <Switch.Thumb class="production-switch-thumb" />
          </Switch.Root>
        </div>
      </div>

      <div class="dialog-footer">
        <Dialog.Close class="secondary-button btn-sm">Cancelar</Dialog.Close>
        <Button.Root class="primary-button btn-sm" type="button" onclick={onCreate}>
          <Check size={14} strokeWidth={2.2} />
          <span>Crear entorno</span>
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
