<script lang="ts">
  import { Plus } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import ServiceCard from "$lib/features/services/ServiceCard.svelte";
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

    <Button.Root class="add-service-button" type="button">
      <Plus size={18} strokeWidth={2} aria-hidden="true" />
      <span>Add Service</span>
    </Button.Root>
  </section>

  <section class="service-list" aria-label="Available services">
    <ServiceCard />
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

  :global(.add-service-button) {
    align-items: center;
    background: var(--color-east-bay-700);
    border: 1px solid var(--color-east-bay-700);
    border-radius: 10px;
    box-sizing: border-box;
    color: var(--color-boulder-50);
    display: inline-flex;
    flex-shrink: 0;
    font: inherit;
    font-size: 14px;
    font-weight: 600;
    gap: 10px;
    justify-content: center;
    min-height: 44px;
    padding: 0 18px;
    transition: background-color 150ms ease, border-color 150ms ease, transform 150ms ease;
  }

  :global(.add-service-button:hover) {
    background: var(--color-east-bay-800);
    border-color: var(--color-east-bay-800);
  }

  :global(.add-service-button:active) {
    transform: translateY(1px);
  }

  @media (max-width: 560px) {
    .services-page {
      padding: 20px 16px;
    }

    .page-header {
      align-items: stretch;
      flex-direction: column;
    }

    :global(.add-service-button) {
      align-self: flex-start;
    }
  }
</style>