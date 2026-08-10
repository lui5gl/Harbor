<script lang="ts">
  import { Anchor } from "@lucide/svelte";

  const applicationName = "Harbor";
</script>

<header class="navigation-bar" data-tauri-drag-region>
  <a class="brand" href="/" aria-label={`${applicationName}, inicio`}>
    <span class="brand-mark" aria-hidden="true">
      <Anchor size={16} strokeWidth={2} />
    </span>
    <span class="brand-copy">
      <span class="brand-name">{applicationName}</span>
      <span class="brand-subtitle">Desktop</span>
    </span>
  </a>

</header>

<style>
  .navigation-bar {
    align-items: center;
    background: var(--color-boulder-50);
    border-bottom: 1px solid var(--color-boulder-200);
    box-sizing: border-box;
    display: flex;
    flex-shrink: 0;
    height: 60px;
    justify-content: space-between;
    padding: 0 24px;
    user-select: none;
  }

  .brand {
    align-items: center;
    display: flex;
  }

  .brand {
    color: var(--color-boulder-950);
    gap: 0.625rem;
    text-decoration: none;
  }

  .brand-mark {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-100);
    border-radius: 8px;
    color: var(--color-east-bay-700);
    display: flex;
    height: 32px;
    justify-content: center;
    width: 32px;
  }

  .brand-copy {
    align-items: baseline;
    display: flex;
    gap: 7px;
  }

  .brand-name {
    font-size: 15px;
    font-weight: 650;
    letter-spacing: 0;
  }

  .brand-subtitle {
    color: var(--color-boulder-500);
    font-size: 11px;
    font-weight: 500;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  @media (max-width: 480px) {
    .navigation-bar {
      padding: 0 12px;
    }

    .brand-subtitle {
      display: none;
    }
  }
</style>