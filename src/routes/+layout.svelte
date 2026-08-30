<script lang="ts">
  import "@fontsource/inter/400.css";
  import "@fontsource/inter/500.css";
  import "@fontsource/inter/600.css";
  import "@fontsource/inter/700.css";
  import { Tooltip } from "bits-ui";
  import { page } from "$app/state";
  import NavigationBar from "$lib/features/navigation/NavigationBar.svelte";
  import StatusBar from "$lib/features/status-bar/StatusBar.svelte";

  let { children } = $props();

  let isQuickTray = $derived(page.url.pathname.startsWith("/quick-tray"));
</script>

<svelte:head>
  <title>Harbor</title>
</svelte:head>

<Tooltip.Provider delayDuration={500}>
  {#if isQuickTray}
    <div class="quick-tray-shell">
      {@render children()}
    </div>
  {:else}
    <div class="application-shell">
      <NavigationBar />
      <div class="application-content">
        {@render children()}
      </div>
      <StatusBar />
    </div>
  {/if}
</Tooltip.Provider>

<style>
  :global(:root) {
    --color-east-bay-50: #f4f6fa;
    --color-east-bay-100: #e5e9f4;
    --color-east-bay-200: #d1d9ec;
    --color-east-bay-300: #b1c0df;
    --color-east-bay-400: #8ca0ce;
    --color-east-bay-500: #7184c0;
    --color-east-bay-600: #5e6db2;
    --color-east-bay-700: #535da2;
    --color-east-bay-800: #484e85;
    --color-east-bay-900: #424874;
    --color-east-bay-950: #282a43;
    --color-boulder-50: #fafafa;
    --color-boulder-100: #f5f5f5;
    --color-boulder-200: #e6e6e6;
    --color-boulder-300: #d3d3d3;
    --color-boulder-400: #a3a3a3;
    --color-boulder-500: #747474;
    --color-boulder-600: #535353;
    --color-boulder-700: #404040;
    --color-boulder-800: #272727;
    --color-boulder-900: #1a1a1a;
    --color-boulder-950: #0b0b0b;
  }

  :global(html),
  :global(body) {
    background: var(--color-boulder-50);
    color: var(--color-boulder-950);
    font-family: "Inter", sans-serif;
    font-size: 14px;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    margin: 0;
    min-height: 100%;
  }

  :global(button),
  :global(a) {
    -webkit-tap-highlight-color: transparent;
  }

  :global(:focus-visible) {
    outline: 2px solid var(--color-east-bay-400);
    outline-offset: 2px;
  }

  .application-shell {
    background: var(--color-boulder-50);
    color: var(--color-boulder-950);
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .application-content {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .quick-tray-shell {
    background: var(--color-boulder-50);
    color: var(--color-boulder-950);
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    box-sizing: border-box;
  }
</style>