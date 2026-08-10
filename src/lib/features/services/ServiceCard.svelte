<script lang="ts">
  import { ChevronDown, Download, Play } from "@lucide/svelte";
  import { Button } from "bits-ui";
  import { siNodedotjs } from "simple-icons";

  const serviceName = "Node.js";
  const serviceDescription = "Runtime environment";
  const selectedVersion = "v20.11.1 (LTS)";
</script>

<article class="service-card" aria-labelledby="node-service-title">
  <div class="service-identity">
    <div class="service-icon" aria-hidden="true">
      <svg viewBox="0 0 24 24" role="img">
        <path d={siNodedotjs.path} />
      </svg>
    </div>
    <div class="service-copy">
      <h2 id="node-service-title">{serviceName}</h2>
      <p>{serviceDescription}</p>
    </div>
  </div>

  <div class="service-controls">
    <Button.Root class="version-button" type="button" aria-haspopup="listbox">
      <span>{selectedVersion}</span>
      <ChevronDown size={16} strokeWidth={2} aria-hidden="true" />
    </Button.Root>

    <Button.Root class="download-button" type="button" aria-label={`Download ${serviceName} ${selectedVersion}`}>
      <Download size={17} strokeWidth={1.8} aria-hidden="true" />
    </Button.Root>

    <Button.Root class="start-button" type="button" aria-label={`Start ${serviceName}`}>
      <Play size={18} strokeWidth={1.8} aria-hidden="true" />
    </Button.Root>
  </div>
</article>

<style>
  .service-card {
    align-items: center;
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 12px;
    box-sizing: border-box;
    display: flex;
    gap: 32px;
    justify-content: space-between;
    min-height: 92px;
    padding: 18px 28px;
  }

  .service-identity {
    align-items: center;
    display: flex;
    flex: 1;
    gap: 20px;
    min-width: 0;
  }

  .service-icon {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-100);
    border-radius: 10px;
    color: var(--color-east-bay-500);
    display: flex;
    flex-shrink: 0;
    height: 48px;
    justify-content: center;
    width: 48px;
  }

  .service-icon svg {
    fill: currentColor;
    height: 25px;
    width: 25px;
  }

  .service-copy {
    min-width: 0;
  }

  h2 {
    color: var(--color-boulder-950);
    font-size: 17px;
    font-weight: 600;
    line-height: 1.25;
    margin: 0;
  }

  p {
    color: var(--color-east-bay-800);
    font-size: 14px;
    line-height: 1.4;
    margin: 4px 0 0;
  }

  .service-controls {
    align-items: center;
    display: flex;
    flex-shrink: 0;
    gap: 8px;
    justify-content: flex-end;
    width: 400px;
  }

  :global(.version-button),
  :global(.download-button),
  :global(.start-button) {
    align-items: center;
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    box-sizing: border-box;
    color: var(--color-east-bay-800);
    display: inline-flex;
    font: inherit;
    justify-content: center;
    min-height: 40px;
    transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
  }

  :global(.version-button:hover),
  :global(.download-button:hover),
  :global(.start-button:hover) {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-200);
    color: var(--color-east-bay-700);
  }

  :global(.version-button) {
    border-radius: 7px;
    gap: 44px;
    justify-content: space-between;
    width: 300px;
    padding: 0 14px;
  }

  :global(.download-button),
  :global(.start-button) {
    border-radius: 7px;
    width: 40px;
  }

  :global(.start-button) {
    margin-left: 0;
  }

  @media (max-width: 720px) {
    .service-card {
      align-items: stretch;
      flex-direction: column;
      padding: 18px;
    }

    .service-controls {
      width: 100%;
      min-width: 0;
    }

    :global(.version-button) {
      flex: 1;
      min-width: 0;
    }
  }
</style>
