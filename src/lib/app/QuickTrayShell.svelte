<script lang="ts">
  let { children } = $props();
</script>

<div class="quick-tray-shell">
  {@render children()}
</div>

<style>
  .quick-tray-shell {
    background: var(--color-boulder-50);
    box-sizing: border-box;
    color: var(--color-boulder-950);
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
</style>
