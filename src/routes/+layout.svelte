<script lang="ts">
  import "../app.css";
  import "@fontsource-variable/geist";
  import "@fontsource-variable/geist-mono";
  import { page } from "$app/state";
  import AppShell from "#lib/app/AppShell.svelte";
  import QuickTrayShell from "#lib/app/QuickTrayShell.svelte";

  let { children } = $props();

  let isQuickTray = $derived(page.url.pathname.startsWith("/quick-tray"));
</script>

<svelte:head>
  <title>Harbor</title>
</svelte:head>

{#if isQuickTray}
  <QuickTrayShell>
    {@render children()}
  </QuickTrayShell>
{:else}
  <AppShell>
    {@render children()}
  </AppShell>
{/if}
