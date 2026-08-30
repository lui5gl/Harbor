<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  onMount(() => {
    void goto("/services", { replaceState: true });
  });
</script>

<svelte:head>
  <title>Harbor</title>
  <meta name="description" content="Harbor local development environment manager" />
</svelte:head>

