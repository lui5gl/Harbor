<script lang="ts">
  import ImportVariablesDialog from "./ImportVariablesDialog.svelte";
  import ProfileHeader from "./ProfileHeader.svelte";
  import ProfileSettingsDialog from "./ProfileSettingsDialog.svelte";
  import RenameProjectDialog from "./RenameProjectDialog.svelte";
  import SecretGeneratorDialog from "./SecretGeneratorDialog.svelte";
  import VariablesTable from "./VariablesTable.svelte";
  import { copyEnvironment, downloadEnvironment } from "../application/environmentExport";
  import type { Environment, Project } from "../types";

  type Props = {
    project: Project;
    environment: Environment;
    activeEnvironmentId: number | null;
    onSelectEnvironment: (id: number) => void;
    onRenameProject: (name: string) => void;
    onRequestDeleteProject: () => void;
    onSaveEnvironment: (name: string, isProduction: boolean) => void;
    onRequestDeleteEnvironment: () => void;
    onAddVariable: () => number | null;
    onAddCustomVariable: (key: string, value: string) => void;
    onImportVariables: (variables: { key: string; value: string }[], replaceAll: boolean) => void;
    onUpdateVariable: (id: number, field: "key" | "value", value: string) => void;
    onRequestDeleteVariable: (id: number) => void;
    onActivateEnvironment: () => void;
  };
  let {
    project,
    environment,
    activeEnvironmentId,
    onSelectEnvironment,
    onRenameProject,
    onRequestDeleteProject,
    onSaveEnvironment,
    onRequestDeleteEnvironment,
    onAddVariable,
    onAddCustomVariable,
    onImportVariables,
    onUpdateVariable,
    onRequestDeleteVariable,
    onActivateEnvironment,
  }: Props = $props();
  let settingsOpen = $state(false);
  let importOpen = $state(false);
  let generatorOpen = $state(false);
  let renameOpen = $state(false);

  function copyEnv() {
    return copyEnvironment(environment);
  }

  function downloadEnv() {
    downloadEnvironment(project, environment);
  }
</script>

<section class="editor-panel" aria-labelledby="environment-title">
  <ProfileHeader
    {project}
    {environment}
    {activeEnvironmentId}
    {onSelectEnvironment}
    onRenameProject={() => (renameOpen = true)}
    {onRequestDeleteProject}
    onOpenSettings={() => (settingsOpen = true)}
    onOpenGenerator={() => (generatorOpen = true)}
    onOpenImport={() => (importOpen = true)}
    onCopyEnvironment={() => void copyEnv()}
    onDownloadEnvironment={downloadEnv}
    {onRequestDeleteEnvironment}
    {onActivateEnvironment}
  />
  <VariablesTable {environment} {onAddVariable} {onUpdateVariable} {onRequestDeleteVariable} />
</section>
<ImportVariablesDialog bind:open={importOpen} onImport={onImportVariables} />
<ProfileSettingsDialog
  bind:open={settingsOpen}
  name={environment.name}
  isProduction={environment.isProduction}
  onSave={onSaveEnvironment}
/>
<SecretGeneratorDialog
  bind:open={generatorOpen}
  initialKey=""
  onApply={(key, value) => onAddCustomVariable(key || "SECRET_KEY", value)}
/>
<RenameProjectDialog bind:open={renameOpen} initialName={project.name} onSave={onRenameProject} />

<style>
  .editor-panel {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    height: 100%;
    max-height: 580px;
    min-width: 0;
    padding: 16px 18px;
  }
  :global(.secondary-action),
  :global(.primary-action),
  :global(.activate-button) {
    align-items: center;
    border: 0;
    border-radius: 6px;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    gap: 5px;
    height: 32px;
    padding: 0 10px;
  }
  :global(.secondary-action) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
  }
  :global(.primary-action) {
    background: var(--color-east-bay-900);
    color: #fff;
  }
  :global(.scrollbar) {
    display: flex;
    padding: 2px;
    width: 6px;
  }
  :global(.scroll-thumb) {
    background: var(--color-boulder-300);
    border-radius: 99px;
    flex: 1;
  }
  @media (max-width: 700px) {
    .editor-panel {
      max-height: none;
      min-height: 570px;
    }
  }
</style>
