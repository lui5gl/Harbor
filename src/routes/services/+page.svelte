<svelte:head>
  <title>Harbor | Services</title>
  <meta name="description" content="Harbor service management is coming soon." />
</svelte:head>

<main class="services-page" aria-labelledby="services-title">
  <section class="services-content">
    <p class="eyebrow">Harbor</p>
    <h1 id="services-title">Services</h1>
    <p>Runtime and local service management is being prepared.</p>
  </section>
</main>

<style>
  .services-page {
    align-items: center;
    box-sizing: border-box;
    display: flex;
    flex: 1;
    justify-content: center;
    padding: 32px;
  }

  .services-content {
    border-left: 3px solid var(--color-east-bay-500);
    max-width: 560px;
    padding: 12px 0 12px 28px;
  }

  .eyebrow {
    color: var(--color-east-bay-700);
    font-size: 13px;
    font-weight: 700;
    letter-spacing: 0;
    margin: 0 0 12px;
    text-transform: uppercase;
  }

  h1 {
    color: var(--color-boulder-950);
    font-size: 40px;
    font-weight: 650;
    line-height: 1;
    margin: 0;
  }

  p:last-child {
    color: var(--color-boulder-600);
    font-size: 16px;
    line-height: 1.55;
    margin: 20px 0 0;
  }
</style>
