<script lang="ts">
  import { Check } from "@lucide/svelte";
  import { Button, Dialog } from "bits-ui";

  type Props = {
    open: boolean;
    editingSecretId: number | null;
    formKey: string;
    formValue: string;
    formError: string;
    onSave: () => void;
  };

  let {
    open = $bindable(),
    editingSecretId,
    formKey = $bindable(),
    formValue = $bindable(),
    formError,
    onSave,
  }: Props = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content class="dialog-content" aria-describedby="variable-dialog-desc">
      <div class="dialog-header">
        <Dialog.Title class="dialog-title">
          {editingSecretId !== null ? "Editar variable" : "Nueva variable"}
        </Dialog.Title>
        <Dialog.Description id="variable-dialog-desc" class="dialog-description">
          Define la clave y el valor para el entorno seleccionado.
        </Dialog.Description>
      </div>

      {#if formError}
        <div class="form-error-box">{formError}</div>
      {/if}

      <div class="dialog-form-fields">
        <div class="field-group">
          <label class="field-label" for="dialog-var-key">Nombre de Clave (KEY)</label>
          <input
            id="dialog-var-key"
            class="dialog-text-input font-mono"
            placeholder="EJ: DATABASE_URL, API_KEY"
            bind:value={formKey}
          />
        </div>
        <div class="field-group">
          <label class="field-label" for="dialog-var-value">Valor (VALUE)</label>
          <input
            id="dialog-var-value"
            class="dialog-text-input font-mono"
            placeholder="Valor del secreto"
            bind:value={formValue}
          />
        </div>
      </div>

      <div class="dialog-footer">
        <Dialog.Close class="secondary-button btn-sm">Cancelar</Dialog.Close>
        <Button.Root class="primary-button btn-sm" type="button" onclick={onSave}>
          <Check size={14} strokeWidth={2.2} />
          <span>Guardar variable</span>
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
