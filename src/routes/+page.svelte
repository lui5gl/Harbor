<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ServiceCard from "$lib/features/services/ServiceCard.svelte";
  import { serviceDefinitions, type ServiceDefinition } from "$lib/features/services/services";

  let services = $state<ServiceDefinition[]>(serviceDefinitions);

  $effect(() => {
    void loadServiceVersions();
  });

  async function loadServiceVersions() {
    const catalogRequests = [
      ["Node.js", "get_node_versions"],
      ["PHP", "get_php_versions"],
      ["Apache", "get_apache_versions"]
    ] as const;

    const catalogResults = await Promise.allSettled(
      catalogRequests.map(async ([serviceName, command]) => [
        serviceName,
          await getCatalog(command)
      ] as const)
    );

    const versionsByService = new Map<string, string[]>();
    for (const result of catalogResults) {
      if (result.status === "fulfilled") {
        versionsByService.set(result.value[0], result.value[1]);
      }
    }

    services = services.map((service) => ({
      ...service,
      versions: versionsByService.get(service.name) ?? service.versions
    }));
  }

    async function getCatalog(command: string): Promise<string[]> {
      return await invoke<string[]>(command);
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

  <section class="service-list" aria-label="Available services">
    {#each services as service (service.name)}
      <ServiceCard
        serviceName={service.name}
        serviceDescription={service.description}
        serviceIconPath={service.iconPath}
        versions={service.versions}
        installedVersions={service.installedVersions}
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