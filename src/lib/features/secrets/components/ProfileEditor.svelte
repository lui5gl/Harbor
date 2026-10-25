<script lang="ts">
  import CloneEnvironmentDialog from "./CloneEnvironmentDialog.svelte";
  import CompareEnvironmentsDialog from "./CompareEnvironmentsDialog.svelte";
  import ImportVariablesDialog from "./ImportVariablesDialog.svelte";
  import ProfileHeader from "./ProfileHeader.svelte";
  import ProfileSettingsDialog from "./ProfileSettingsDialog.svelte";
  import SecretGeneratorDialog from "./SecretGeneratorDialog.svelte";
  import VariablesTable from "./VariablesTable.svelte";
  import { copyEnvironment, downloadEnvironment } from "../application/environmentExport";
  import type { Environment, Project } from "../types";

  type Props = {
    disabled?: boolean;
    busy?: boolean;
    activationBusy?: boolean;
    project: Project;
    environment: Environment;
    activeEnvironmentId: number | null;
    onActivate: () => void;
    onDeactivate: () => void;
    onSaveEnvironment: (name: string) => void;
    onRequestDeleteEnvironment: () => void;
    onCloneEnvironment: (name: string) => void;
    onAddVariable: () => number | null;
    onAddCustomVariable: (key: string, value: string) => void;
    onImportVariables: (variables: { key: string; value: string }[], replaceAll: boolean) => void;
    onUpdateVariable: (id: number, field: "key" | "value", value: string) => void;
    onRequestDeleteVariable: (id: number) => void;
    onOpenTerminal: () => void;
  };
  let {
    disabled = false,
    busy = false,
    activationBusy = false,
    project,
    environment,
    activeEnvironmentId,
    onActivate,
    onDeactivate,
    onSaveEnvironment,
    onRequestDeleteEnvironment,
    onCloneEnvironment,
    onAddVariable,
    onAddCustomVariable,
    onImportVariables,
    onUpdateVariable,
    onRequestDeleteVariable,
    onOpenTerminal,
  }: Props = $props();
  let settingsOpen = $state(false);
  let importOpen = $state(false);
  let cloneOpen = $state(false);
  let compareOpen = $state(false);
  let generatorOpen = $state(false);

  function copyEnv() {
    return copyEnvironment(environment);
  }

  function downloadEnv() {
    downloadEnvironment(project, environment);
  }
</script>

<section
  class="flex h-full min-h-[min(560px,calc(100dvh-280px))] min-w-0 flex-col rounded-lg border border-boulder-200 bg-white px-5 py-4 shadow-sm"
  aria-labelledby="environment-title"
>
  <ProfileHeader
    {disabled}
    {busy}
    {activationBusy}
    {project}
    {environment}
    {activeEnvironmentId}
    {onActivate}
    {onDeactivate}
    onOpenSettings={() => (settingsOpen = true)}
    onOpenGenerator={() => (generatorOpen = true)}
    onOpenImport={() => (importOpen = true)}
    onOpenClone={() => (cloneOpen = true)}
    onOpenCompare={() => (compareOpen = true)}
    onCopyEnvironment={() => void copyEnv()}
    onDownloadEnvironment={downloadEnv}
    {onRequestDeleteEnvironment}
    {onOpenTerminal}
  />
  <VariablesTable
    {disabled}
    {environment}
    {onAddVariable}
    {onUpdateVariable}
    {onRequestDeleteVariable}
  />
</section>
<ImportVariablesDialog
  bind:open={importOpen}
  existingKeys={environment.secrets.map((secret) => secret.key)}
  onImport={onImportVariables}
/>
<CloneEnvironmentDialog
  bind:open={cloneOpen}
  projectName={project.name}
  sourceEnvironmentName={environment.name}
  variableCount={environment.secrets.length}
  onClone={onCloneEnvironment}
/>
<CompareEnvironmentsDialog
  bind:open={compareOpen}
  currentEnvironment={environment}
  availableEnvironments={project.environments.filter((e) => e.id !== environment.id)}
  onImportMissing={(missing) => onImportVariables(missing, false)}
/>
<ProfileSettingsDialog
  bind:open={settingsOpen}
  name={environment.name}
  onSave={onSaveEnvironment}
/>
<SecretGeneratorDialog
  bind:open={generatorOpen}
  initialKey=""
  onApply={(key, value) => onAddCustomVariable(key || "SECRET_KEY", value)}
/>
