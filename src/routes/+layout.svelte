<script lang="ts">
  import NavigationBar from "$lib/features/navigation/NavigationBar.svelte";

  let { children } = $props();
</script>

<svelte:head>
  <title>Harbor</title>
</svelte:head>

<div class="application-shell">
  <NavigationBar />
  {@render children()}
</div>

<style>
  :global(html),
  :global(body) {
    background: #f8fafc;
    color: #0f172a;
    font-family: "Segoe UI Variable", "Segoe UI", sans-serif;
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
    outline: 2px solid #6366f1;
    outline-offset: 2px;
  }

  .application-shell {
    background: #f8fafc;
    color: #0f172a;
    min-height: 100vh;
  }
</style>