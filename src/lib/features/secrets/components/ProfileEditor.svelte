<script lang="ts">
  import {
    Check,
    Copy,
    Download,
    Eye,
    EyeOff,
    FileUp,
    FolderGit2,
    KeyRound,
    Lock,
    MoreHorizontal,
    Pencil,
    Shield,
    Trash2,
  } from "@lucide/svelte";
  import { Button, DropdownMenu, ScrollArea } from "bits-ui";
  import ImportVariablesDialog from "./ImportVariablesDialog.svelte";
  import ProfileSettingsDialog from "./ProfileSettingsDialog.svelte";
  import SecretGeneratorDialog from "./SecretGeneratorDialog.svelte";
  import type { Environment, Project, Secret } from "../types";
  import { t } from "$lib/i18n";

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
  let revealed = $state<number[]>([]);
  let copied = $state<number | null>(null);
  let settingsOpen = $state(false);
  let importOpen = $state(false);
  let generatorOpen = $state(false);
  let renameOpen = $state(false);
  let projectName = $state("");
  let draftKey = $state("");
  let isActive = $derived(environment.id === activeEnvironmentId);
  let secrets = $derived(
    environment.secrets.filter((secret) => secret.key.trim() || secret.value.trim()),
  );
  let allRevealed = $derived(
    environment.secrets.length > 0 &&
      environment.secrets.every((secret) => revealed.includes(secret.id)),
  );
  const filePart = (value: string) => value.toLowerCase().replace(/[^a-z0-9_-]/g, "-") || "local";
  function toggle(id: number) {
    revealed = revealed.includes(id) ? revealed.filter((item) => item !== id) : [...revealed, id];
  }
  function toggleAll() {
    revealed = allRevealed ? [] : environment.secrets.map((secret) => secret.id);
  }
  function exportText() {
    return environment.secrets
      .filter((secret) => secret.key.trim())
      .map((secret) =>
        secret.value.includes(" ") ||
        secret.value.includes("\n") ||
        secret.value.includes('"') ||
        secret.value.includes("#")
          ? `${secret.key}="${secret.value.replace(/"/g, '\\"')}"`
          : `${secret.key}=${secret.value}`,
      )
      .join("\n");
  }
  async function copyValue(secret: Secret) {
    try {
      await navigator.clipboard.writeText(secret.value);
      copied = secret.id;
      window.setTimeout(() => {
        if (copied === secret.id) copied = null;
      }, 1400);
    } catch {
      copied = null;
    }
  }
  async function copyEnv() {
    try {
      await navigator.clipboard.writeText(exportText());
    } catch {}
  }
  function downloadEnv() {
    const url = URL.createObjectURL(new Blob([exportText()], { type: "text/plain;charset=utf-8" }));
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = `.env.${filePart(project.name)}.${filePart(environment.name)}`;
    anchor.click();
    URL.revokeObjectURL(url);
  }
  function splitAssignment(secret: Secret, event: ClipboardEvent) {
    const text = event.clipboardData?.getData("text") ?? "";
    const separator = text.indexOf("=");
    if (separator <= 0) return;
    const key = text.slice(0, separator).trim();
    if (!key) return;
    event.preventDefault();
    onUpdateVariable(secret.id, "key", key);
    onUpdateVariable(secret.id, "value", text.slice(separator + 1));
  }
  function splitDraftAssignment(event: ClipboardEvent) {
    const text = event.clipboardData?.getData("text") ?? "";
    const separator = text.indexOf("=");
    if (separator <= 0) return;
    const key = text.slice(0, separator).trim();
    if (!key) return;
    event.preventDefault();
    const id = onAddVariable();
    if (id === null) return;
    onUpdateVariable(id, "key", key);
    onUpdateVariable(id, "value", text.slice(separator + 1));
    draftKey = "";
  }
  function updateDraftKey(value: string) {
    if (!value.trim()) {
      draftKey = "";
      return;
    }
    const id = onAddVariable();
    if (id === null) return;
    onUpdateVariable(id, "key", value);
    draftKey = "";
  }
  function saveProject() {
    const name = projectName.trim();
    if (name) onRenameProject(name);
    renameOpen = false;
  }
</script>

<section class="editor-panel" aria-labelledby="environment-title">
  <header class="project-header">
    <div class="project-identity">
      <FolderGit2 size={17} />
      <div>
        <p>{t("secrets.project")}</p>
        <h2>{project.name}</h2>
      </div>
    </div>
    <div class="header-actions">
      <DropdownMenu.Root
        ><DropdownMenu.Trigger class="icon-button" type="button" aria-label={t("secrets.actions")}
          ><MoreHorizontal size={17} /></DropdownMenu.Trigger
        ><DropdownMenu.Portal
          ><DropdownMenu.Content class="actions-menu" sideOffset={6} align="end"
            ><DropdownMenu.Item
              class="menu-item"
              onclick={() => {
                projectName = project.name;
                renameOpen = true;
              }}><Pencil size={14} />{t("secrets.renameProject")}</DropdownMenu.Item
            ><DropdownMenu.Separator class="menu-separator" /><DropdownMenu.Item
              class="menu-item destructive"
              onclick={onRequestDeleteProject}
              ><Trash2 size={14} />{t("secrets.deleteProject")}</DropdownMenu.Item
            ></DropdownMenu.Content
          ></DropdownMenu.Portal
        ></DropdownMenu.Root
      >
    </div>
  </header>
  <nav class="environment-tabs" aria-label={t("secrets.environments")}>
    {#each project.environments as item (item.id)}<Button.Root
        class={item.id === environment.id ? "selected" : ""}
        type="button"
        onclick={() => onSelectEnvironment(item.id)}
        ><span class:active={item.id === activeEnvironmentId} class="tab-dot"
        ></span>{item.name}{#if item.isProduction}<Lock size={11} />{/if}</Button.Root
      >{/each}
  </nav>
  <div class="environment-heading">
    <div>
      <div class="title-row">
        <h3 id="environment-title">{environment.name}</h3>
        {#if environment.isProduction}<span class="production-tag">{t("secrets.production")}</span
          >{/if}
      </div>
      <p>
        {environment.secrets.length}
        {environment.secrets.length === 1
          ? t("secrets.variable")
          : t("secrets.variables")}{#if isActive}<span class="active-label"
            >{t("secrets.activeInShell")}</span
          >{/if}
      </p>
    </div>
    <div class="header-actions">
      {#if !isActive}<Button.Root
          class="activate-button"
          type="button"
          onclick={onActivateEnvironment}><Shield size={14} />{t("secrets.activate")}</Button.Root
        >{/if}<DropdownMenu.Root
        ><DropdownMenu.Trigger class="icon-button" type="button" aria-label={t("secrets.actions")}
          ><MoreHorizontal size={17} /></DropdownMenu.Trigger
        ><DropdownMenu.Portal
          ><DropdownMenu.Content class="actions-menu" sideOffset={6} align="end"
            ><DropdownMenu.Item class="menu-item" onclick={() => (settingsOpen = true)}
              ><Pencil size={14} />{t("secrets.environmentSettings")}</DropdownMenu.Item
            ><DropdownMenu.Item class="menu-item" onclick={() => (generatorOpen = true)}
              ><KeyRound size={14} />{t("secrets.generateSecret")}</DropdownMenu.Item
            ><DropdownMenu.Item class="menu-item" onclick={() => (importOpen = true)}
              ><FileUp size={14} />{t("secrets.importEnv")}</DropdownMenu.Item
            ><DropdownMenu.Item class="menu-item" onclick={() => void copyEnv()}
              ><Copy size={14} />{t("secrets.copyEnv")}</DropdownMenu.Item
            ><DropdownMenu.Item class="menu-item" onclick={downloadEnv}
              ><Download size={14} />{t("secrets.downloadEnv")}</DropdownMenu.Item
            ><DropdownMenu.Separator class="menu-separator" /><DropdownMenu.Item
              class="menu-item destructive"
              onclick={onRequestDeleteEnvironment}
              ><Trash2 size={14} />{t("secrets.deleteEnvironment")}</DropdownMenu.Item
            ></DropdownMenu.Content
          ></DropdownMenu.Portal
        ></DropdownMenu.Root
      >
    </div>
  </div>
  <ScrollArea.Root class="variables-scroll" type="auto"
    ><ScrollArea.Viewport class="variables-table"
      ><div class="table-header">
        <span>Key</span><span
          >Value <Button.Root type="button" onclick={toggleAll}
            >{#if allRevealed}<EyeOff size={12} />Hide all{:else}<Eye size={12} />Show all{/if}</Button.Root
          ></span
        ><span aria-hidden="true"></span>
      </div>
      {#each secrets as secret (secret.id)}{@const shown = revealed.includes(secret.id)}
        <div class="variable-row">
          <input
            class="input"
            placeholder="VARIABLE_NAME"
            value={secret.key}
            oninput={(event) => onUpdateVariable(secret.id, "key", event.currentTarget.value)}
            onpaste={(event) => splitAssignment(secret, event)}
          />
          <div class="value-input">
            <input
              class="input"
              type={shown ? "text" : "password"}
              placeholder="Value"
              value={secret.value}
              oninput={(event) => onUpdateVariable(secret.id, "value", event.currentTarget.value)}
            /><Button.Root
              type="button"
              onclick={() => toggle(secret.id)}
              aria-label="Toggle visibility"
              title="Toggle visibility"
              >{#if shown}<EyeOff size={14} />{:else}<Eye size={14} />{/if}</Button.Root
            >
          </div>
          <div class="row-actions">
            <Button.Root
              type="button"
              onclick={() => void copyValue(secret)}
              aria-label="Copy value"
              title="Copy value"
              >{#if copied === secret.id}<Check size={14} />{:else}<Copy
                  size={14}
                />{/if}</Button.Root
            ><Button.Root
              class="danger-icon"
              type="button"
              onclick={() => onRequestDeleteVariable(secret.id)}
              aria-label="Delete variable"
              title="Delete variable"><Trash2 size={14} /></Button.Root
            >
          </div>
        </div>{/each}
      <div class="variable-row draft-row">
        <input
          class="input"
          placeholder="VARIABLE_NAME"
          value={draftKey}
          oninput={(event) => updateDraftKey(event.currentTarget.value)}
          onpaste={splitDraftAssignment}
        />
        <div class="value-input"><input class="input" placeholder="Value" disabled /></div>
        <div class="row-actions"></div>
      </div></ScrollArea.Viewport
    ><ScrollArea.Scrollbar class="scrollbar" orientation="vertical"
      ><ScrollArea.Thumb class="scroll-thumb" /></ScrollArea.Scrollbar
    ></ScrollArea.Root
  >
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
{#if renameOpen}<div class="rename-backdrop">
    <form
      class="rename-dialog"
      onsubmit={(event) => {
        event.preventDefault();
        saveProject();
      }}
    >
      <h3>Rename project</h3>
      <input class="input" bind:value={projectName} />
      <div>
        <Button.Root type="button" onclick={() => (renameOpen = false)}>Cancel</Button.Root
        ><Button.Root class="save-button" type="submit">Save</Button.Root>
      </div>
    </form>
  </div>{/if}

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
  .project-header,
  .environment-heading,
  .project-identity,
  .header-actions,
  .title-row,
  .value-input,
  .row-actions {
    align-items: center;
    display: flex;
  }
  .project-header,
  .environment-heading {
    justify-content: space-between;
  }
  .project-header {
    border-bottom: 1px solid var(--color-boulder-100);
    padding-bottom: 12px;
  }
  .project-identity {
    color: var(--color-east-bay-700);
    gap: 9px;
  }
  .project-identity p {
    color: var(--color-boulder-500);
    font-size: 10px;
    font-weight: 700;
    margin: 0 0 2px;
    text-transform: uppercase;
  }
  .project-identity h2 {
    color: var(--color-boulder-950);
    font-size: 15px;
    margin: 0;
  }
  .header-actions {
    gap: 7px;
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
  :global(.activate-button) {
    background: #166534;
    color: #fff;
  }
  :global(.icon-button) {
    align-items: center;
    background: #fff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: inline-flex;
    height: 32px;
    justify-content: center;
    width: 32px;
  }
  .environment-tabs {
    border-bottom: 1px solid var(--color-boulder-200);
    display: flex;
    gap: 3px;
    overflow-x: auto;
    padding-top: 10px;
  }
  .environment-tabs :global(button) {
    align-items: center;
    background: transparent;
    border: 0;
    border-bottom: 2px solid transparent;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12px;
    font-weight: 650;
    gap: 5px;
    height: 34px;
    padding: 0 9px;
    white-space: nowrap;
  }
  .environment-tabs :global(button.selected) {
    border-bottom-color: var(--color-east-bay-700);
    color: var(--color-east-bay-900);
  }
  .tab-dot {
    background: var(--color-boulder-300);
    border-radius: 50%;
    height: 6px;
    width: 6px;
  }
  .tab-dot.active {
    background: #16a34a;
  }
  .environment-heading {
    padding: 14px 0 11px;
  }
  .environment-heading h3 {
    color: var(--color-boulder-950);
    font-size: 16px;
    margin: 0;
  }
  .environment-heading p {
    color: var(--color-boulder-500);
    font-size: 11px;
    margin: 4px 0 0;
  }
  .production-tag {
    background: #fff7ed;
    border: 1px solid #fed7aa;
    border-radius: 4px;
    color: #9a3412;
    font-size: 9px;
    font-weight: 700;
    margin-left: 8px;
    padding: 2px 5px;
    text-transform: uppercase;
  }
  .active-label {
    color: #15803d;
    font-weight: 650;
    margin-left: 8px;
  }
  .value-input :global(button),
  .row-actions :global(button),
  .table-header :global(button) {
    align-items: center;
    background: transparent;
    border: 0;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    padding: 3px;
  }
  :global(.variables-scroll) {
    flex: 1;
    min-height: 0;
  }
  :global(.variables-table) {
    height: 100%;
    width: 100%;
  }
  .table-header,
  .variable-row {
    display: grid;
    gap: 10px;
    grid-template-columns: minmax(120px, 0.8fr) minmax(180px, 1.5fr) 62px;
  }
  .table-header {
    color: var(--color-boulder-500);
    font-size: 10px;
    font-weight: 700;
    padding: 10px 2px 7px;
    text-transform: uppercase;
  }
  .table-header span {
    align-items: center;
    display: flex;
    gap: 6px;
  }
  .table-header :global(button) {
    color: var(--color-east-bay-700);
    font-size: 10px;
    font-weight: 650;
    text-transform: none;
  }
  .variable-row {
    border-top: 1px solid var(--color-boulder-100);
    padding: 7px 2px;
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
    min-width: 0;
    outline: none;
    padding: 0 8px;
    width: 100%;
  }
  .input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 15%);
  }
  .value-input {
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    overflow: hidden;
  }
  .value-input .input {
    border: 0;
  }
  .value-input:focus-within {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 15%);
  }
  .row-actions {
    gap: 2px;
    justify-content: flex-end;
  }
  .row-actions :global(button) {
    border-radius: 4px;
  }
  .row-actions :global(button:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
  }
  .row-actions :global(.danger-icon:hover) {
    background: #fef2f2;
    color: #b91c1c;
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
  :global(.actions-menu) {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    min-width: 185px;
    padding: 4px;
    z-index: 20;
  }
  :global(.menu-item) {
    align-items: center;
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-700);
    cursor: pointer;
    display: flex;
    font: inherit;
    font-size: 12px;
    gap: 8px;
    padding: 8px;
    width: 100%;
  }
  :global(.menu-item:hover) {
    background: var(--color-boulder-50);
  }
  :global(.menu-item.destructive) {
    color: #b91c1c;
  }
  :global(.menu-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }
  .rename-backdrop {
    align-items: center;
    background: rgb(0 0 0 / 35%);
    display: flex;
    inset: 0;
    justify-content: center;
    position: fixed;
    z-index: 100;
  }
  .rename-dialog {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-shadow: 0 15px 40px rgb(0 0 0 / 15%);
    padding: 18px;
    width: min(360px, calc(100vw - 32px));
  }
  .rename-dialog h3 {
    color: var(--color-boulder-900);
    font-size: 15px;
    margin: 0 0 12px;
  }
  .rename-dialog div {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 12px;
  }
  .rename-dialog :global(button) {
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
  .rename-dialog :global(.save-button) {
    background: var(--color-east-bay-900);
    color: #fff;
  }
  @media (max-width: 700px) {
    .editor-panel {
      max-height: none;
      min-height: 570px;
    }
    .project-header,
    .environment-heading {
      align-items: flex-start;
      flex-direction: column;
    }
    .header-actions {
      width: 100%;
    }
    .header-actions :global(.activate-button) {
      flex: 1;
      justify-content: center;
    }
    .table-header {
      display: none;
    }
    .variable-row {
      grid-template-columns: 1fr 38px;
    }
    .variable-row > :first-child {
      grid-column: 1 / -1;
    }
    .value-input {
      grid-column: 1;
      grid-row: 2;
    }
    .row-actions {
      grid-column: 2;
      grid-row: 2;
    }
  }
</style>
