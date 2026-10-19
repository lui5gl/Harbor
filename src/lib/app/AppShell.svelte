<script lang="ts">
  import NavigationBar from "#lib/app/navigation/NavigationBar.svelte";
  import StatusBar from "#lib/app/status-bar/StatusBar.svelte";

  let { children } = $props();
</script>

<div class="flex h-dvh flex-col overflow-hidden bg-boulder-50 text-boulder-950">
  <NavigationBar />
  <div class="flex min-h-0 flex-1 flex-col overflow-y-auto">
    {@render children()}
  </div>
  <StatusBar />
</div>
