<script lang="ts">
  import ServiceCard from "$lib/features/services/ServiceCard.svelte";
  import { serviceDefinitions } from "$lib/features/services/services";
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
    {#each serviceDefinitions as service (service.name)}
      <ServiceCard
        serviceName={service.name}
        serviceDescription={service.description}
        serviceIconPath={service.iconPath}
        versions={service.versions}
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