<script lang="ts">
  import { AlertCircle, Download, Play, Square } from "@lucide/svelte";
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

<section
  class="flex w-full flex-col gap-5 rounded-lg border border-boulder-200 bg-white p-6"
  aria-labelledby="web-stack-title"
>
  <header
    class="flex items-start justify-between gap-5 border-b border-boulder-100 pb-4.5 max-md:flex-col"
  >
    <div class="min-w-0">
      <div>
        <h2 id="web-stack-title" class="text-[22px] leading-tight font-semibold text-boulder-950">
          PHP & Servidor Web
        </h2>
      </div>
      <p class="mt-1.5 text-[13.5px] leading-relaxed text-boulder-600">
        Perfiles de desarrollo con PHP y Apache integrados para ejecutar aplicaciones web y comandos
        CLI locales.
      </p>
    </div>

    <div class="flex shrink-0 items-center gap-3">
      <div
        class={`inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-semibold ${isPhpRunning ? "border-emerald-200 bg-emerald-50 text-emerald-800" : "border-boulder-200 bg-boulder-100 text-boulder-700"}`}
      >
        <span
          class={`size-1.5 rounded-full ${isPhpRunning ? "bg-emerald-500 ring-3 ring-emerald-500/20" : "bg-boulder-400"}`}
          aria-hidden="true"
        ></span>
        <span>{isPhpRunning ? "Servidor Activo" : "Servidor Detenido"}</span>
      </div>

      <button
        class={`inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-md px-3.5 text-[13px] font-semibold text-white disabled:cursor-not-allowed disabled:opacity-50 ${isPhpRunning ? "bg-red-700 hover:bg-red-800" : "bg-east-bay-900 hover:bg-east-bay-950"}`}
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
      </button>
    </div>
  </header>

  <!-- Missing Apache Web Engine Notice (Only when required) -->
  {#if isApacheMissing}
    <div class="flex items-center gap-3 rounded-lg border border-amber-200 bg-amber-50 px-4 py-3">
      <div class="shrink-0 text-amber-600" aria-hidden="true">
        <AlertCircle size={18} strokeWidth={2.2} />
      </div>
      <div class="min-w-0 flex-1">
        <span class="block text-[13px] font-semibold text-amber-900"
          >Componente Apache requerido</span
        >
        <p class="mt-0.5 text-[12.5px] leading-snug text-amber-700">
          Se necesita el servidor web Apache base para servir tus sitios en el puerto local.
        </p>
      </div>
      {#if compatibility.recommended.length > 0}
        <button
          class="inline-flex h-8 shrink-0 cursor-pointer items-center gap-1.5 rounded-md bg-east-bay-900 px-3.5 text-[12.5px] font-semibold text-white hover:bg-east-bay-950 disabled:opacity-50"
          type="button"
          disabled={isInstalling}
          onclick={() => void onInstallVersion("Apache", compatibility.recommended[0])}
        >
          <Download size={14} strokeWidth={2} aria-hidden="true" />
          <span>Instalar Servidor Web</span>
        </button>
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
