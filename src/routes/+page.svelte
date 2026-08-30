<svelte:head>
  <title>Harbor | Coming soon</title>
  <meta name="description" content="Harbor local development environment manager" />
</svelte:head>

<main class="coming-soon-page" aria-labelledby="coming-soon-title">
  <section class="coming-soon-content">
    <p class="eyebrow">Harbor</p>
    <h1 id="coming-soon-title">Coming soon</h1>
    <p class="message">We are preparing a better local development experience.</p>
  </section>
</main>

<style>
  .coming-soon-page {
    align-items: center;
    background:
      linear-gradient(135deg, rgb(240 246 245 / 90%), rgb(255 255 255 / 95%)),
      repeating-linear-gradient(0deg, transparent 0 31px, rgb(36 87 79 / 5%) 31px 32px),
      repeating-linear-gradient(90deg, transparent 0 31px, rgb(36 87 79 / 5%) 31px 32px);
    box-sizing: border-box;
    display: flex;
    flex: 1;
    justify-content: center;
    padding: 32px;
  }

  .coming-soon-content {
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
    font-size: clamp(40px, 6vw, 72px);
    font-weight: 650;
    letter-spacing: 0;
    line-height: 1;
    margin: 0;
  }

  .message {
    color: var(--color-boulder-600);
    font-size: 17px;
    line-height: 1.55;
    margin: 20px 0 0;
  }

  @media (max-width: 640px) {
    .coming-soon-page {
      align-items: flex-start;
      padding: 72px 24px;
    }

    .coming-soon-content {
      padding-left: 20px;
    }
  }
</style>
