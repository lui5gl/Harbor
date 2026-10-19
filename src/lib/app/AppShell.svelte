<script lang="ts">
  import NavigationBar from "#lib/app/navigation/NavigationBar.svelte";
  import StatusBar from "#lib/app/status-bar/StatusBar.svelte";

  let { children } = $props();
</script>

<div class="flex min-h-screen flex-col bg-boulder-50 text-boulder-950">
  <NavigationBar />
  <div class="flex flex-1 flex-col">
    {@render children()}
  </div>
  <StatusBar />
</div>
