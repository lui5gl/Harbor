<script lang="ts">
  import {
    AlertCircle,
    Download,
    ExternalLink,
    Folder,
    FolderOpen,
    Globe,
    Layers,
    Pencil,
    Play,
    Plus,
    Server,
    Square,
    Terminal,
    Trash2,
  } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import DeleteRuntimeDialog from "./DeleteRuntimeDialog.svelte";
  import PhpStackProfileDialog from "./PhpStackProfileDialog.svelte";
  import PhpWebStackOverview from "./PhpWebStackOverview.svelte";
  import InstallVersionDialog from "./InstallVersionDialog.svelte";
  import PhpStackProfilesPanel from "./PhpStackProfilesPanel.svelte";
  import { createPhpStackProfiles } from "../application/phpStackProfiles.svelte";
  import { openRuntimePath, openRuntimeUrl } from "../infrastructure/runtimeOpener";
  import { RuntimePath } from "../runtimePaths";
  import {
    cleanVersion,
    getCompatibleApacheVersions,
    parseVersionString,
    type PhpStackProfile,
  } from "../types";

  type PhpWebStackEditorProps = {
    activePhpVersion: string | null;
    installedPhpVersions: string[];
    availablePhpVersions: string[];
    activeApacheVersion: string | null;
    installedApacheVersions: string[];
    availableApacheVersions: string[];
    isPhpRunning: boolean;
    fastCgiAddress: string;
    isInstalling: boolean;
    installProgress: number;
    installingService: string;
    installingVersion: string;
    installError: string;
    onSelectPhpVersion: (version: string) => Promise<void>;
    onTogglePhpFastCgi: () => Promise<void>;
    onInstallVersion: (service: "PHP" | "Apache", version: string) => Promise<void>;
    onDeleteVersion: (service: "PHP" | "Apache", version: string) => Promise<void>;
  };

  let {
    activePhpVersion,
    installedPhpVersions,
    availablePhpVersions,
    activeApacheVersion,
    installedApacheVersions,
    availableApacheVersions,
    isPhpRunning,
    fastCgiAddress,
    isInstalling,
    installProgress,
    installingService,
    installingVersion,
    installError,
    onSelectPhpVersion,
    onTogglePhpFastCgi,
    onInstallVersion,
    onDeleteVersion,
  }: PhpWebStackEditorProps = $props();

  const profileStore = createPhpStackProfiles();

  let isProfileModalOpen = $state(false);
  let editingProfile = $state<PhpStackProfile | null>(null);

  let profileFormName = $state("");
  let profileFormPhp = $state("");
  let profileFormApache = $state("");
  let profileFormError = $state("");

  let pendingDelete = $state<{ service: "PHP" | "Apache"; version: string } | null>(null);

  $effect(() => profileStore.load());

  $effect(() => {
    profileStore.ensureDefaults(
      activePhpVersion ? [activePhpVersion, ...installedPhpVersions] : installedPhpVersions,
      activeApacheVersion
        ? [activeApacheVersion, ...installedApacheVersions]
        : installedApacheVersions,
    );
  });

  let cleanInstalledPhp = $derived(installedPhpVersions.map(cleanVersion));
  let cleanInstalledApache = $derived(installedApacheVersions.map(cleanVersion));

  let phpDownloadOptions = $derived(
    availablePhpVersions
      .map((v) => {
        const meta = parseVersionString(v);
        return {
          version: meta.version,
          label: `PHP ${meta.version} (${meta.channel || "Disponible"})`,
        };
      })
      .filter((opt) => !cleanInstalledPhp.includes(opt.version)),
  );

  let apacheDownloadOptions = $derived(
    availableApacheVersions
      .map((v) => {
        const meta = parseVersionString(v);
        return {
          version: meta.version,
          label: `Apache ${meta.version} (${meta.channel || "Disponible"})`,
        };
      })
      .filter((opt) => !cleanInstalledApache.includes(opt.version)),
  );

  let isSelectedPhpNeedsInstall = $derived(
    profileFormPhp ? !cleanInstalledPhp.includes(cleanVersion(profileFormPhp)) : false,
  );

  let isSelectedApacheNeedsInstall = $derived(
    profileFormApache ? !cleanInstalledApache.includes(cleanVersion(profileFormApache)) : false,
  );

  let cleanActivePhp = $derived(activePhpVersion ? cleanVersion(activePhpVersion) : null);

  let cleanActiveApache = $derived(activeApacheVersion ? cleanVersion(activeApacheVersion) : null);

  let activeProfile = $derived(() => {
    if (!cleanActivePhp) return profileStore.profiles[0] ?? null;
    return (
      profileStore.profiles.find((p) => p.phpVersion === cleanActivePhp) ??
      profileStore.profiles[0] ??
      null
    );
  });

  let isApacheMissing = $derived(installedApacheVersions.length === 0);

  let compatibility = $derived(
    getCompatibleApacheVersions(activePhpVersion, availableApacheVersions),
  );

  function openCreateProfileModal() {
    editingProfile = null;
    profileFormName = "";
    profileFormPhp =
      cleanActivePhp ??
      cleanInstalledPhp[0] ??
      (availablePhpVersions[0] ? cleanVersion(availablePhpVersions[0]) : "8.3.17");
    profileFormApache =
      cleanActiveApache ??
      cleanInstalledApache[0] ??
      (availableApacheVersions[0] ? cleanVersion(availableApacheVersions[0]) : "2.4.62");
    profileFormError = "";
    isProfileModalOpen = true;
  }

  function openEditProfileModal(profile: PhpStackProfile, event: MouseEvent) {
    event.stopPropagation();
    editingProfile = profile;
    profileFormName = profile.name;
    profileFormPhp = profile.phpVersion;
    profileFormApache = profile.apacheVersion;
    profileFormError = "";
    isProfileModalOpen = true;
  }

  async function handleSaveProfile() {
    if (!profileFormName.trim()) {
      profileFormError = "Ingresa un nombre descriptivo para el perfil.";
      return;
    }

    const cleanPhp = cleanVersion(profileFormPhp);
    const cleanApache = cleanVersion(profileFormApache);

    // Auto-install PHP if needed
    if (isSelectedPhpNeedsInstall) {
      try {
        await onInstallVersion("PHP", cleanPhp);
      } catch (err) {
        profileFormError = `Error al descargar PHP ${cleanPhp}: ${err instanceof Error ? err.message : String(err)}`;
        return;
      }
    }

    // Auto-install Apache if needed
    if (isSelectedApacheNeedsInstall) {
      try {
        await onInstallVersion("Apache", cleanApache);
      } catch (err) {
        profileFormError = `Error al descargar Apache ${cleanApache}: ${err instanceof Error ? err.message : String(err)}`;
        return;
      }
    }

    if (editingProfile) {
      const updated = profileStore.profiles.map((p) =>
        p.id === editingProfile?.id
          ? {
              ...p,
              name: profileFormName.trim(),
              phpVersion: cleanPhp,
              apacheVersion: cleanApache,
            }
          : p,
      );
      profileStore.save(updated);
    } else {
      const newProfile: PhpStackProfile = {
        id: `profile-${Date.now()}`,
        name: profileFormName.trim(),
        phpVersion: cleanPhp,
        apacheVersion: cleanApache,
      };
      profileStore.save([...profileStore.profiles, newProfile]);
      void handleSelectProfile(newProfile);
    }

    isProfileModalOpen = false;
  }

  function handleSelectProfile(profile: PhpStackProfile) {
    if (profile.phpVersion !== cleanActivePhp) {
      void onSelectPhpVersion(profile.phpVersion);
    }
  }

  function handleDeleteProfile(id: string, event: MouseEvent) {
    event.stopPropagation();
    if (profileStore.profiles.length <= 1) return;
    const updated = profileStore.profiles.filter((p) => p.id !== id);
    profileStore.save(updated);
  }

  function requestDelete(service: "PHP" | "Apache", version: string) {
    pendingDelete = { service, version: cleanVersion(version) };
  }

  function confirmDelete() {
    if (pendingDelete) {
      void onDeleteVersion(pendingDelete.service, pendingDelete.version);
      pendingDelete = null;
    }
  }
</script>

<section class="editor-panel" aria-labelledby="web-stack-title">
  <!-- Header -->
  <header class="editor-header">
    <div class="header-main">
      <div class="header-titles">
        <p class="eyebrow">Entorno Web</p>
        <h2 id="web-stack-title">PHP & Servidor Web</h2>
      </div>
      <p class="header-sub">
        Perfiles de desarrollo con PHP y Apache integrados para ejecutar aplicaciones web y comandos
        CLI locales.
      </p>
    </div>

    <div class="header-controls">
      <div class={`status-pill${isPhpRunning ? " running" : ""}`}>
        <span class="status-indicator" aria-hidden="true"></span>
        <span class="status-label">{isPhpRunning ? "Servidor Activo" : "Servidor Detenido"}</span>
      </div>

      <Button.Root
        class={`toggle-service-btn${isPhpRunning ? " is-stop" : ""}`}
        type="button"
        disabled={!activePhpVersion || isInstalling}
        onclick={() => void onTogglePhpFastCgi()}
      >
        {#if isPhpRunning}
          <Square size={14} strokeWidth={2.4} aria-hidden="true" />
          <span>Detener Servidor</span>
        {:else}
          <Play size={14} strokeWidth={2.4} aria-hidden="true" />
          <span>Iniciar Servidor</span>
        {/if}
      </Button.Root>
    </div>
  </header>

  <!-- Missing Apache Web Engine Notice (Only when required) -->
  {#if isApacheMissing}
    <div class="missing-engine-banner">
      <div class="banner-icon-box" aria-hidden="true">
        <AlertCircle size={18} strokeWidth={2.2} />
      </div>
      <div class="banner-content">
        <span class="banner-title">Componente Apache Requerido</span>
        <p class="banner-text">
          Se necesita el servidor web Apache base para servir tus sitios en el puerto local.
        </p>
      </div>
      {#if compatibility.recommended.length > 0}
        <Button.Root
          class="primary-button-sm banner-action"
          type="button"
          disabled={isInstalling}
          onclick={() => void onInstallVersion("Apache", compatibility.recommended[0])}
        >
          <Download size={14} strokeWidth={2} aria-hidden="true" />
          <span>Instalar Servidor Web</span>
        </Button.Root>
      {/if}
    </div>
  {/if}

  <PhpWebStackOverview
    localUrl={RuntimePath.localUrl}
    documentRoot={RuntimePath.documentRoot}
    activeProfileName={activeProfile()?.name ?? "Default Stack"}
    onOpenUrl={openRuntimeUrl}
    onOpenPath={openRuntimePath}
  />

  <PhpStackProfilesPanel
    profiles={profileStore.profiles}
    {activePhpVersion}
    onCreate={openCreateProfileModal}
    onSelect={handleSelectProfile}
    onEdit={openEditProfileModal}
    onDelete={handleDeleteProfile}
  />
</section>

<PhpStackProfileDialog
  bind:open={isProfileModalOpen}
  {editingProfile}
  bind:name={profileFormName}
  bind:phpVersion={profileFormPhp}
  bind:apacheVersion={profileFormApache}
  installedPhpVersions={cleanInstalledPhp}
  installedApacheVersions={cleanInstalledApache}
  {phpDownloadOptions}
  {apacheDownloadOptions}
  isPhpMissing={isSelectedPhpNeedsInstall}
  isApacheMissing={isSelectedApacheNeedsInstall}
  {isInstalling}
  {installProgress}
  error={profileFormError}
  onSubmit={handleSaveProfile}
/>

<DeleteRuntimeDialog
  open={Boolean(pendingDelete)}
  serviceLabel={pendingDelete?.service ?? ""}
  version={pendingDelete?.version ?? ""}
  onOpenChange={(open) => {
    if (!open) pendingDelete = null;
  }}
  onConfirm={confirmDelete}
/>

<style>
  .editor-panel {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding: 24px;
    width: 100%;
  }

  .editor-header {
    align-items: flex-start;
    border-bottom: 1px solid var(--color-boulder-100);
    display: flex;
    justify-content: space-between;
    padding-bottom: 18px;
    gap: 20px;
  }

  .header-main {
    min-width: 0;
  }

  .eyebrow {
    color: var(--color-east-bay-700);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.08em;
    margin: 0 0 6px;
    text-transform: uppercase;
  }

  h2 {
    color: var(--color-boulder-950);
    font-size: 22px;
    font-weight: 650;
    line-height: 1.2;
    margin: 0;
  }

  .header-sub {
    color: var(--color-boulder-600);
    font-size: 13.5px;
    line-height: 1.5;
    margin: 6px 0 0;
  }

  .header-controls {
    align-items: center;
    display: flex;
    flex-shrink: 0;
    gap: 12px;
  }

  .status-pill {
    align-items: center;
    background: var(--color-boulder-100);
    border: 1px solid var(--color-boulder-200);
    border-radius: 999px;
    color: var(--color-boulder-700);
    display: inline-flex;
    font-size: 12px;
    font-weight: 600;
    gap: 7px;
    padding: 6px 12px;
  }

  .status-pill.running {
    background: #ecfdf5;
    border-color: #a7f3d0;
    color: #065f46;
  }

  .status-indicator {
    background: #9ca3af;
    border-radius: 50%;
    height: 7px;
    width: 7px;
  }

  .status-pill.running .status-indicator {
    background: #10b981;
    box-shadow: 0 0 0 3px rgb(16 185 129 / 20%);
  }

  :global(.toggle-service-btn) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 6px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 13px;
    font-weight: 600;
    gap: 7px;
    height: 36px;
    padding: 0 14px;
    transition: background-color 150ms ease;
  }

  :global(.toggle-service-btn:hover:not(:disabled)) {
    background: var(--color-east-bay-950);
  }

  :global(.toggle-service-btn.is-stop) {
    background: #b91c1c;
  }

  :global(.toggle-service-btn.is-stop:hover:not(:disabled)) {
    background: #991b1b;
  }

  :global(.toggle-service-btn:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Missing Engine Banner */
  .missing-engine-banner {
    align-items: center;
    background: #fffbeb;
    border: 1px solid #fde68a;
    border-radius: 8px;
    display: flex;
    gap: 12px;
    padding: 12px 16px;
  }

  .banner-icon-box {
    color: #d97706;
    flex-shrink: 0;
  }

  .banner-content {
    flex: 1;
    min-width: 0;
  }

  .banner-title {
    color: #92400e;
    display: block;
    font-size: 12px;
    font-weight: 700;
    letter-spacing: 0.02em;
    text-transform: uppercase;
  }

  .banner-text {
    color: #b45309;
    font-size: 12.5px;
    line-height: 1.4;
    margin: 2px 0 0;
  }

  :global(.banner-action) {
    flex-shrink: 0;
  }

  /* Environment Overview Card */
  /* Profiles Card */
  :global(.secondary-button) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12.5px;
    font-weight: 600;
    gap: 6px;
    height: 32px;
    padding: 0 12px;
    transition:
      background-color 150ms ease,
      border-color 150ms ease;
  }

  :global(.secondary-button:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  :global(.primary-button-sm) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 0;
    border-radius: 6px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-size: 12.5px;
    font-weight: 600;
    gap: 6px;
    height: 32px;
    padding: 0 14px;
  }

  :global(.primary-button-sm:hover:not(:disabled)) {
    background: var(--color-east-bay-950);
  }

  @media (max-width: 720px) {
    .editor-header {
      flex-direction: column;
    }
  }
</style>
