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
    background: #ffffff;
    border-bottom: 1px solid #e2e8f0;
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
    color: #0f172a;
    gap: 0.625rem;
    text-decoration: none;
  }

  .brand-mark {
    align-items: center;
    background: #eef2ff;
    border: 1px solid #e0e7ff;
    border-radius: 8px;
    color: #4f46e5;
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
    color: #94a3b8;
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