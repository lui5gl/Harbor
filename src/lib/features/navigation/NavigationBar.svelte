<script lang="ts">
  import { page } from "$app/state";
  import { Anchor, Boxes, KeyRound } from "@lucide/svelte";

  const applicationName = "Harbor";
  const navigationItems = [
    { href: "/services", label: "Services", icon: Boxes },
    { href: "/secrets", label: "Secrets", icon: KeyRound }
  ];
</script>

<header class="navigation-bar" data-tauri-drag-region>
  <a class="brand" href="/services" aria-label={`${applicationName}, inicio`}>
    <span class="brand-mark" aria-hidden="true">
      <Anchor size={16} strokeWidth={2} />
    </span>
    <span class="brand-copy">
      <span class="brand-name">{applicationName}</span>
      <span class="brand-subtitle">Desktop</span>
    </span>
  </a>

  <nav class="navigation-links" aria-label="Primary navigation">
    {#each navigationItems as item (item.href)}
      {@const isActive = page.url.pathname === item.href || page.url.pathname.startsWith(item.href + "/")}
      <a
        class={`navigation-link${isActive ? " active" : ""}`}
        href={item.href}
        data-tauri-drag-region="false"
        aria-current={isActive ? "page" : undefined}
      >
        <item.icon size={16} strokeWidth={1.9} aria-hidden="true" />
        <span>{item.label}</span>
      </a>
    {/each}
  </nav>
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

  .navigation-links {
    align-items: center;
    display: flex;
    gap: 4px;
  }

  .navigation-link {
    align-items: center;
    border-radius: 6px;
    color: var(--color-boulder-600);
    display: inline-flex;
    font-size: 13px;
    font-weight: 600;
    gap: 7px;
    min-height: 34px;
    padding: 0 10px;
    text-decoration: none;
    transition: background-color 150ms ease, color 150ms ease;
  }

  .navigation-link:hover {
    background: var(--color-boulder-100);
    color: var(--color-boulder-950);
  }

  .navigation-link.active {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-700);
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

    .navigation-link {
      padding: 0 7px;
    }

    .navigation-link span {
      display: none;
    }
  }
</style>