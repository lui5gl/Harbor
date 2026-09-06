<script lang="ts">
  import NavigationBar from "$lib/app/navigation/NavigationBar.svelte";
  import StatusBar from "$lib/app/status-bar/StatusBar.svelte";

  let { children } = $props();
</script>

<div class="application-shell">
  <NavigationBar />
  <div class="application-content">
    {@render children()}
  </div>
  <StatusBar />
</div>

<style>
  .application-shell {
    background: var(--color-boulder-50);
    color: var(--color-boulder-950);
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .application-content {
    display: flex;
    flex: 1;
    flex-direction: column;
  }
</style>
