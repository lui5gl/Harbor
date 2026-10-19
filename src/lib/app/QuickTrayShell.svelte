<script lang="ts">
  let { children } = $props();
</script>

<div class="flex h-screen flex-col overflow-hidden bg-boulder-50 text-boulder-950">
  {@render children()}
</div>
