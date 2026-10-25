<script lang="ts">
  import CloneEnvironmentDialog from "./CloneEnvironmentDialog.svelte";
  import CompareEnvironmentsDialog from "./CompareEnvironmentsDialog.svelte";
  import ImportVariablesDialog from "./ImportVariablesDialog.svelte";
  import ProfileHeader from "./ProfileHeader.svelte";
  import ProfileSettingsDialog from "./ProfileSettingsDialog.svelte";
  import type { EnvironmentAction } from "./EnvironmentActionsMenu.svelte";
  import SecretGeneratorDialog from "./SecretGeneratorDialog.svelte";
  import VariablesTable from "./VariablesTable.svelte";
  import { copyEnvironment, downloadEnvironment } from "../application/environmentExport";
  import type { Environment, Project } from "../types";

  type Props = {
    disabled?: boolean;
    project: Project;
    environment: Environment;
    activeEnvironmentId: number | null;
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
    project,
    environment,
    activeEnvironmentId,
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

  export function run(action: Exclude<EnvironmentAction, "activate" | "deactivate">) {
    if (action === "settings") settingsOpen = true;
    else if (action === "generate") generatorOpen = true;
    else if (action === "import") importOpen = true;
    else if (action === "clone") cloneOpen = true;
    else if (action === "compare") compareOpen = true;
    else if (action === "terminal") onOpenTerminal();
    else if (action === "copy") void copyEnv();
    else if (action === "download") downloadEnv();
    else onRequestDeleteEnvironment();
  }
</script>

<section
  class="flex h-full min-h-[min(560px,calc(100dvh-280px))] min-w-0 flex-col rounded-lg border border-boulder-200 bg-white p-3.5 shadow-sm"
  aria-labelledby="environment-title"
>
  <ProfileHeader {project} {environment} {activeEnvironmentId} />
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
