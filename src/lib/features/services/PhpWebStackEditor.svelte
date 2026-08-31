<script lang="ts">
  import {
    AlertCircle,
    Check,
    ChevronDown,
    Download,
    ExternalLink,
    Folder,
    FolderOpen,
    Globe,
    Layers,
    Loader2,
    Pencil,
    Play,
    Plus,
    Server,
    Square,
    Terminal,
    Trash2,
    X
  } from "@lucide/svelte";
  import { Button, Dialog, Select, Separator, Tooltip } from "bits-ui";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import DeleteRuntimeDialog from "./DeleteRuntimeDialog.svelte";
  import InstallVersionDialog from "./InstallVersionDialog.svelte";
  import {
    cleanVersion,
    getCompatibleApacheVersions,
    parseVersionString,
    type PhpStackProfile
  } from "./types";

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
    onDeleteVersion
  }: PhpWebStackEditorProps = $props();

  const STORAGE_KEY = "harbor_php_stack_profiles";

  let isProfileModalOpen = $state(false);
  let editingProfile = $state<PhpStackProfile | null>(null);

  let profileFormName = $state("");
  let profileFormPhp = $state("");
  let profileFormApache = $state("");
  let profileFormError = $state("");

  let pendingDelete = $state<{ service: "PHP" | "Apache"; version: string } | null>(null);

  // Profiles State loaded from localStorage or initialized
  let profiles = $state<PhpStackProfile[]>(loadSavedProfiles());

  function loadSavedProfiles(): PhpStackProfile[] {
    if (typeof window === "undefined") return [];
    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      if (raw) {
        const parsed = JSON.parse(raw) as PhpStackProfile[];
        if (Array.isArray(parsed) && parsed.length > 0) return parsed;
      }
    } catch {
      // Fallback
    }
    return [];
  }

  function saveProfiles(updated: PhpStackProfile[]) {
    profiles = updated;
    if (typeof window !== "undefined") {
      try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(updated));
      } catch {
        // Storage failure
      }
    }
  }

  // Ensure default profiles exist when runtimes are available
  $effect(() => {
    if (profiles.length === 0 && (installedPhpVersions.length > 0 || activePhpVersion)) {
      const defaultPhp = activePhpVersion ? cleanVersion(activePhpVersion) : cleanVersion(installedPhpVersions[0] ?? "8.3.17");
      const defaultApache = activeApacheVersion ? cleanVersion(activeApacheVersion) : cleanVersion(installedApacheVersions[0] ?? "2.4.62");
      
      const initial: PhpStackProfile[] = [
        {
          id: "default-stack",
          name: "Default Web Stack",
          phpVersion: defaultPhp,
          apacheVersion: defaultApache,
          isDefault: true
        }
      ];

      // Add extra profiles if there are multiple PHP versions
      installedPhpVersions.forEach((phpVer) => {
        const clean = cleanVersion(phpVer);
        if (clean !== defaultPhp) {
          initial.push({
            id: `stack-php-${clean.replace(/\./g, "-")}`,
            name: `PHP ${clean} Environment`,
            phpVersion: clean,
            apacheVersion: defaultApache
          });
        }
      });

      saveProfiles(initial);
    }
  });

  let cleanInstalledPhp = $derived(installedPhpVersions.map(cleanVersion));
  let cleanInstalledApache = $derived(installedApacheVersions.map(cleanVersion));

  let phpDownloadOptions = $derived(
    availablePhpVersions
      .map((v) => {
        const meta = parseVersionString(v);
        return { version: meta.version, label: `PHP ${meta.version} (${meta.channel || "Disponible"})` };
      })
      .filter((opt) => !cleanInstalledPhp.includes(opt.version))
  );

  let apacheDownloadOptions = $derived(
    availableApacheVersions
      .map((v) => {
        const meta = parseVersionString(v);
        return { version: meta.version, label: `Apache ${meta.version} (${meta.channel || "Disponible"})` };
      })
      .filter((opt) => !cleanInstalledApache.includes(opt.version))
  );

  let isSelectedPhpNeedsInstall = $derived(
    profileFormPhp ? !cleanInstalledPhp.includes(cleanVersion(profileFormPhp)) : false
  );

  let isSelectedApacheNeedsInstall = $derived(
    profileFormApache ? !cleanInstalledApache.includes(cleanVersion(profileFormApache)) : false
  );

  let cleanActivePhp = $derived(
    activePhpVersion ? cleanVersion(activePhpVersion) : null
  );

  let cleanActiveApache = $derived(
    activeApacheVersion ? cleanVersion(activeApacheVersion) : null
  );

  let activeProfile = $derived(() => {
    if (!cleanActivePhp) return profiles[0] ?? null;
    return profiles.find((p) => p.phpVersion === cleanActivePhp) ?? profiles[0] ?? null;
  });

  let isApacheMissing = $derived(installedApacheVersions.length === 0);

  let compatibility = $derived(
    getCompatibleApacheVersions(activePhpVersion, availableApacheVersions)
  );

  function openCreateProfileModal() {
    editingProfile = null;
    profileFormName = "";
    profileFormPhp = cleanActivePhp ?? cleanInstalledPhp[0] ?? (availablePhpVersions[0] ? cleanVersion(availablePhpVersions[0]) : "8.3.17");
    profileFormApache = cleanActiveApache ?? cleanInstalledApache[0] ?? (availableApacheVersions[0] ? cleanVersion(availableApacheVersions[0]) : "2.4.62");
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
      const updated = profiles.map((p) =>
        p.id === editingProfile?.id
          ? {
              ...p,
              name: profileFormName.trim(),
              phpVersion: cleanPhp,
              apacheVersion: cleanApache
            }
          : p
      );
      saveProfiles(updated);
    } else {
      const newProfile: PhpStackProfile = {
        id: `profile-${Date.now()}`,
        name: profileFormName.trim(),
        phpVersion: cleanPhp,
        apacheVersion: cleanApache
      };
      saveProfiles([...profiles, newProfile]);
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
    if (profiles.length <= 1) return;
    const updated = profiles.filter((p) => p.id !== id);
    saveProfiles(updated);
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

  async function handleOpenUrl(url: string) {
    try {
      if (isTauri()) {
        await openUrl(url);
      } else {
        window.open(url, "_blank");
      }
    } catch (err) {
      console.error("Error al abrir URL:", err);
    }
  }

  async function handleOpenPath(path: string) {
    try {
      if (isTauri()) {
        await invoke("open_directory", { path });
      }
    } catch (err) {
      console.error("Error al abrir ruta:", err);
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
        Perfiles de desarrollo con PHP y Apache integrados para ejecutar aplicaciones web y comandos CLI locales.
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

  <!-- Active Environment Hero Card -->
  <div class="env-overview-card">
    <div class="overview-item">
      <div class="overview-label-row">
        <Globe size={14} strokeWidth={2} class="overview-icon" />
        <span class="overview-label">URL Local</span>
      </div>
      <button
        type="button"
        class="overview-link-button"
        onclick={() => void handleOpenUrl("http://localhost")}
        title="Abrir http://localhost en el navegador"
      >
        <span class="overview-value">http://localhost</span>
        <ExternalLink size={13} strokeWidth={2} class="overview-action-icon" aria-hidden="true" />
      </button>
    </div>

    <div class="overview-divider" aria-hidden="true"></div>

    <div class="overview-item">
      <div class="overview-label-row">
        <Folder size={14} strokeWidth={2} class="overview-icon" />
        <span class="overview-label">Carpeta Web (Document Root)</span>
      </div>
      <button
        type="button"
        class="overview-link-button"
        onclick={() => void handleOpenPath("C:\\Harbor\\www")}
        title="Abrir C:\Harbor\www en el explorador de archivos"
      >
        <span class="overview-value">C:\Harbor\www</span>
        <FolderOpen size={13} strokeWidth={2} class="overview-action-icon" aria-hidden="true" />
      </button>
    </div>

    <div class="overview-divider" aria-hidden="true"></div>

    <div class="overview-item">
      <div class="overview-label-row">
        <Layers size={14} strokeWidth={2} class="overview-icon" />
        <span class="overview-label">Perfil Activo</span>
      </div>
      <span class="overview-value highlight">
        {activeProfile()?.name ?? "Default Stack"}
      </span>
    </div>
  </div>

  <!-- Profiles Management Section -->
  <div class="profiles-card">
    <div class="card-header">
      <div class="card-title-group">
        <div class="card-icon layers-icon" aria-hidden="true">
          <Layers size={18} strokeWidth={2.2} />
        </div>
        <div>
          <h3>Perfiles de Entorno Web</h3>
          <p>Configura pares de PHP y Apache para alternar de stack según el proyecto.</p>
        </div>
      </div>

      <div class="header-actions">
        <Button.Root
          class="primary-button-sm"
          type="button"
          onclick={openCreateProfileModal}
        >
          <Plus size={14} strokeWidth={2.2} aria-hidden="true" />
          <span>Nuevo Perfil</span>
        </Button.Root>
      </div>
    </div>

    {#if profiles.length === 0}
      <div class="empty-runtime-box">
        <p>No tienes perfiles configurados.</p>
        <Button.Root
          class="primary-button-sm"
          type="button"
          onclick={openCreateProfileModal}
        >
          <Plus size={14} strokeWidth={2.2} aria-hidden="true" />
          <span>Crear Primer Perfil</span>
        </Button.Root>
      </div>
    {:else}
      <div class="profiles-list">
        {#each profiles as profile (profile.id)}
          {@const isActive = profile.phpVersion === cleanActivePhp}
          <div
            role="button"
            tabindex="0"
            class={`profile-row${isActive ? " is-active" : ""}`}
            onclick={() => handleSelectProfile(profile)}
            onkeydown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                handleSelectProfile(profile);
              }
            }}
          >
            <!-- Radio Selector -->
            <div class="profile-radio" aria-hidden="true">
              <span class={`radio-circle${isActive ? " checked" : ""}`}></span>
            </div>

            <!-- Profile Info -->
            <div class="profile-main-info">
              <div class="profile-title-row">
                <span class="profile-name">{profile.name}</span>
                {#if isActive}
                  <span class="active-badge">
                    <Check size={11} strokeWidth={2.8} aria-hidden="true" />
                    <span>Activo · CLI & Web</span>
                  </span>
                {/if}
              </div>

              <!-- Compact Stack Badges -->
              <div class="profile-stack-badges">
                <span class="stack-badge php">
                  <Terminal size={12} strokeWidth={2} />
                  <span>PHP {profile.phpVersion}</span>
                </span>
                <span class="stack-badge apache">
                  <Server size={12} strokeWidth={2} />
                  <span>Apache {profile.apacheVersion}</span>
                </span>
              </div>
            </div>

            <!-- Profile Actions -->
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="profile-actions" onclick={(e) => e.stopPropagation()}>
              <Tooltip.Root>
                <Tooltip.Trigger
                  class="action-btn edit-btn"
                  type="button"
                  aria-label={`Editar perfil ${profile.name}`}
                  onclick={(e: MouseEvent) => openEditProfileModal(profile, e)}
                >
                  <Pencil size={13.5} strokeWidth={2.2} aria-hidden="true" />
                </Tooltip.Trigger>
                <Tooltip.Portal>
                  <Tooltip.Content class="tooltip-content" sideOffset={6}>
                    Editar perfil
                  </Tooltip.Content>
                </Tooltip.Portal>
              </Tooltip.Root>

              {#if profiles.length > 1}
                <Tooltip.Root>
                  <Tooltip.Trigger
                    class="action-btn delete-btn"
                    type="button"
                    aria-label={`Eliminar perfil ${profile.name}`}
                    onclick={(e: MouseEvent) => handleDeleteProfile(profile.id, e)}
                  >
                    <Trash2 size={13.5} strokeWidth={2.2} aria-hidden="true" />
                  </Tooltip.Trigger>
                  <Tooltip.Portal>
                    <Tooltip.Content class="tooltip-content" sideOffset={6}>
                      Eliminar perfil
                    </Tooltip.Content>
                  </Tooltip.Portal>
                </Tooltip.Root>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</section>

<!-- Unified Profile & Runtime Dialog -->
<Dialog.Root bind:open={isProfileModalOpen}>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-overlay" />
    <Dialog.Content class="modal-content" aria-labelledby="profile-modal-title">
      <div class="modal-header">
        <div>
          <Dialog.Title id="profile-modal-title" class="modal-title">
            {editingProfile ? "Editar Perfil de Stack" : "Nuevo Perfil de Entorno"}
          </Dialog.Title>
          <Dialog.Description class="modal-desc">
            Configura el par de PHP y Apache para este stack. Si eliges una versión no descargada, se instalará automáticamente.
          </Dialog.Description>
        </div>
        <Dialog.Close class="modal-close-btn" aria-label="Cerrar">
          <X size={16} strokeWidth={2} />
        </Dialog.Close>
      </div>

      {#if profileFormError}
        <div class="form-error-banner">
          <span>{profileFormError}</span>
        </div>
      {/if}

      <div class="modal-form">
        <div class="form-group">
          <label for="profile-name-input" class="form-label">Nombre del Perfil</label>
          <input
            id="profile-name-input"
            type="text"
            class="form-input"
            placeholder="ej. Laravel 11 Project, Legacy PHP 7.4"
            bind:value={profileFormName}
          />
        </div>

        <div class="form-grid-two">
          <!-- PHP Selector (Bits-UI: Installed + Downloadable) -->
          <div class="form-group">
            <div class="field-label-row">
              <span class="form-label">Versión de PHP</span>
              {#if isSelectedPhpNeedsInstall}
                <span class="auto-download-tag">Se descargará</span>
              {/if}
            </div>

            <Select.Root
              type="single"
              bind:value={profileFormPhp}
            >
              <Select.Trigger class="bits-select-trigger" aria-label="Seleccionar versión de PHP">
                <span class="select-value-text">
                  {profileFormPhp ? `PHP ${profileFormPhp}` : "Seleccionar PHP"}
                </span>
                <ChevronDown size={14} strokeWidth={2.2} class="select-chevron" />
              </Select.Trigger>
              <Select.Portal>
                <Select.Content class="bits-select-content" sideOffset={5} align="start">
                  <Select.Viewport class="bits-select-viewport">
                    {#if cleanInstalledPhp.length > 0}
                      <Select.Group>
                        <Select.GroupHeading class="bits-select-group-label">Versiones Instaladas</Select.GroupHeading>
                        {#each cleanInstalledPhp as phpVer}
                          <Select.Item class="bits-select-item" value={phpVer} label={`PHP ${phpVer}`}>
                            {#snippet children({ selected })}
                              <span class="item-label">PHP {phpVer} (Instalada ✓)</span>
                              {#if selected}
                                <Check size={14} strokeWidth={2.4} class="item-check" />
                              {/if}
                            {/snippet}
                          </Select.Item>
                        {/each}
                      </Select.Group>
                    {/if}

                    {#if phpDownloadOptions.length > 0}
                      <Separator.Root class="bits-select-separator" />
                      <Select.Group>
                        <Select.GroupHeading class="bits-select-group-label">Descargar e Instalar</Select.GroupHeading>
                        {#each phpDownloadOptions as opt}
                          <Select.Item class="bits-select-item download-item" value={opt.version} label={opt.label}>
                            {#snippet children({ selected })}
                              <span class="item-label">⬇️ {opt.label}</span>
                              {#if selected}
                                <Check size={14} strokeWidth={2.4} class="item-check" />
                              {/if}
                            {/snippet}
                          </Select.Item>
                        {/each}
                      </Select.Group>
                    {/if}
                  </Select.Viewport>
                </Select.Content>
              </Select.Portal>
            </Select.Root>
          </div>

          <!-- Apache Selector (Bits-UI: Installed + Downloadable) -->
          <div class="form-group">
            <div class="field-label-row">
              <span class="form-label">Versión de Apache</span>
              {#if isSelectedApacheNeedsInstall}
                <span class="auto-download-tag">Se descargará</span>
              {/if}
            </div>

            <Select.Root
              type="single"
              bind:value={profileFormApache}
            >
              <Select.Trigger class="bits-select-trigger" aria-label="Seleccionar versión de Apache">
                <span class="select-value-text">
                  {profileFormApache ? `Apache ${profileFormApache}` : "Seleccionar Apache"}
                </span>
                <ChevronDown size={14} strokeWidth={2.2} class="select-chevron" />
              </Select.Trigger>
              <Select.Portal>
                <Select.Content class="bits-select-content" sideOffset={5} align="start">
                  <Select.Viewport class="bits-select-viewport">
                    {#if cleanInstalledApache.length > 0}
                      <Select.Group>
                        <Select.GroupHeading class="bits-select-group-label">Versiones Instaladas</Select.GroupHeading>
                        {#each cleanInstalledApache as apacheVer}
                          <Select.Item class="bits-select-item" value={apacheVer} label={`Apache ${apacheVer}`}>
                            {#snippet children({ selected })}
                              <span class="item-label">Apache {apacheVer} (Instalada ✓)</span>
                              {#if selected}
                                <Check size={14} strokeWidth={2.4} class="item-check" />
                              {/if}
                            {/snippet}
                          </Select.Item>
                        {/each}
                      </Select.Group>
                    {/if}

                    {#if apacheDownloadOptions.length > 0}
                      <Separator.Root class="bits-select-separator" />
                      <Select.Group>
                        <Select.GroupHeading class="bits-select-group-label">Descargar e Instalar</Select.GroupHeading>
                        {#each apacheDownloadOptions as opt}
                          <Select.Item class="bits-select-item download-item" value={opt.version} label={opt.label}>
                            {#snippet children({ selected })}
                              <span class="item-label">⬇️ {opt.label}</span>
                              {#if selected}
                                <Check size={14} strokeWidth={2.4} class="item-check" />
                              {/if}
                            {/snippet}
                          </Select.Item>
                        {/each}
                      </Select.Group>
                    {/if}
                  </Select.Viewport>
                </Select.Content>
              </Select.Portal>
            </Select.Root>
          </div>
        </div>

        {#if isSelectedPhpNeedsInstall || isSelectedApacheNeedsInstall}
          <div class="download-notice">
            <Download size={14} strokeWidth={2} class="download-notice-icon" />
            <span>Las versiones marcadas se descargarán e instalarán automáticamente al guardar.</span>
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <Button.Root
          type="button"
          class="secondary-button"
          disabled={isInstalling}
          onclick={() => (isProfileModalOpen = false)}
        >
          Cancelar
        </Button.Root>
        <Button.Root
          type="button"
          class="primary-button-sm"
          disabled={isInstalling}
          onclick={handleSaveProfile}
        >
          {#if isInstalling}
            <Loader2 size={14} class="spin" />
            <span>Instalando... {installProgress > 0 ? `${installProgress}%` : ""}</span>
          {:else}
            <span>{editingProfile ? "Guardar Cambios" : "Crear Perfil"}</span>
          {/if}
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<DeleteRuntimeDialog
  open={Boolean(pendingDelete)}
  serviceLabel={pendingDelete?.service ?? ""}
  version={pendingDelete?.version ?? ""}
  onOpenChange={(open) => { if (!open) pendingDelete = null; }}
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
  .env-overview-card {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    gap: 16px;
    padding: 14px 18px;
  }

  .overview-item {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .overview-label-row {
    align-items: center;
    color: var(--color-boulder-500);
    display: flex;
    gap: 6px;
  }

  :global(.overview-icon) {
    color: var(--color-boulder-400);
  }

  .overview-label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.03em;
    text-transform: uppercase;
  }

  .overview-link-button {
    align-items: center;
    appearance: none;
    -webkit-appearance: none;
    background: transparent;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    display: inline-flex;
    gap: 6px;
    margin: 0;
    max-width: 100%;
    outline: none;
    padding: 0;
    text-align: left;
  }

  .overview-link-button:hover .overview-value {
    color: var(--color-east-bay-700);
    text-decoration: underline;
  }

  .overview-link-button:hover :global(.overview-action-icon) {
    color: var(--color-east-bay-700);
    transform: translate(1px, -1px);
  }

  .overview-link-button:focus-visible {
    outline: 2px solid var(--color-east-bay-400);
    outline-offset: 2px;
  }

  :global(.overview-action-icon) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
    transition: color 0.15s ease, transform 0.15s ease;
  }

  .overview-value {
    color: var(--color-boulder-900);
    font-family: var(--font-mono);
    font-size: 13px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .overview-value.highlight {
    color: var(--color-east-bay-900);
    font-family: inherit;
    font-weight: 700;
  }

  .overview-divider {
    background: var(--color-boulder-200);
    height: 32px;
    width: 1px;
  }

  /* Profiles Card */
  .profiles-card {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding: 18px 20px;
  }

  .card-header {
    align-items: flex-start;
    display: flex;
    justify-content: space-between;
    gap: 12px;
  }

  .card-title-group {
    align-items: flex-start;
    display: flex;
    gap: 12px;
  }

  .card-icon {
    align-items: center;
    border-radius: 8px;
    display: flex;
    flex-shrink: 0;
    height: 36px;
    justify-content: center;
    width: 36px;
  }

  .layers-icon {
    background: var(--color-east-bay-100);
    color: var(--color-east-bay-800);
  }

  .card-title-group h3 {
    color: var(--color-boulder-950);
    font-size: 15px;
    font-weight: 650;
    margin: 0;
  }

  .card-title-group p {
    color: var(--color-boulder-600);
    font-size: 12.5px;
    line-height: 1.4;
    margin: 3px 0 0;
  }

  .header-actions {
    align-items: center;
    display: flex;
    gap: 8px;
  }

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
    transition: background-color 150ms ease, border-color 150ms ease;
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

  .empty-runtime-box {
    align-items: center;
    background: #ffffff;
    border: 1px dashed var(--color-boulder-300);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 22px;
    text-align: center;
  }

  .empty-runtime-box p {
    color: var(--color-boulder-500);
    font-size: 13px;
    margin: 0;
  }

  .profiles-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  /* Profile Row (Interactive Radio Tile) */
  .profile-row {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    gap: 14px;
    padding: 12px 16px;
    position: relative;
    transition: border-color 150ms ease, box-shadow 150ms ease, background-color 150ms ease;
  }

  .profile-row:hover {
    background: #fafafa;
    border-color: var(--color-boulder-300);
  }

  .profile-row.is-active {
    background: #ffffff;
    border-color: var(--color-east-bay-500);
    box-shadow: 0 1px 4px rgb(15 23 42 / 6%);
  }

  .profile-radio {
    display: flex;
    flex-shrink: 0;
  }

  .radio-circle {
    border: 2px solid var(--color-boulder-300);
    border-radius: 50%;
    box-sizing: border-box;
    display: block;
    height: 16px;
    position: relative;
    transition: border-color 150ms ease, background-color 150ms ease;
    width: 16px;
  }

  .profile-row:hover .radio-circle {
    border-color: var(--color-boulder-400);
  }

  .radio-circle.checked {
    border-color: var(--color-east-bay-900);
    background-color: var(--color-east-bay-900);
  }

  .radio-circle.checked::after {
    background: #ffffff;
    border-radius: 50%;
    content: "";
    height: 6px;
    left: 3px;
    position: absolute;
    top: 3px;
    width: 6px;
  }

  .profile-main-info {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 5px;
    min-width: 0;
  }

  .profile-title-row {
    align-items: center;
    display: flex;
    gap: 10px;
  }

  .profile-name {
    color: var(--color-boulder-950);
    font-size: 14px;
    font-weight: 650;
  }

  .active-badge {
    align-items: center;
    background: var(--color-east-bay-100);
    border-radius: 999px;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font-size: 11px;
    font-weight: 700;
    gap: 4px;
    letter-spacing: 0.02em;
    padding: 2px 8px;
  }

  .profile-stack-badges {
    align-items: center;
    display: flex;
    gap: 8px;
  }

  .stack-badge {
    align-items: center;
    border-radius: 4px;
    display: inline-flex;
    font-family: var(--font-mono);
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
    padding: 2px 7px;
  }

  .stack-badge.php {
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    color: var(--color-east-bay-800);
  }

  .stack-badge.apache {
    background: #fefce8;
    border: 1px solid #fef08a;
    color: #854d0e;
  }

  .profile-actions {
    align-items: center;
    display: flex;
    gap: 6px;
    margin-left: auto;
    opacity: 0.85;
    transition: opacity 150ms ease;
  }

  .profile-row:hover .profile-actions {
    opacity: 1;
  }

  :global(.action-btn) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    color: var(--color-boulder-600);
    cursor: pointer;
    display: inline-flex;
    height: 30px;
    justify-content: center;
    padding: 0;
    transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease, transform 100ms ease;
    width: 30px;
  }

  :global(.action-btn:hover) {
    background: var(--color-boulder-100);
    border-color: var(--color-boulder-300);
    color: var(--color-boulder-900);
  }

  :global(.action-btn.delete-btn:hover) {
    background: #fef2f2;
    border-color: #fecaca;
    color: #dc2626;
  }

  :global(.action-btn:active) {
    transform: scale(0.95);
  }

  /* Modal Styles */
  :global(.modal-overlay) {
    background: rgb(0 0 0 / 40%);
    inset: 0;
    position: fixed;
    z-index: 50;
  }

  :global(.modal-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    display: flex;
    flex-direction: column;
    gap: 18px;
    left: 50%;
    max-width: 480px;
    padding: 24px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: calc(100vw - 32px);
    z-index: 51;
  }

  .modal-header {
    align-items: flex-start;
    display: flex;
    justify-content: space-between;
    gap: 12px;
  }

  :global(.modal-title) {
    color: var(--color-boulder-950);
    font-size: 16px;
    font-weight: 650;
    margin: 0;
  }

  :global(.modal-desc) {
    color: var(--color-boulder-600);
    font-size: 13px;
    line-height: 1.4;
    margin: 4px 0 0;
  }

  :global(.modal-close-btn) {
    background: transparent;
    border: 0;
    border-radius: 4px;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: flex;
    padding: 4px;
    transition: color 150ms ease;
  }

  :global(.modal-close-btn:hover) {
    color: var(--color-boulder-800);
  }

  .form-error-banner {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 6px;
    color: #b91c1c;
    font-size: 12.5px;
    padding: 8px 12px;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-label-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }

  .auto-download-tag {
    background: #ecfdf5;
    border-radius: 4px;
    color: #065f46;
    font-size: 10.5px;
    font-weight: 600;
    padding: 1px 6px;
  }

  .form-grid-two {
    display: grid;
    gap: 12px;
    grid-template-columns: 1fr 1fr;
  }

  .form-label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 600;
  }

  .form-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 13px;
    height: 36px;
    padding: 0 10px;
    width: 100%;
    transition: border-color 150ms ease;
  }

  .form-input:focus {
    border-color: var(--color-east-bay-600);
    outline: none;
  }

  /* Bits-UI Select Styles */
  :global(.bits-select-trigger) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    cursor: pointer;
    display: flex;
    font: inherit;
    font-size: 13px;
    height: 36px;
    justify-content: space-between;
    padding: 0 10px;
    transition: border-color 150ms ease;
    width: 100%;
  }

  :global(.bits-select-trigger:focus-visible) {
    border-color: var(--color-east-bay-600);
    outline: none;
  }

  .select-value-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.select-chevron) {
    color: var(--color-boulder-500);
    flex-shrink: 0;
  }

  :global(.bits-select-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 10px 25px rgb(0 0 0 / 12%);
    max-height: 240px;
    min-width: 220px;
    overflow-y: auto;
    padding: 4px;
    z-index: 80;
  }

  :global(.bits-select-viewport) {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  :global(.bits-select-group-label) {
    color: var(--color-boulder-500);
    font-size: 10.5px;
    font-weight: 700;
    letter-spacing: 0.04em;
    padding: 6px 8px 3px;
    text-transform: uppercase;
  }

  :global(.bits-select-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }

  :global(.bits-select-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: flex;
    font-size: 12.5px;
    justify-content: space-between;
    outline: none;
    padding: 6px 8px;
    transition: background-color 100ms ease;
    user-select: none;
  }

  :global(.bits-select-item[data-highlighted]) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  :global(.bits-select-item[data-selected]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-900);
    font-weight: 600;
  }

  :global(.bits-select-item.download-item) {
    color: var(--color-east-bay-700);
  }

  .item-label {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.item-check) {
    color: var(--color-east-bay-800);
    flex-shrink: 0;
  }

  .download-notice {
    align-items: center;
    background: #f0fdf4;
    border: 1px solid #bbf7d0;
    border-radius: 6px;
    color: #15803d;
    display: flex;
    font-size: 12px;
    gap: 8px;
    padding: 8px 12px;
  }

  :global(.download-notice-icon) {
    color: #16a34a;
    flex-shrink: 0;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding-top: 6px;
  }

  :global(.tooltip-content) {
    background: var(--color-boulder-900);
    border-radius: 4px;
    color: #ffffff;
    font-size: 11px;
    padding: 5px 8px;
    z-index: 60;
    box-shadow: 0 4px 12px rgb(0 0 0 / 15%);
  }

  :global(.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  @media (max-width: 720px) {
    .editor-header {
      flex-direction: column;
    }

    .env-overview-card {
      flex-direction: column;
      align-items: stretch;
    }

    .overview-divider {
      display: none;
    }

    .form-grid-two {
      grid-template-columns: 1fr;
    }
  }
</style>

