<script lang="ts">
  import { Activity, Cpu } from "@lucide/svelte";

  const memoryRefreshInterval = 2000;
  const runningServices = 0;

  let memoryUsage = $state<string>("Unavailable");

  function updateMemoryUsage() {
    const performanceWithMemory = performance as Performance & {
      memory?: {
        usedJSHeapSize: number;
      };
    };

    const usedBytes = performanceWithMemory.memory?.usedJSHeapSize;

    if (usedBytes === undefined) {
      memoryUsage = "Unavailable";
      return;
    }

    memoryUsage = `${Math.round(usedBytes / 1024 / 1024)} MB`;
  }

  $effect(() => {
    updateMemoryUsage();
    const refreshHandle = window.setInterval(updateMemoryUsage, memoryRefreshInterval);

    return () => window.clearInterval(refreshHandle);
  });
</script>

<footer class="status-bar" aria-label="Application status">
  <div class="status-group">
    <span class="status-item">
      <Activity size={14} strokeWidth={2} aria-hidden="true" />
      <span>Services</span>
      <strong>{runningServices} running</strong>
    </span>
  </div>

  <div class="status-group">
    <span class="status-item">
      <Cpu size={14} strokeWidth={2} aria-hidden="true" />
      <span>Memory Usage</span>
      <strong>{memoryUsage}</strong>
    </span>
  </div>
</footer>

<style>
  .status-bar {
    align-items: center;
    background: var(--color-east-bay-950);
    box-sizing: border-box;
    color: var(--color-east-bay-50);
    display: flex;
    flex-shrink: 0;
    font-size: 12px;
    justify-content: space-between;
    min-height: 28px;
    padding: 0 12px;
    user-select: none;
  }

  .status-group {
    align-items: center;
    display: flex;
    min-width: 0;
  }

  .status-item {
    align-items: center;
    display: inline-flex;
    gap: 6px;
    min-height: 28px;
    white-space: nowrap;
  }

  .status-item :global(svg) {
    color: var(--color-east-bay-300);
  }

  .status-item strong {
    color: var(--color-boulder-50);
    font-weight: 600;
  }

  @media (max-width: 480px) {
    .status-bar {
      gap: 12px;
      overflow: hidden;
      padding: 0 8px;
    }

    .status-item {
      gap: 4px;
    }
  }
</style>
