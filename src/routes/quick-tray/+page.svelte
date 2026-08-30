<script lang="ts">
  import { invoke, isTauri } from "@tauri-apps/api/core";
  import { listen, emit } from "@tauri-apps/api/event";
  import {
    AlertTriangle,
    Anchor,
    Boxes,
    Check,
    ChevronDown,
    Copy,
    Ellipsis,
    Eye,
    EyeOff,
    KeyRound,
    Maximize2,
    Pencil,
    Play,
    Plus,
    Search,
    ShieldAlert,
    Square,
    Trash2,
    X,
  } from "@lucide/svelte";
  import {
    AlertDialog,
    Button,
    Dialog,
    DropdownMenu,
    ScrollArea,
    Select,
    Switch,
    Tooltip,
  } from "bits-ui";
  import { onMount } from "svelte";
  import type {
    Profile,
    Secret,
    SecretsConfiguration,
  } from "$lib/features/secrets/types";

  type ServiceState = {
    installedVersions: string[];
    selectedVersion: string;
  };

  const starterProfiles: Profile[] = [
    {
      id: 1,
      name: "Pruebas",
      isProduction: false,
      secrets: [
        { id: 1, key: "API_URL", value: "https://api-pruebas.example.test" },
      ],
    },
    {
      id: 2,
      name: "Production",
      isProduction: true,
      secrets: [
        { id: 2, key: "API_URL", value: "https://api.example.com" },
        { id: 3, key: "API_TOKEN", value: "replace-with-a-secret" },
      ],
    },
  ];

  const isNativeApp = isTauri();
  let currentTab = $state<"secrets" | "services">("secrets");

  // Secrets state
  let profiles = $state<Profile[]>([]);
  let activeProfileId = $state<number | null>(null);
  let selectedProfileId = $state<number | null>(null);
  let selectedProfileIdStr = $state<string>("");
  let searchQuery = $state("");
  let revealedSecretIds = $state<number[]>([]);
  let copiedId = $state<string | null>(null);
  let isLoading = $state(true);
  let isSaving = $state(false);
  let isActivating = $state(false);
  let statusMessage = $state("");
  let errorMessage = $state("");

  // Services state
  let isPhpRunning = $state(false);
  let phpState = $state<ServiceState>({ installedVersions: [], selectedVersion: "" });
  let nodeState = $state<ServiceState>({ installedVersions: [], selectedVersion: "" });
  let apacheState = $state<ServiceState>({ installedVersions: [], selectedVersion: "" });
  let isServiceBusy = $state(false);

  // Variable Dialog State
  let isVariableDialogOpen = $state(false);
  let editingSecretId = $state<number | null>(null);
  let formKey = $state("");
  let formValue = $state("");
  let formError = $state("");

  // Profile Dialog State
  let isProfileDialogOpen = $state(false);
  let newProfileName = $state("");
  let newProfileIsProduction = $state(false);

  // Confirmations
  let isProductionDialogOpen = $state(false);
  let isDeleteVariableDialogOpen = $state(false);
  let pendingDeleteSecretId = $state<number | null>(null);

  let selectedProfile = $derived(
    profiles.find((p) => p.id === selectedProfileId),
  );
  let isCurrentProfileActive = $derived(
    selectedProfile ? selectedProfile.id === activeProfileId : false,
  );

  let filteredSecrets = $derived.by(() => {
    if (!selectedProfile) return [];
    const query = searchQuery.trim().toLowerCase();
    if (!query) return selectedProfile.secrets;
    return selectedProfile.secrets.filter(
      (s) =>
        s.key.toLowerCase().includes(query) ||
        s.value.toLowerCase().includes(query),
    );
  });

  $effect(() => {
    if (selectedProfileId !== null) {
      selectedProfileIdStr = String(selectedProfileId);
    }
  });

  onMount(() => {
    void loadConfiguration();
    void loadServicesState();

    const handleFocus = () => {
      void loadConfiguration();
      void loadServicesState();
    };
    window.addEventListener("focus", handleFocus);

    let unlisten: (() => void) | undefined;
    if (isNativeApp) {
      listen("secrets-updated", () => {
        void loadConfiguration();
      }).then((fn) => {
        unlisten = fn;
      });
    }

    const serviceInterval = window.setInterval(() => {
      if (currentTab === "services") {
        void updatePhpStatus();
      }
    }, 4000);

    return () => {
      window.removeEventListener("focus", handleFocus);
      window.clearInterval(serviceInterval);
      if (unlisten) unlisten();
    };
  });

  async function loadConfiguration() {
    if (!isNativeApp) {
      profiles = structuredClone(starterProfiles);
      activeProfileId = profiles[0]?.id ?? null;
      selectedProfileId = activeProfileId;
      isLoading = false;
      return;
    }

    try {
      const configuration = await invoke<SecretsConfiguration>(
        "load_secret_profiles",
      );
      profiles =
        configuration.profiles.length > 0
          ? configuration.profiles
          : structuredClone(starterProfiles);
      activeProfileId =
        configuration.activeProfileId ?? profiles[0]?.id ?? null;
      if (
        selectedProfileId === null ||
        !profiles.some((p) => p.id === selectedProfileId)
      ) {
        selectedProfileId = activeProfileId ?? profiles[0]?.id ?? null;
      }
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isLoading = false;
    }
  }

  async function loadServicesState() {
    if (!isNativeApp) {
      phpState = { installedVersions: ["8.3.17", "8.2.27"], selectedVersion: "8.3.17" };
      nodeState = { installedVersions: ["22.14.0", "20.18.3"], selectedVersion: "22.14.0" };
      apacheState = { installedVersions: ["2.4.62"], selectedVersion: "2.4.62" };
      isPhpRunning = false;
      return;
    }

    try {
      const [phpInstalled, nodeInstalled, apacheInstalled, running] = await Promise.all([
        invoke<string[]>("get_installed_versions", { service: "PHP" }),
        invoke<string[]>("get_installed_versions", { service: "Node.js" }),
        invoke<string[]>("get_installed_versions", { service: "Apache" }),
        invoke<boolean>("get_php_status"),
      ]);

      phpState = {
        installedVersions: phpInstalled,
        selectedVersion: phpState.selectedVersion && phpInstalled.includes(phpState.selectedVersion)
          ? phpState.selectedVersion
          : (phpInstalled[0] ?? ""),
      };
      nodeState = {
        installedVersions: nodeInstalled,
        selectedVersion: nodeState.selectedVersion && nodeInstalled.includes(nodeState.selectedVersion)
          ? nodeState.selectedVersion
          : (nodeInstalled[0] ?? ""),
      };
      apacheState = {
        installedVersions: apacheInstalled,
        selectedVersion: apacheState.selectedVersion && apacheInstalled.includes(apacheState.selectedVersion)
          ? apacheState.selectedVersion
          : (apacheInstalled[0] ?? ""),
      };
      isPhpRunning = running;
    } catch (error) {
      // Non-blocking for services catalog
    }
  }

  async function updatePhpStatus() {
    if (!isNativeApp) return;
    try {
      isPhpRunning = await invoke<boolean>("get_php_status");
    } catch {
      isPhpRunning = false;
    }
  }

  function handleProfileSelect(val: string) {
    const num = Number(val);
    if (!isNaN(num)) {
      selectedProfileId = num;
    }
  }

  async function saveConfiguration(
    updatedProfiles: Profile[],
    updatedActiveId: number | null,
  ) {
    profiles = updatedProfiles;
    activeProfileId = updatedActiveId;
    if (!isNativeApp) return;

    isSaving = true;
    errorMessage = "";
    try {
      await invoke("save_secret_profiles", {
        configuration: {
          profiles: updatedProfiles,
          activeProfileId: updatedActiveId,
        },
      });
      await emit("secrets-updated");
      flashStatus("Cambios guardados");
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isSaving = false;
    }
  }

  function handleActivationRequest() {
    if (!selectedProfile) return;
    if (
      selectedProfile.isProduction &&
      selectedProfile.id !== activeProfileId
    ) {
      isProductionDialogOpen = true;
      return;
    }
    void executeActivation(selectedProfile.id);
  }

  async function executeActivation(profileId: number) {
    isActivating = true;
    errorMessage = "";
    isProductionDialogOpen = false;
    try {
      await saveConfiguration(profiles, profileId);
      if (isNativeApp) {
        await invoke("activate_secret_profile_for_powershell", { profileId });
        await emit("secrets-updated");
      }
      flashStatus("Perfil activado en el sistema");
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isActivating = false;
    }
  }

  function flashStatus(msg: string) {
    statusMessage = msg;
    window.setTimeout(() => {
      if (statusMessage === msg) statusMessage = "";
    }, 2500);
  }

  function toggleReveal(id: number) {
    revealedSecretIds = revealedSecretIds.includes(id)
      ? revealedSecretIds.filter((item) => item !== id)
      : [...revealedSecretIds, id];
  }

  async function copyText(text: string, identifier: string) {
    try {
      await navigator.clipboard.writeText(text);
      copiedId = identifier;
      window.setTimeout(() => {
        if (copiedId === identifier) copiedId = null;
      }, 1500);
    } catch {
      // ignore
    }
  }

  function openAddVariableDialog() {
    formKey = "";
    formValue = "";
    formError = "";
    editingSecretId = null;
    isVariableDialogOpen = true;
  }

  function openEditVariableDialog(secret: Secret) {
    formKey = secret.key;
    formValue = secret.value;
    formError = "";
    editingSecretId = secret.id;
    isVariableDialogOpen = true;
  }

  async function saveVariable() {
    const key = formKey.trim();
    if (!key) {
      formError = "El nombre de la variable es obligatorio";
      return;
    }
    if (!/^[A-Za-z0-9_]+$/.test(key)) {
      formError = "Solo letras, números y guiones bajos";
      return;
    }
    if (!selectedProfile) return;

    const duplicate = selectedProfile.secrets.find(
      (s) =>
        s.key.toUpperCase() === key.toUpperCase() && s.id !== editingSecretId,
    );
    if (duplicate) {
      formError = `La variable ${key} ya existe en este perfil`;
      return;
    }

    let updatedSecrets: Secret[];
    if (editingSecretId !== null) {
      updatedSecrets = selectedProfile.secrets.map((s) =>
        s.id === editingSecretId ? { ...s, key, value: formValue } : s,
      );
    } else {
      const nextId =
        Math.max(0, ...profiles.flatMap((p) => p.secrets.map((s) => s.id))) + 1;
      updatedSecrets = [
        ...selectedProfile.secrets,
        { id: nextId, key, value: formValue },
      ];
    }

    const updatedProfiles = profiles.map((p) =>
      p.id === selectedProfileId ? { ...p, secrets: updatedSecrets } : p,
    );

    isVariableDialogOpen = false;
    await saveConfiguration(updatedProfiles, activeProfileId);

    if (selectedProfile.id === activeProfileId && isNativeApp) {
      try {
        await invoke("activate_secret_profile_for_powershell", {
          profileId: activeProfileId,
        });
      } catch {
        // non-blocking
      }
    }
  }

  function requestDeleteVariable(secretId: number) {
    pendingDeleteSecretId = secretId;
    isDeleteVariableDialogOpen = true;
  }

  async function confirmDeleteVariable() {
    if (pendingDeleteSecretId === null || !selectedProfile) return;
    const secretId = pendingDeleteSecretId;
    pendingDeleteSecretId = null;
    isDeleteVariableDialogOpen = false;

    const updatedSecrets = selectedProfile.secrets.filter(
      (s) => s.id !== secretId,
    );
    const updatedProfiles = profiles.map((p) =>
      p.id === selectedProfileId ? { ...p, secrets: updatedSecrets } : p,
    );
    await saveConfiguration(updatedProfiles, activeProfileId);

    if (selectedProfile.id === activeProfileId && isNativeApp) {
      try {
        await invoke("activate_secret_profile_for_powershell", {
          profileId: activeProfileId,
        });
      } catch {
        // non-blocking
      }
    }
  }

  function openCreateProfileDialog() {
    newProfileName = "";
    newProfileIsProduction = false;
    isProfileDialogOpen = true;
  }

  async function saveNewProfile() {
    const name = newProfileName.trim();
    if (!name) return;
    const nextId = Math.max(0, ...profiles.map((p) => p.id)) + 1;
    const newProfile: Profile = {
      id: nextId,
      name,
      isProduction: newProfileIsProduction,
      secrets: [],
    };
    const updatedProfiles = [...profiles, newProfile];
    selectedProfileId = nextId;
    isProfileDialogOpen = false;
    await saveConfiguration(updatedProfiles, activeProfileId);
  }

  // Service operations
  async function handlePhpVersionChange(version: string) {
    phpState.selectedVersion = version;
    if (!isNativeApp || !version) return;
    isServiceBusy = true;
    try {
      await invoke("configure_php_cli_alias", { version });
      if (isPhpRunning) {
        await invoke("start_php", { version });
      }
      flashStatus(`PHP CLI configurado a v${version}`);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isServiceBusy = false;
    }
  }

  async function togglePhpService() {
    if (!isNativeApp) {
      isPhpRunning = !isPhpRunning;
      return;
    }
    if (!phpState.selectedVersion) return;

    isServiceBusy = true;
    errorMessage = "";
    try {
      if (isPhpRunning) {
        await invoke("stop_php");
        isPhpRunning = false;
        flashStatus("PHP FastCGI detenido");
      } else {
        await invoke("start_php", { version: phpState.selectedVersion });
        isPhpRunning = true;
        flashStatus(`PHP FastCGI iniciado (v${phpState.selectedVersion})`);
      }
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isServiceBusy = false;
    }
  }

  async function handleNodeVersionChange(version: string) {
    nodeState.selectedVersion = version;
    if (!isNativeApp || !version) return;
    isServiceBusy = true;
    try {
      await invoke("set_active_node_version", { version });
      flashStatus(`Node.js activo: v${version}`);
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : String(error);
    } finally {
      isServiceBusy = false;
    }
  }

  async function openMainWindow() {
    if (isNativeApp) {
      await invoke("show_main_window");
    }
  }

  async function closeQuickTray() {
    if (isNativeApp) {
      await invoke("hide_quick_tray");
    }
  }
</script>

<div class="tray-container">
  <!-- Top Navigation Header -->
  <header class="tray-header" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <span class="brand-badge" aria-hidden="true">
        <Anchor size={14} strokeWidth={2.4} />
      </span>
      <div class="header-tabs">
        <button
          type="button"
          class={`tab-btn${currentTab === "secrets" ? " active" : ""}`}
          onclick={() => (currentTab = "secrets")}
        >
          <KeyRound size={13} strokeWidth={2.2} />
          <span>Secrets</span>
        </button>
        <button
          type="button"
          class={`tab-btn${currentTab === "services" ? " active" : ""}`}
          onclick={() => (currentTab = "services")}
        >
          <Boxes size={13} strokeWidth={2.2} />
          <span>Services</span>
        </button>
      </div>
    </div>

    <div class="header-actions" data-tauri-drag-region="false">
      <Tooltip.Root>
        <Tooltip.Trigger
          class="header-action-btn"
          onclick={openMainWindow}
          aria-label="Abrir aplicación completa"
        >
          <Maximize2 size={13} strokeWidth={2.2} />
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content class="tooltip-content" sideOffset={6}
            >Abrir Harbor</Tooltip.Content
          >
        </Tooltip.Portal>
      </Tooltip.Root>

      <Tooltip.Root>
        <Tooltip.Trigger
          class="header-action-btn"
          onclick={closeQuickTray}
          aria-label="Cerrar panel"
        >
          <X size={14} strokeWidth={2.2} />
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content class="tooltip-content" sideOffset={6}
            >Ocultar</Tooltip.Content
          >
        </Tooltip.Portal>
      </Tooltip.Root>
    </div>
  </header>

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <span>Cargando Harbor...</span>
    </div>
  {:else}
    <!-- Main Content Area -->
    <div class="tray-body">
      <!-- Feedback notifications -->
      {#if statusMessage}
        <div class="feedback-banner success">{statusMessage}</div>
      {/if}
      {#if errorMessage}
        <div class="feedback-banner error">{errorMessage}</div>
      {/if}

      {#if currentTab === "secrets"}
        <!-- Profile Selector & Activation Toolbar -->
        <section class="profile-toolbar">
          <div class="profile-select-group">
            <Select.Root
              type="single"
              bind:value={selectedProfileIdStr}
              onValueChange={handleProfileSelect}
            >
              <Select.Trigger
                class="profile-select-trigger"
                aria-label="Seleccionar perfil"
              >
                <div class="select-label-wrapper">
                  {#if isCurrentProfileActive}
                    <span class="active-dot" title="Perfil activo en el sistema"
                    ></span>
                  {/if}
                  <span class="select-profile-name"
                    >{selectedProfile?.name || "Seleccionar perfil"}</span
                  >
                  {#if selectedProfile?.isProduction}
                    <span class="prod-tag">Prod</span>
                  {/if}
                </div>
                <ChevronDown size={14} strokeWidth={2.2} class="select-chevron" />
              </Select.Trigger>

              <Select.Portal>
                <Select.Content
                  class="profile-select-content"
                  sideOffset={5}
                  align="start"
                >
                  <Select.Viewport class="profile-select-viewport">
                    {#each profiles as profile (profile.id)}
                      {@const isProfileActive = profile.id === activeProfileId}
                      <Select.Item
                        class="profile-select-item"
                        value={String(profile.id)}
                        label={profile.name}
                      >
                        {#snippet children({ selected })}
                          <div class="item-left">
                            {#if isProfileActive}
                              <span class="active-dot"></span>
                            {:else}
                              <span class="dot-placeholder"></span>
                            {/if}
                            <span class="item-name">{profile.name}</span>
                            {#if profile.isProduction}
                              <span class="prod-tag">Prod</span>
                            {/if}
                          </div>
                          {#if selected}
                            <Check
                              size={14}
                              strokeWidth={2.4}
                              class="item-check"
                            />
                          {/if}
                        {/snippet}
                      </Select.Item>
                    {/each}
                  </Select.Viewport>
                </Select.Content>
              </Select.Portal>
            </Select.Root>

            <Button.Root
              class="secondary-button new-profile-btn"
              type="button"
              onclick={openCreateProfileDialog}
              title="Crear nuevo perfil"
            >
              <Plus size={14} strokeWidth={2.2} />
              <span>Nuevo</span>
            </Button.Root>
          </div>

          <!-- Activation status and action button -->
          <div class="activation-row">
            {#if isCurrentProfileActive}
              <div class="active-status-tag">
                <span class="active-pulse"></span>
                <span>Perfil activo en el sistema</span>
              </div>
            {:else}
              <div class="inactive-status-row">
                <span class="inactive-label">Este perfil no está activo</span>
                <Button.Root
                  class="activate-action-btn"
                  type="button"
                  disabled={isActivating}
                  onclick={handleActivationRequest}
                >
                  <Play size={11} strokeWidth={2.2} />
                  <span>{isActivating ? "Activando..." : "Activar perfil"}</span>
                </Button.Root>
              </div>
            {/if}
          </div>
        </section>

        <!-- Search & Add Variable Toolbar -->
        <section class="search-section">
          <div class="search-input-wrapper">
            <Search size={14} strokeWidth={2} class="search-icon" />
            <input
              type="text"
              placeholder="Buscar variable..."
              bind:value={searchQuery}
            />
            {#if searchQuery}
              <button
                class="clear-search-btn"
                type="button"
                onclick={() => (searchQuery = "")}
                aria-label="Limpiar búsqueda"
              >
                <X size={12} strokeWidth={2.4} />
              </button>
            {/if}
          </div>
          <Button.Root
            class="add-var-btn"
            type="button"
            onclick={openAddVariableDialog}
            title="Agregar variable"
          >
            <Plus size={13} strokeWidth={2.2} />
            <span>Variable</span>
          </Button.Root>
        </section>

        <!-- Variables List -->
        <div class="secrets-container">
          {#if !selectedProfile || selectedProfile.secrets.length === 0}
            <div class="empty-state">
              <div class="empty-icon"><KeyRound size={22} strokeWidth={1.8} /></div>
              <p class="empty-title">Sin variables de entorno</p>
              <p class="empty-subtitle">
                Agrega variables a este perfil para administrarlas en el sistema.
              </p>
              <Button.Root
                class="secondary-button"
                type="button"
                onclick={openAddVariableDialog}
              >
                <Plus size={14} strokeWidth={2.2} />
                <span>Agregar primera variable</span>
              </Button.Root>
            </div>
          {:else if filteredSecrets.length === 0}
            <div class="empty-state">
              <p class="empty-title">No hay resultados</p>
              <p class="empty-subtitle">
                No se encontraron variables coincidentes con "{searchQuery}".
              </p>
            </div>
          {:else}
            <ScrollArea.Root class="variables-scroll-area" type="auto">
              <ScrollArea.Viewport class="variables-viewport">
                {#each filteredSecrets as secret (secret.id)}
                  {@const isRevealed = revealedSecretIds.includes(secret.id)}
                  {@const isCopied = copiedId === `val-${secret.id}`}
                  {@const isKeyCopied = copiedId === `key-${secret.id}`}
                  <div class="secret-card">
                    <div class="secret-info">
                      <div class="secret-key-line">
                        <span class="secret-key-text">{secret.key}</span>
                        <button
                          class={`copy-key-btn${isKeyCopied ? " copied" : ""}`}
                          type="button"
                          onclick={() => copyText(secret.key, `key-${secret.id}`)}
                          title="Copiar nombre de variable"
                          aria-label="Copiar nombre de variable"
                        >
                          {#if isKeyCopied}
                            <Check size={11} strokeWidth={2.4} />
                          {:else}
                            <Copy size={11} strokeWidth={2} />
                          {/if}
                        </button>
                      </div>
                      <div class="secret-val-line">
                        {#if isRevealed}
                          <span class="secret-val-text"
                            >{secret.value || "<vacío>"}</span
                          >
                        {:else}
                          <span class="secret-val-masked">••••••••••••</span>
                        {/if}
                      </div>
                    </div>

                    <div class="secret-card-actions">
                      <Tooltip.Root>
                        <Tooltip.Trigger
                          class="action-icon-btn"
                          type="button"
                          aria-label={isRevealed ? "Ocultar valor" : "Ver valor"}
                          onclick={() => toggleReveal(secret.id)}
                        >
                          {#if isRevealed}
                            <EyeOff size={14} strokeWidth={2} />
                          {:else}
                            <Eye size={14} strokeWidth={2} />
                          {/if}
                        </Tooltip.Trigger>
                        <Tooltip.Portal>
                          <Tooltip.Content class="tooltip-content" sideOffset={6}>
                            {isRevealed ? "Ocultar" : "Mostrar"}
                          </Tooltip.Content>
                        </Tooltip.Portal>
                      </Tooltip.Root>

                      <Tooltip.Root>
                        <Tooltip.Trigger
                          class={`action-icon-btn${isCopied ? " copied" : ""}`}
                          type="button"
                          aria-label="Copiar valor"
                          onclick={() =>
                            copyText(secret.value, `val-${secret.id}`)}
                        >
                          {#if isCopied}
                            <Check size={14} strokeWidth={2.4} />
                          {:else}
                            <Copy size={14} strokeWidth={2} />
                          {/if}
                        </Tooltip.Trigger>
                        <Tooltip.Portal>
                          <Tooltip.Content class="tooltip-content" sideOffset={6}>
                            {isCopied ? "Copiado" : "Copiar valor"}
                          </Tooltip.Content>
                        </Tooltip.Portal>
                      </Tooltip.Root>

                      <DropdownMenu.Root>
                        <DropdownMenu.Trigger
                          class="action-icon-btn"
                          type="button"
                          aria-label="Opciones"
                        >
                          <Ellipsis size={15} strokeWidth={2} />
                        </DropdownMenu.Trigger>
                        <DropdownMenu.Portal>
                          <DropdownMenu.Content
                            class="dropdown-content"
                            sideOffset={4}
                            align="end"
                          >
                            <DropdownMenu.Item
                              class="dropdown-item"
                              onclick={() => openEditVariableDialog(secret)}
                            >
                              <Pencil size={14} strokeWidth={2} />
                              <span>Editar variable</span>
                            </DropdownMenu.Item>
                            <DropdownMenu.Separator class="dropdown-separator" />
                            <DropdownMenu.Item
                              class="dropdown-item destructive"
                              onclick={() => requestDeleteVariable(secret.id)}
                            >
                              <Trash2 size={14} strokeWidth={2} />
                              <span>Eliminar variable</span>
                            </DropdownMenu.Item>
                          </DropdownMenu.Content>
                        </DropdownMenu.Portal>
                      </DropdownMenu.Root>
                    </div>
                  </div>
                {/each}
              </ScrollArea.Viewport>
              <ScrollArea.Scrollbar
                class="variables-scrollbar"
                orientation="vertical"
              >
                <ScrollArea.Thumb class="variables-scrollbar-thumb" />
              </ScrollArea.Scrollbar>
            </ScrollArea.Root>
          {/if}
        </div>

        <!-- Secrets Footer -->
        <footer class="tray-footer">
          <div class="footer-status">
            <span class="footer-indicator"></span>
            <span
              >{selectedProfile ? selectedProfile.secrets.length : 0}
              {selectedProfile?.secrets.length === 1
                ? "variable"
                : "variables"}</span
            >
          </div>

          <Button.Root
            class="footer-open-btn"
            type="button"
            onclick={openMainWindow}
          >
            <span>Abrir Harbor completo</span>
            <Maximize2 size={11} strokeWidth={2.4} />
          </Button.Root>
        </footer>
      {:else}
        <!-- Services Tab Content -->
        <div class="services-tray-view">
          <div class="services-tray-list">
            <!-- PHP Service Card -->
            <div class="service-tray-card">
              <div class="service-tray-header">
                <div class="service-tray-title">
                  <span class="service-icon-badge php-badge">PHP</span>
                  <div>
                    <h3 class="service-name">PHP</h3>
                    <p class="service-subtitle">FastCGI & CLI</p>
                  </div>
                </div>

                <Button.Root
                  class={`service-toggle-btn${isPhpRunning ? " running" : ""}`}
                  type="button"
                  disabled={isServiceBusy || phpState.installedVersions.length === 0}
                  onclick={togglePhpService}
                  title={isPhpRunning ? "Detener FastCGI" : "Iniciar FastCGI"}
                >
                  {#if isPhpRunning}
                    <Square size={13} strokeWidth={2} />
                    <span>Detener</span>
                  {:else}
                    <Play size={13} strokeWidth={2} />
                    <span>Iniciar</span>
                  {/if}
                </Button.Root>
              </div>

              <div class="service-tray-body">
                <div class="service-field">
                  <span class="field-desc">Versión CLI / FastCGI:</span>
                  {#if phpState.installedVersions.length > 0}
                    <select
                      class="service-select"
                      value={phpState.selectedVersion}
                      disabled={isServiceBusy}
                      onchange={(e) => handlePhpVersionChange(e.currentTarget.value)}
                    >
                      {#each phpState.installedVersions as version}
                        <option value={version}>{version}</option>
                      {/each}
                    </select>
                  {:else}
                    <span class="not-installed-label">No instalado</span>
                  {/if}
                </div>

                <div class="service-status-row">
                  <span class={`status-pill ${isPhpRunning ? "running" : "stopped"}`}>
                    <span class="status-dot-indicator"></span>
                    <span>{isPhpRunning ? "FastCGI 127.0.0.1:9070" : "Detenido"}</span>
                  </span>
                </div>
              </div>
            </div>

            <!-- Node.js Service Card -->
            <div class="service-tray-card">
              <div class="service-tray-header">
                <div class="service-tray-title">
                  <span class="service-icon-badge node-badge">Node</span>
                  <div>
                    <h3 class="service-name">Node.js</h3>
                    <p class="service-subtitle">JavaScript Runtime</p>
                  </div>
                </div>
              </div>

              <div class="service-tray-body">
                <div class="service-field">
                  <span class="field-desc">Versión activa:</span>
                  {#if nodeState.installedVersions.length > 0}
                    <select
                      class="service-select"
                      value={nodeState.selectedVersion}
                      disabled={isServiceBusy}
                      onchange={(e) => handleNodeVersionChange(e.currentTarget.value)}
                    >
                      {#each nodeState.installedVersions as version}
                        <option value={version}>{version}</option>
                      {/each}
                    </select>
                  {:else}
                    <span class="not-installed-label">No instalado</span>
                  {/if}
                </div>

                <div class="service-status-row">
                  <span class="status-pill ready">
                    <span class="status-dot-indicator"></span>
                    <span>{nodeState.selectedVersion ? `v${nodeState.selectedVersion} listo` : "Sin versión"}</span>
                  </span>
                </div>
              </div>
            </div>

            <!-- Apache Service Card -->
            <div class="service-tray-card">
              <div class="service-tray-header">
                <div class="service-tray-title">
                  <span class="service-icon-badge apache-badge">HTTP</span>
                  <div>
                    <h3 class="service-name">Apache</h3>
                    <p class="service-subtitle">Servidor Web</p>
                  </div>
                </div>
              </div>

              <div class="service-tray-body">
                <div class="service-field">
                  <span class="field-desc">Versión instalada:</span>
                  {#if apacheState.installedVersions.length > 0}
                    <span class="service-static-ver">{apacheState.installedVersions[0]}</span>
                  {:else}
                    <span class="not-installed-label">No instalado</span>
                  {/if}
                </div>
              </div>
            </div>
          </div>

          <!-- Services Footer -->
          <footer class="tray-footer">
            <div class="footer-status">
              <span class="footer-indicator"></span>
              <span>{phpState.installedVersions.length + nodeState.installedVersions.length + apacheState.installedVersions.length} instalados</span>
            </div>

            <Button.Root
              class="footer-open-btn"
              type="button"
              onclick={openMainWindow}
            >
              <span>Descargar más en Harbor</span>
              <Maximize2 size={11} strokeWidth={2.4} />
            </Button.Root>
          </footer>
        </div>
      {/if}
    </div>
  {/if}
</div>

<!-- Bits-UI Dialog: Add / Edit Variable -->
<Dialog.Root bind:open={isVariableDialogOpen}>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content
      class="dialog-content"
      aria-describedby="variable-dialog-desc"
    >
      <div class="dialog-header">
        <Dialog.Title class="dialog-title">
          {editingSecretId !== null ? "Editar variable" : "Nueva variable"}
        </Dialog.Title>
        <Dialog.Description
          id="variable-dialog-desc"
          class="dialog-description"
        >
          Define la clave y el valor para el perfil seleccionado.
        </Dialog.Description>
      </div>

      {#if formError}
        <div class="form-error-box">{formError}</div>
      {/if}

      <div class="dialog-form-fields">
        <div class="field-group">
          <label class="field-label" for="dialog-var-key"
            >Nombre de Clave (KEY)</label
          >
          <input
            id="dialog-var-key"
            class="dialog-text-input font-mono"
            placeholder="EJ: DATABASE_URL, API_KEY"
            bind:value={formKey}
          />
        </div>

        <div class="field-group">
          <label class="field-label" for="dialog-var-value">Valor (VALUE)</label
          >
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
        <Button.Root
          class="primary-button btn-sm"
          type="button"
          onclick={saveVariable}
        >
          <Check size={14} strokeWidth={2.2} />
          <span>Guardar variable</span>
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<!-- Bits-UI Dialog: Create Profile -->
<Dialog.Root bind:open={isProfileDialogOpen}>
  <Dialog.Portal>
    <Dialog.Overlay class="modal-backdrop" />
    <Dialog.Content
      class="dialog-content"
      aria-describedby="profile-dialog-desc"
    >
      <div class="dialog-header">
        <Dialog.Title class="dialog-title">Nuevo perfil de entorno</Dialog.Title
        >
        <Dialog.Description id="profile-dialog-desc" class="dialog-description">
          Crea un entorno aislado para gestionar sus variables.
        </Dialog.Description>
      </div>

      <div class="dialog-form-fields">
        <div class="field-group">
          <label class="field-label" for="dialog-prof-name"
            >Nombre del perfil</label
          >
          <input
            id="dialog-prof-name"
            class="dialog-text-input"
            placeholder="EJ: Staging, Local, QA"
            bind:value={newProfileName}
          />
        </div>

        <div class="production-switch-row">
          <div class="switch-info">
            <span class="switch-title">Entorno de Producción</span>
            <span class="switch-desc"
              >Solicitará confirmación antes de activarse</span
            >
          </div>
          <Switch.Root
            class="production-switch"
            bind:checked={newProfileIsProduction}
          >
            <Switch.Thumb class="production-switch-thumb" />
          </Switch.Root>
        </div>
      </div>

      <div class="dialog-footer">
        <Dialog.Close class="secondary-button btn-sm">Cancelar</Dialog.Close>
        <Button.Root
          class="primary-button btn-sm"
          type="button"
          onclick={saveNewProfile}
        >
          <Check size={14} strokeWidth={2.2} />
          <span>Crear perfil</span>
        </Button.Root>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<!-- Bits-UI AlertDialog: Production Warning -->
<AlertDialog.Root bind:open={isProductionDialogOpen}>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="dialog-content confirmation-dialog">
      <div class="warning-icon-wrapper" aria-hidden="true">
        <ShieldAlert size={22} strokeWidth={2.2} />
      </div>
      <AlertDialog.Title class="dialog-title"
        >¿Activar perfil de producción?</AlertDialog.Title
      >
      <AlertDialog.Description class="dialog-description">
        Esto cargará las variables de producción en el sistema y en PowerShell.
        Confirma solo si es intencional.
      </AlertDialog.Description>
      <div class="dialog-footer">
        <AlertDialog.Cancel class="secondary-button btn-sm"
          >Cancelar</AlertDialog.Cancel
        >
        <AlertDialog.Action
          class="primary-button btn-sm warning-action-btn"
          onclick={() =>
            selectedProfile && executeActivation(selectedProfile.id)}
        >
          Activar producción
        </AlertDialog.Action>
      </div>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>

<!-- Bits-UI AlertDialog: Delete Variable -->
<AlertDialog.Root bind:open={isDeleteVariableDialogOpen}>
  <AlertDialog.Portal>
    <AlertDialog.Overlay class="modal-backdrop" />
    <AlertDialog.Content class="dialog-content confirmation-dialog">
      <div class="danger-icon-wrapper" aria-hidden="true">
        <AlertTriangle size={22} strokeWidth={2.2} />
      </div>
      <AlertDialog.Title class="dialog-title"
        >Eliminar variable</AlertDialog.Title
      >
      <AlertDialog.Description class="dialog-description">
        ¿Deseas eliminar permanentemente esta variable del perfil?
      </AlertDialog.Description>
      <div class="dialog-footer">
        <AlertDialog.Cancel class="secondary-button btn-sm"
          >Cancelar</AlertDialog.Cancel
        >
        <AlertDialog.Action
          class="danger-button btn-sm"
          onclick={confirmDeleteVariable}
        >
          Eliminar
        </AlertDialog.Action>
      </div>
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>

<style>
  /* Base Container */
  .tray-container {
    background: var(--color-boulder-50);
    color: var(--color-boulder-950);
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    user-select: none;
    box-sizing: border-box;
    font-family:
      "Inter",
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      Roboto,
      sans-serif;
    font-size: 13px;
  }

  .tray-container :global(button) {
    appearance: none;
    -webkit-appearance: none;
    background: transparent;
    border: none;
    outline: none;
    font-family: inherit;
    cursor: pointer;
  }

  /* Header */
  .tray-header {
    align-items: center;
    background: #ffffff;
    border-bottom: 1px solid var(--color-boulder-200);
    display: flex;
    height: 48px;
    justify-content: space-between;
    padding: 0 14px;
    flex-shrink: 0;
    cursor: default;
  }

  .brand {
    align-items: center;
    display: flex;
    gap: 10px;
  }

  .brand-badge {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200);
    border-radius: 7px;
    color: var(--color-east-bay-700);
    display: flex;
    height: 26px;
    justify-content: center;
    width: 26px;
  }

  .header-tabs {
    background: var(--color-boulder-100);
    border-radius: 6px;
    display: flex;
    padding: 2px;
    gap: 2px;
  }

  .tab-btn {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-600);
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
    height: 24px;
    padding: 0 8px;
    transition: all 0.15s ease;
  }

  .tab-btn.active {
    background: #ffffff;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
    color: var(--color-east-bay-900);
  }

  .header-actions {
    align-items: center;
    display: flex;
    gap: 4px;
  }

  :global(.header-action-btn) {
    align-items: center;
    background: transparent;
    border: none !important;
    border-radius: 6px;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    height: 28px;
    justify-content: center;
    padding: 0;
    width: 28px;
    transition:
      background 0.15s ease,
      color 0.15s ease;
  }

  :global(.header-action-btn:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  /* Body */
  .tray-body {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    background: var(--color-boulder-50);
  }

  /* Profile Toolbar (Select & Activation) */
  .profile-toolbar {
    background: #ffffff;
    border-bottom: 1px solid var(--color-boulder-200);
    padding: 10px 14px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex-shrink: 0;
  }

  .profile-select-group {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
  }

  :global(.profile-select-trigger) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-300) !important;
    border-radius: 6px;
    color: var(--color-boulder-900);
    cursor: pointer;
    display: flex;
    flex: 1;
    height: 32px;
    justify-content: space-between;
    padding: 0 10px;
    transition: all 0.15s ease;
  }

  :global(.profile-select-trigger:hover) {
    background: var(--color-boulder-50);
    border-color: var(--color-boulder-400) !important;
  }

  .select-label-wrapper {
    align-items: center;
    display: flex;
    gap: 7px;
    overflow: hidden;
  }

  .select-profile-name {
    font-size: 12.5px;
    font-weight: 650;
    color: var(--color-boulder-950);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.select-chevron) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
  }

  .active-dot {
    background: #10b981;
    border-radius: 50%;
    box-shadow: 0 0 4px #10b981;
    height: 7px;
    width: 7px;
    flex-shrink: 0;
  }

  .dot-placeholder {
    height: 7px;
    width: 7px;
    flex-shrink: 0;
  }

  .prod-tag {
    background: #fff1f2;
    border: 1px solid #fecdd3;
    border-radius: 4px;
    color: #be123c;
    font-size: 9.5px;
    font-weight: 700;
    line-height: 1;
    padding: 2px 5px;
    flex-shrink: 0;
  }

  :global(.new-profile-btn) {
    font-size: 12px;
    font-weight: 600;
    height: 32px;
    padding: 0 10px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  /* Profile Select Dropdown Menu */
  :global(.profile-select-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-shadow: 0 12px 32px rgb(11 11 11 / 14%);
    min-width: 220px;
    overflow: hidden;
    padding: 4px;
    z-index: 100;
  }

  :global(.profile-select-viewport) {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  :global(.profile-select-item) {
    align-items: center;
    border-radius: 5px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: flex;
    font-size: 12.5px;
    font-weight: 550;
    justify-content: space-between;
    height: 32px;
    outline: none;
    padding: 0 10px;
    transition: background 0.1s ease;
  }

  :global(.profile-select-item[data-highlighted]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-900);
  }

  .item-left {
    align-items: center;
    display: flex;
    gap: 7px;
  }

  .item-name {
    font-weight: 600;
  }

  :global(.item-check) {
    color: var(--color-east-bay-700);
  }

  /* Activation Row */
  .activation-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }

  .active-status-tag {
    align-items: center;
    background: #ecfdf5;
    border: 1px solid #a7f3d0;
    border-radius: 6px;
    color: #047857;
    display: flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 6px;
    padding: 4px 10px;
    width: 100%;
  }

  .active-pulse {
    background: #10b981;
    border-radius: 50%;
    height: 6px;
    width: 6px;
  }

  .inactive-status-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
    width: 100%;
  }

  .inactive-label {
    color: var(--color-boulder-500);
    font-size: 12px;
  }

  :global(.activate-action-btn) {
    align-items: center;
    background: var(--color-east-bay-700);
    border: none !important;
    border-radius: 5px;
    color: #ffffff;
    display: inline-flex;
    gap: 5px;
    font-size: 11.5px;
    font-weight: 650;
    height: 26px;
    padding: 0 10px;
    transition: background 0.15s ease;
  }

  :global(.activate-action-btn:hover) {
    background: var(--color-east-bay-900);
  }

  /* Search & Action Toolbar */
  .search-section {
    align-items: center;
    background: #ffffff;
    border-bottom: 1px solid var(--color-boulder-200);
    box-sizing: border-box;
    display: flex;
    gap: 8px;
    padding: 8px 14px;
    flex-shrink: 0;
  }

  .search-input-wrapper {
    align-items: center;
    background: var(--color-boulder-100);
    border: 1px solid transparent;
    border-radius: 6px;
    box-sizing: border-box;
    display: flex;
    flex: 1;
    height: 30px;
    padding: 0 8px;
    transition: all 0.15s ease;
  }

  .search-input-wrapper:focus-within {
    background: #ffffff;
    border-color: var(--color-east-bay-400);
    box-shadow: 0 0 0 2px rgb(113 132 192 / 14%);
  }

  :global(.search-icon) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
    margin-right: 6px;
  }

  .search-input-wrapper input {
    background: transparent;
    border: none;
    color: var(--color-boulder-950);
    flex: 1;
    font-family: inherit;
    font-size: 12px;
    min-width: 0;
    outline: none;
    padding: 0;
  }

  .clear-search-btn {
    align-items: center;
    color: var(--color-boulder-400);
    display: inline-flex;
    height: 16px;
    justify-content: center;
    width: 16px;
  }

  :global(.add-var-btn) {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-200) !important;
    border-radius: 6px;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 4px;
    height: 30px;
    padding: 0 10px;
    white-space: nowrap;
  }

  :global(.add-var-btn:hover) {
    background: var(--color-east-bay-100);
  }

  /* Feedback */
  .feedback-banner {
    font-size: 11.5px;
    font-weight: 550;
    padding: 6px 14px;
    text-align: center;
  }

  .feedback-banner.success {
    background: #ecfdf5;
    color: #065f46;
  }

  .feedback-banner.error {
    background: #fef2f2;
    color: #991b1b;
  }

  /* Secrets List Area */
  .secrets-container {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-height: 0;
    overflow: hidden;
  }

  :global(.variables-scroll-area) {
    flex: 1;
    min-height: 0;
    height: 100%;
  }

  :global(.variables-viewport) {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 10px 14px;
    box-sizing: border-box;
  }

  .secret-card {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    display: flex;
    justify-content: space-between;
    padding: 8px 10px;
    transition: border-color 0.15s ease;
  }

  .secret-card:hover {
    border-color: var(--color-boulder-300);
  }

  .secret-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
    flex: 1;
  }

  .secret-key-line {
    align-items: center;
    display: flex;
    gap: 5px;
  }

  .secret-key-text {
    color: var(--color-boulder-950);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
    font-weight: 700;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .copy-key-btn {
    align-items: center;
    color: var(--color-boulder-400);
    display: inline-flex;
    height: 16px;
    justify-content: center;
    width: 16px;
    border-radius: 3px;
  }

  .copy-key-btn:hover {
    color: var(--color-boulder-700);
    background: var(--color-boulder-100);
  }

  .copy-key-btn.copied {
    color: #10b981;
  }

  .secret-val-line {
    min-width: 0;
  }

  .secret-val-text {
    color: var(--color-boulder-600);
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 11px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: block;
  }

  .secret-val-masked {
    color: var(--color-boulder-400);
    font-size: 10px;
    letter-spacing: 2px;
  }

  .secret-card-actions {
    align-items: center;
    display: flex;
    gap: 2px;
    margin-left: 8px;
    flex-shrink: 0;
  }

  :global(.action-icon-btn) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-400);
    display: inline-flex;
    height: 24px;
    justify-content: center;
    width: 24px;
    transition: all 0.1s ease;
  }

  :global(.action-icon-btn:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-800);
  }

  :global(.action-icon-btn.copied) {
    color: #10b981;
  }

  /* Services Tab Views */
  .services-tray-view {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
  }

  .services-tray-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 14px;
    flex: 1;
    overflow-y: auto;
  }

  .service-tray-card {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    padding: 12px 14px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .service-tray-header {
    align-items: center;
    display: flex;
    justify-content: space-between;
  }

  .service-tray-title {
    align-items: center;
    display: flex;
    gap: 10px;
  }

  .service-icon-badge {
    align-items: center;
    border-radius: 6px;
    display: inline-flex;
    font-size: 10px;
    font-weight: 800;
    height: 28px;
    justify-content: center;
    width: 32px;
  }

  .php-badge {
    background: #ede9fe;
    color: #6d28d9;
  }

  .node-badge {
    background: #ecfdf5;
    color: #047857;
  }

  .apache-badge {
    background: #fef3c7;
    color: #b45309;
  }

  .service-name {
    font-size: 13.5px;
    font-weight: 700;
    margin: 0;
    color: var(--color-boulder-950);
  }

  .service-subtitle {
    font-size: 11px;
    color: var(--color-boulder-500);
    margin: 0;
  }

  :global(.service-toggle-btn) {
    align-items: center;
    background: var(--color-east-bay-900);
    border-radius: 5px;
    color: #ffffff;
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
    height: 28px;
    padding: 0 10px;
  }

  :global(.service-toggle-btn.running) {
    background: #e11d48;
  }

  .service-tray-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-top: 4px;
    border-top: 1px solid var(--color-boulder-100);
  }

  .service-field {
    align-items: center;
    display: flex;
    justify-content: space-between;
    font-size: 12px;
  }

  .field-desc {
    color: var(--color-boulder-600);
  }

  .service-select {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-300);
    border-radius: 5px;
    color: var(--color-boulder-900);
    font-family: inherit;
    font-size: 12px;
    font-weight: 600;
    height: 26px;
    padding: 0 6px;
    outline: none;
  }

  .service-select:focus {
    border-color: var(--color-east-bay-500);
  }

  .service-static-ver {
    font-size: 12px;
    font-weight: 600;
    color: var(--color-boulder-800);
  }

  .not-installed-label {
    color: var(--color-boulder-400);
    font-size: 11.5px;
    font-style: italic;
  }

  .service-status-row {
    align-items: center;
    display: flex;
  }

  .status-pill {
    align-items: center;
    border-radius: 12px;
    display: inline-flex;
    font-size: 10.5px;
    font-weight: 600;
    gap: 5px;
    padding: 2px 8px;
  }

  .status-pill.running {
    background: #ecfdf5;
    color: #047857;
  }

  .status-pill.stopped {
    background: var(--color-boulder-100);
    color: var(--color-boulder-600);
  }

  .status-pill.ready {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-800);
  }

  .status-dot-indicator {
    border-radius: 50%;
    height: 6px;
    width: 6px;
    background: currentColor;
  }

  /* Empty state */
  .empty-state {
    align-items: center;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 36px 20px;
    text-align: center;
    margin: auto 0;
  }

  .empty-icon {
    align-items: center;
    background: var(--color-boulder-100);
    border-radius: 50%;
    color: var(--color-boulder-500);
    display: flex;
    height: 48px;
    justify-content: center;
    margin-bottom: 12px;
    width: 48px;
  }

  .empty-title {
    color: var(--color-boulder-950);
    font-size: 13.5px;
    font-weight: 650;
    margin: 0 0 4px;
  }

  .empty-subtitle {
    color: var(--color-boulder-500);
    font-size: 12px;
    line-height: 1.4;
    margin: 0 0 16px;
    max-width: 240px;
  }

  /* Footer */
  .tray-footer {
    align-items: center;
    background: #ffffff;
    border-top: 1px solid var(--color-boulder-200);
    display: flex;
    height: 38px;
    justify-content: space-between;
    padding: 0 14px;
    flex-shrink: 0;
  }

  .footer-status {
    align-items: center;
    color: var(--color-boulder-500);
    display: flex;
    font-size: 11.5px;
    gap: 6px;
  }

  .footer-indicator {
    background: var(--color-boulder-400);
    border-radius: 50%;
    height: 5px;
    width: 5px;
  }

  :global(.footer-open-btn) {
    align-items: center;
    color: var(--color-east-bay-700);
    display: inline-flex;
    font-size: 11.5px;
    font-weight: 600;
    gap: 5px;
  }

  :global(.footer-open-btn:hover) {
    color: var(--color-east-bay-950);
  }

  /* Buttons & Inputs */
  :global(.secondary-button) {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-300) !important;
    border-radius: 6px;
    color: var(--color-boulder-800);
    display: inline-flex;
    font-size: 12px;
    font-weight: 600;
    gap: 6px;
    height: 32px;
    justify-content: center;
    padding: 0 12px;
    transition: all 0.15s ease;
  }

  :global(.secondary-button:hover) {
    background: var(--color-boulder-100);
  }

  :global(.primary-button) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: 1px solid transparent !important;
    border-radius: 6px;
    color: #ffffff;
    display: inline-flex;
    font-size: 12px;
    font-weight: 600;
    gap: 6px;
    height: 32px;
    justify-content: center;
    padding: 0 12px;
  }

  :global(.primary-button:hover) {
    background: var(--color-east-bay-950);
  }

  :global(.danger-button) {
    align-items: center;
    background: #e11d48;
    border-radius: 6px;
    color: #ffffff;
    display: inline-flex;
    font-size: 12px;
    font-weight: 600;
    height: 32px;
    padding: 0 12px;
  }

  :global(.btn-sm) {
    height: 30px;
    font-size: 11.5px;
    padding: 0 10px;
  }

  /* Modals and Dialogs */
  :global(.modal-backdrop) {
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(2px);
    inset: 0;
    position: fixed;
    z-index: 200;
  }

  :global(.dialog-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 10px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    left: 50%;
    max-width: 340px;
    padding: 18px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 90%;
    z-index: 201;
  }

  :global(.dialog-title) {
    color: var(--color-boulder-950);
    font-size: 15px;
    font-weight: 700;
    margin: 0 0 4px;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 12px;
    margin: 0 0 14px;
  }

  .dialog-form-fields {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 16px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .field-label {
    color: var(--color-boulder-700);
    font-size: 11px;
    font-weight: 600;
  }

  .dialog-text-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-950);
    font-family: inherit;
    font-size: 12.5px;
    height: 32px;
    outline: none;
    padding: 0 10px;
    width: 100%;
  }

  .dialog-text-input:focus {
    border-color: var(--color-east-bay-500);
  }

  .dialog-text-input.font-mono {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
  }

  .form-error-box {
    background: #fef2f2;
    border: 1px solid #fecdd3;
    border-radius: 6px;
    color: #991b1b;
    font-size: 11.5px;
    margin-bottom: 12px;
    padding: 6px 10px;
  }

  .production-switch-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
    padding-top: 4px;
  }

  .switch-title {
    color: var(--color-boulder-900);
    display: block;
    font-size: 12px;
    font-weight: 600;
  }

  .switch-desc {
    color: var(--color-boulder-500);
    display: block;
    font-size: 11px;
  }

  :global(.production-switch) {
    background: var(--color-boulder-300);
    border-radius: 9999px;
    height: 18px;
    position: relative;
    width: 32px;
    transition: background 0.15s ease;
  }

  :global(.production-switch[data-state="checked"]) {
    background: var(--color-east-bay-800);
  }

  :global(.production-switch-thumb) {
    background: #ffffff;
    border-radius: 9999px;
    display: block;
    height: 14px;
    transform: translateX(2px);
    width: 14px;
    transition: transform 0.15s ease;
  }

  :global(.production-switch[data-state="checked"] .production-switch-thumb) {
    transform: translateX(16px);
  }

  .dialog-footer {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  /* Confirmation Dialog */
  .confirmation-dialog {
    text-align: center;
    align-items: center;
  }

  .warning-icon-wrapper {
    align-items: center;
    background: #fffbeb;
    border-radius: 50%;
    color: #d97706;
    display: flex;
    height: 44px;
    justify-content: center;
    margin-bottom: 10px;
    width: 44px;
  }

  .danger-icon-wrapper {
    align-items: center;
    background: #fef2f2;
    border-radius: 50%;
    color: #e11d48;
    display: flex;
    height: 44px;
    justify-content: center;
    margin-bottom: 10px;
    width: 44px;
  }

  /* Dropdown Menu */
  :global(.dropdown-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
    padding: 4px;
    z-index: 150;
  }

  :global(.dropdown-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: flex;
    font-size: 12px;
    gap: 8px;
    height: 28px;
    outline: none;
    padding: 0 8px;
  }

  :global(.dropdown-item[data-highlighted]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-900);
  }

  :global(.dropdown-item.destructive) {
    color: #e11d48;
  }

  :global(.dropdown-item.destructive[data-highlighted]) {
    background: #fff1f2;
    color: #be123c;
  }

  :global(.dropdown-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }

  /* Tooltip */
  :global(.tooltip-content) {
    background: var(--color-boulder-900);
    border-radius: 4px;
    color: #ffffff;
    font-size: 11px;
    padding: 3px 6px;
    z-index: 300;
  }

  /* Spinner */
  .loading-state {
    align-items: center;
    color: var(--color-boulder-500);
    display: flex;
    flex-direction: column;
    gap: 10px;
    justify-content: center;
    height: 100%;
  }

  .spinner {
    border: 2px solid var(--color-boulder-200);
    border-top-color: var(--color-east-bay-700);
    border-radius: 50%;
    height: 20px;
    width: 20px;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .search-input-wrapper {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    display: flex;
    flex: 1;
    gap: 6px;
    height: 34px;
    min-width: 0;
    padding: 0 10px;
    transition: all 0.15s ease;
  }

  .search-input-wrapper:focus-within {
    background: #ffffff;
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 14%);
  }

  .search-input-wrapper input {
    background: transparent;
    border: none;
    color: var(--color-boulder-900);
    font-family: inherit;
    font-size: 12.5px;
    outline: none;
    width: 100%;
  }

  .search-input-wrapper input::placeholder {
    color: var(--color-boulder-400);
  }

  :global(.search-icon) {
    color: var(--color-boulder-400);
    flex-shrink: 0;
  }

  .clear-search-btn {
    align-items: center;
    background: transparent;
    border: none !important;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: flex;
    padding: 2px;
  }

  .clear-search-btn:hover {
    color: var(--color-boulder-700);
  }

  :global(.add-variable-btn) {
    font-size: 12px;
    font-weight: 650;
    height: 34px;
    padding: 0 12px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  /* Feedback Banner */
  .feedback-banner {
    font-size: 11.5px;
    font-weight: 600;
    padding: 6px 16px;
    text-align: center;
    flex-shrink: 0;
  }

  .feedback-banner.success {
    background: #ecfdf5;
    border-bottom: 1px solid #a7f3d0;
    color: #047857;
  }

  .feedback-banner.error {
    background: #fff3f1;
    border-bottom: 1px solid #fecdd3;
    color: #a13f32;
  }

  /* Variables Container & Separated Cards */
  .secrets-container {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    padding: 12px 16px;
    box-sizing: border-box;
  }

  :global(.variables-scroll-area) {
    flex: 1;
    min-height: 0;
    width: 100%;
    gap: 4px;
  }

  :global(.variables-viewport) {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding-right: 4px;
    box-sizing: border-box;
  }

  :global(.variables-scrollbar) {
    display: flex;
    padding: 2px;
    width: 6px;
  }

  :global(.variables-scrollbar-thumb) {
    background: var(--color-boulder-300);
    border-radius: 999px;
  }

  .secret-card {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-shadow: 0 1px 3px rgb(0 0 0 / 3%);
    display: flex;
    justify-content: space-between;
    padding: 11px 14px;
    transition: all 0.15s ease;
    box-sizing: border-box;
    margin-bottom: 4px;
  }

  .secret-card:hover {
    border-color: var(--color-east-bay-300);
    box-shadow: 0 2px 8px rgb(0 0 0 / 5%);
  }

  .secret-info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
    flex: 1;
  }

  .secret-key-line {
    align-items: center;
    display: flex;
    gap: 6px;
  }

  .secret-key-text {
    color: var(--color-east-bay-950);
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 12.5px;
    font-weight: 650;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .copy-key-btn {
    align-items: center;
    background: transparent;
    border: none !important;
    border-radius: 4px;
    color: var(--color-boulder-400);
    cursor: pointer;
    display: inline-flex;
    padding: 2px;
    transition: color 0.15s ease;
  }

  .copy-key-btn:hover {
    color: var(--color-boulder-800);
  }

  .copy-key-btn.copied {
    color: #047857;
  }

  .secret-val-line {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .secret-val-text {
    color: var(--color-boulder-600);
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 11.5px;
  }

  .secret-val-masked {
    color: var(--color-boulder-400);
    font-size: 10px;
    letter-spacing: 2px;
  }

  .secret-card-actions {
    align-items: center;
    display: flex;
    gap: 2px;
    margin-left: 8px;
    flex-shrink: 0;
  }

  :global(.action-icon-btn) {
    align-items: center;
    background: transparent;
    border: none !important;
    border-radius: 6px;
    color: var(--color-boulder-500);
    cursor: pointer;
    display: inline-flex;
    height: 28px;
    justify-content: center;
    padding: 0;
    width: 28px;
    transition: all 0.15s ease;
  }

  :global(.action-icon-btn:hover) {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  :global(.action-icon-btn.copied) {
    color: #047857;
  }

  /* Empty State */
  .empty-state {
    align-items: center;
    display: flex;
    flex-direction: column;
    gap: 8px;
    justify-content: center;
    flex: 1;
    min-height: 180px;
    text-align: center;
  }

  .empty-icon {
    align-items: center;
    background: var(--color-east-bay-50);
    border-radius: 50%;
    color: var(--color-east-bay-600);
    display: flex;
    height: 44px;
    justify-content: center;
    width: 44px;
    margin-bottom: 4px;
  }

  .empty-title {
    color: var(--color-boulder-900);
    font-size: 13px;
    font-weight: 650;
    margin: 0;
  }

  .empty-subtitle {
    color: var(--color-boulder-500);
    font-size: 12px;
    margin: 0 0 8px;
    max-width: 240px;
  }

  /* Footer */
  .tray-footer {
    align-items: center;
    background: #ffffff;
    border-top: 1px solid var(--color-boulder-200);
    display: flex;
    height: 38px;
    justify-content: space-between;
    padding: 0 16px;
    flex-shrink: 0;
    font-size: 11.5px;
  }

  .footer-status {
    align-items: center;
    color: var(--color-boulder-500);
    display: flex;
    gap: 6px;
    font-weight: 500;
  }

  .footer-indicator {
    background: var(--color-boulder-400);
    border-radius: 50%;
    height: 5px;
    width: 5px;
  }

  :global(.footer-open-btn) {
    align-items: center;
    background: transparent;
    border: none !important;
    color: var(--color-east-bay-700);
    cursor: pointer;
    display: inline-flex;
    font-family: inherit;
    font-size: 11.5px;
    font-weight: 650;
    gap: 5px;
    padding: 4px 6px;
    border-radius: 5px;
    transition: all 0.15s ease;
  }

  :global(.footer-open-btn:hover) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-950);
  }

  /* Buttons & Inputs */
  :global(.primary-button) {
    align-items: center;
    background: var(--color-east-bay-900);
    border: none !important;
    border-radius: 6px;
    color: #ffffff;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-weight: 650;
    gap: 6px;
    justify-content: center;
    transition: background 0.15s ease;
  }

  :global(.primary-button:hover) {
    background: var(--color-east-bay-950);
  }

  :global(.primary-button.warning-action-btn) {
    background: #a13f32;
  }

  :global(.primary-button.warning-action-btn:hover) {
    background: #8b3226;
  }

  :global(.secondary-button) {
    align-items: center;
    background: var(--color-boulder-100);
    border: 1px solid var(--color-boulder-200) !important;
    border-radius: 6px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-weight: 650;
    gap: 6px;
    justify-content: center;
    transition:
      background 0.15s ease,
      color 0.15s ease;
  }

  :global(.secondary-button:hover) {
    background: var(--color-boulder-200);
    color: var(--color-boulder-950);
  }

  :global(.danger-button) {
    align-items: center;
    background: #fff3f1;
    border: 1px solid #fecdd3 !important;
    border-radius: 6px;
    color: #a13f32;
    cursor: pointer;
    display: inline-flex;
    font: inherit;
    font-weight: 650;
    gap: 6px;
    justify-content: center;
    transition: background 0.15s ease;
  }

  :global(.danger-button:hover) {
    background: #fee2e2;
  }

  :global(.btn-sm) {
    font-size: 12px;
    height: 32px;
    padding: 0 14px;
  }

  /* Modals & Dialogs */
  :global(.modal-backdrop) {
    align-items: center;
    background: rgb(11 11 11 / 35%);
    display: flex;
    inset: 0;
    justify-content: center;
    padding: 16px;
    position: fixed;
    z-index: 50;
  }

  :global(.dialog-content) {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 20px 60px rgb(11 11 11 / 20%);
    display: flex;
    flex-direction: column;
    left: 50%;
    max-width: 360px;
    padding: 20px;
    position: fixed;
    top: 50%;
    transform: translate(-50%, -50%);
    width: min(calc(100% - 32px), 360px);
    z-index: 51;
  }

  .dialog-header {
    margin-bottom: 14px;
  }

  :global(.dialog-title) {
    color: var(--color-boulder-950);
    font-size: 16px;
    font-weight: 650;
    margin: 0;
  }

  :global(.dialog-description) {
    color: var(--color-boulder-600);
    font-size: 12.5px;
    line-height: 1.4;
    margin: 4px 0 0;
  }

  .dialog-form-fields {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .field-label {
    color: var(--color-east-bay-700);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .dialog-text-input {
    background: #ffffff;
    border: 1px solid var(--color-boulder-300);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-900);
    font: inherit;
    font-size: 12.5px;
    height: 34px;
    min-width: 0;
    outline: none;
    padding: 0 10px;
    width: 100%;
    transition: all 0.15s ease;
  }

  .dialog-text-input:focus {
    border-color: var(--color-east-bay-500);
    box-shadow: 0 0 0 3px rgb(113 132 192 / 16%);
  }

  .font-mono {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  }

  .production-switch-row {
    align-items: center;
    border-top: 1px solid var(--color-boulder-200);
    display: flex;
    justify-content: space-between;
    margin-top: 4px;
    padding-top: 12px;
  }

  .switch-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .switch-title {
    color: var(--color-boulder-900);
    font-size: 12.5px;
    font-weight: 650;
  }

  .switch-desc {
    color: var(--color-boulder-500);
    font-size: 11px;
  }

  :global(.production-switch) {
    background: var(--color-boulder-300);
    border: none !important;
    border-radius: 999px;
    cursor: pointer;
    display: inline-flex;
    height: 22px;
    padding: 2px;
    width: 40px;
    transition: background 0.15s ease;
  }

  :global(.production-switch[data-state="checked"]) {
    background: var(--color-east-bay-700);
  }

  :global(.production-switch-thumb) {
    background: #ffffff;
    border-radius: 999px;
    display: block;
    height: 18px;
    width: 18px;
    transition: transform 0.15s ease;
  }

  :global(.production-switch[data-state="checked"] .production-switch-thumb) {
    transform: translateX(18px);
  }

  .dialog-footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 20px;
  }

  .form-error-box {
    background: #fff3f1;
    border: 1px solid #fecdd3;
    border-radius: 6px;
    color: #a13f32;
    font-size: 11.5px;
    margin-bottom: 12px;
    padding: 6px 10px;
  }

  .warning-icon-wrapper,
  .danger-icon-wrapper {
    align-items: center;
    border-radius: 6px;
    display: flex;
    height: 40px;
    justify-content: center;
    margin-bottom: 12px;
    width: 40px;
  }

  .warning-icon-wrapper {
    background: #fff1dc;
    color: #965d00;
  }

  .danger-icon-wrapper {
    background: #fff3f1;
    color: #a13f32;
  }

  /* Tooltip & Dropdowns */
  :global(.tooltip-content) {
    background: var(--color-boulder-900);
    border-radius: 4px;
    color: #ffffff;
    font-size: 11px;
    font-weight: 500;
    padding: 3px 7px;
    z-index: 100;
  }

  :global(.dropdown-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-shadow: 0 10px 30px rgb(11 11 11 / 14%);
    min-width: 150px;
    padding: 4px;
    z-index: 100;
  }

  :global(.dropdown-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-800);
    cursor: pointer;
    display: flex;
    font-size: 12px;
    font-weight: 500;
    gap: 8px;
    height: 30px;
    outline: none;
    padding: 0 8px;
  }

  :global(.dropdown-item:hover),
  :global(.dropdown-item[data-highlighted]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-900);
  }

  :global(.dropdown-item.destructive) {
    color: #a13f32;
  }

  :global(.dropdown-item.destructive:hover),
  :global(.dropdown-item.destructive[data-highlighted]) {
    background: #fff3f1;
    color: #a13f32;
  }

  :global(.dropdown-separator) {
    background: var(--color-boulder-200);
    height: 1px;
    margin: 4px 0;
  }

  /* Loading State */
  .loading-state {
    align-items: center;
    display: flex;
    flex-direction: column;
    gap: 10px;
    justify-content: center;
    flex: 1;
    color: var(--color-boulder-500);
  }

  .spinner {
    border: 2px solid var(--color-boulder-200);
    border-top: 2px solid var(--color-east-bay-600);
    border-radius: 50%;
    width: 22px;
    height: 22px;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
