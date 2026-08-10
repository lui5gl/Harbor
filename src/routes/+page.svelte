<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ServiceCard from "$lib/features/services/ServiceCard.svelte";
  import { serviceDefinitions, type ServiceDefinition } from "$lib/features/services/services";

  let services = $state<ServiceDefinition[]>(serviceDefinitions);
  let catalogError = $state("");
  let selectedVersions = $state<Record<string, string>>({});

  $effect(() => {
    void initializeWorkspace();
    void loadServiceVersions();
  });

  async function initializeWorkspace() {
    try {
      await invoke<string>("initialize_harbor_workspace");
    } catch (error) {
      catalogError = error instanceof Error ? error.message : String(error);
    }
  }

  async function loadServiceVersions() {
    catalogError = "";
    const catalogRequests = [
      ["Node.js", "get_node_versions"],
      ["PHP", "get_php_versions"],
      ["Apache", "get_apache_versions"]
    ] as const;

    const catalogResults = await Promise.allSettled(catalogRequests.map(async ([serviceName, command]) => [
      serviceName,
      await getCatalog(command),
      await getInstalledVersions(serviceName)
    ] as const));

    const versionsByService = new Map<string, string[]>();
    const failures: string[] = [];
    for (const result of catalogResults) {
      if (result.status === "fulfilled") {
        versionsByService.set(result.value[0], result.value[1]);
        const installedVersions = result.value[2];
        if (result.value[0] === "PHP" && installedVersions[0]) {
          await configurePhpAlias(installedVersions[0]);
        }
        services = services.map((service) => service.name === result.value[0]
          ? { ...service, installedVersions }
          : service);
      } else {
        failures.push(result.reason instanceof Error ? result.reason.message : String(result.reason));
      }
    }

    services = services.map((service) => ({
      ...service,
      versions: versionsByService.get(service.name) ?? service.versions
    }));
    if (failures.length > 0) {
      catalogError = `Unable to load runtime versions: ${failures.join(" | ")}`;
    }
  }

    async function getCatalog(command: string): Promise<string[]> {
      return await invoke<string[]>(command);
    }

    async function getInstalledVersions(serviceName: string): Promise<string[]> {
      return await invoke<string[]>("get_installed_versions", { service: serviceName });
    }

    async function configurePhpAlias(version: string): Promise<void> {
      await invoke<string>("configure_php_cli_alias", { version: version.split(" ")[0] });
    }

    async function installVersion(serviceName: string, version: string): Promise<void> {
      if (serviceName !== "PHP") {
        return;
      }
      const installableVersion = version.split(" ")[0];
      await invoke<string>("install_php", { version: installableVersion });
      await configurePhpAlias(installableVersion);
      const installedVersions = await getInstalledVersions(serviceName);
      services = services.map((service) => service.name === serviceName
        ? { ...service, installedVersions }
        : service);
      selectedVersions = { ...selectedVersions, [serviceName]: version };
    }

    async function startService(serviceName: string, version: string): Promise<void> {
      if (serviceName !== "Apache") {
        throw new Error(`${serviceName} is a runtime and does not run as a web service`);
      }
      await invoke<string>("start_apache", { version });
    }

    async function stopService(serviceName: string): Promise<void> {
      if (serviceName !== "Apache") {
        throw new Error(`${serviceName} is a runtime and does not run as a web service`);
      }
      await invoke("stop_apache");
    }

    async function getServiceStatus(serviceName: string): Promise<boolean> {
      if (serviceName !== "Apache") {
        return false;
      }
      return await invoke<boolean>("get_apache_status");
    }

    async function selectVersion(serviceName: string, version: string): Promise<void> {
      selectedVersions = { ...selectedVersions, [serviceName]: version };
      if (serviceName === "PHP") {
        await configurePhpAlias(version);
      }
    }
</script>

<svelte:head>
  <meta name="description" content="Harbor local development environment manager" />
</svelte:head>

<main class="services-page" aria-labelledby="services-title">
  <section class="page-header">
    <div class="heading-group">
      <h1 id="services-title">Services</h1>
      <p>Manage your local development environments.</p>
    </div>

  </section>

  {#if catalogError}
    <p class="catalog-error" role="alert">{catalogError}</p>
  {/if}

  <section class="service-list" aria-label="Available services">
    {#each services as service (service.name)}
      <ServiceCard
        serviceName={service.name}
        serviceDescription={service.description}
        serviceIconPath={service.iconPath}
        versions={service.versions}
        installedVersions={service.installedVersions}
        onInstall={(version) => installVersion(service.name, version)}
        onStart={(version) => startService(service.name, version)}
        onStop={() => stopService(service.name)}
        getStatus={() => getServiceStatus(service.name)}
        onVersionSelect={(version) => selectVersion(service.name, version)}
      />
    {/each}
  </section>
</main>

<style>
  .services-page {
    box-sizing: border-box;
    min-height: 100%;
    padding: 24px;
  }

  .page-header {
    align-items: flex-start;
    display: flex;
    justify-content: space-between;
    gap: 24px;
    max-width: 1120px;
    margin: 0 auto;
    width: 100%;
  }

  .heading-group {
    min-width: 0;
  }

  .service-list {
    display: grid;
    gap: 12px;
    margin-top: 32px;
    max-width: 1120px;
    margin-left: auto;
    margin-right: auto;
    width: 100%;
  }

  .catalog-error {
    background: #fff4f2;
    border: 1px solid #f0b8ad;
    border-radius: 8px;
    color: #8f2d21;
    margin: 20px auto 0;
    max-width: 1120px;
    padding: 12px 16px;
  }

  h1 {
    color: var(--color-boulder-950);
    font-size: 24px;
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.2;
    margin: 0;
  }

  p {
    color: var(--color-east-bay-800);
    font-size: 16px;
    line-height: 1.5;
    margin: 8px 0 0;
  }

  @media (max-width: 560px) {
    .services-page {
      padding: 20px 16px;
    }

    .page-header {
      align-items: stretch;
      flex-direction: column;
    }

  }
</style>