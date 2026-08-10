<script lang="ts">
  import { ChevronDown, Check, Download, Play, Search } from "@lucide/svelte";
  import { Button, Combobox } from "bits-ui";
  import { siNodedotjs } from "simple-icons";

  const serviceName = "Node.js";
  const serviceDescription = "Runtime environment";
  const versions = [
    "v24.4.1 (Current)",
    "v24.3.0 (Current)",
    "v22.17.1 (LTS)",
    "v22.16.0 (LTS)",
    "v22.15.1 (LTS)",
    "v20.19.4 (LTS)",
    "v20.19.3 (LTS)",
    "v20.18.3 (LTS)",
    "v18.20.8 (LTS)",
    "v18.20.7 (LTS)"
  ];
  let selectedVersion = versions[2];
  let searchValue = "";
  $: filteredVersions = searchValue === ""
    ? versions
    : versions.filter((version) => version.toLowerCase().includes(searchValue.toLowerCase()));
  $: installedVersions = filteredVersions.filter((version) => version === selectedVersion);
  $: downloadableVersions = filteredVersions.filter((version) => version !== selectedVersion);
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
    <Combobox.Root
      type="single"
      items={versions.map((version) => ({ value: version, label: version }))}
      bind:value={selectedVersion}
      onOpenChangeComplete={(isOpen) => {
        if (!isOpen) searchValue = "";
      }}
    >
      <Combobox.Trigger class="version-button" aria-label={`Select ${serviceName} version`}>
        <span>{selectedVersion}</span>
        <ChevronDown size={16} strokeWidth={2} aria-hidden="true" />
      </Combobox.Trigger>

      <Combobox.Portal>
        <Combobox.Content class="version-content" customAnchor=".version-button" sideOffset={6}>
          <div class="version-search-row">
            <Search class="version-search-icon" size={16} strokeWidth={2} aria-hidden="true" />
            <Combobox.Input
              class="version-search"
              oninput={(event) => (searchValue = event.currentTarget.value)}
              placeholder="Search versions"
              aria-label={`Search ${serviceName} versions`}
            />
          </div>
          <Combobox.Viewport>
            {#if installedVersions.length > 0}
              <div class="version-content-heading">Instaladas</div>
              {#each installedVersions as version (version)}
                <Combobox.Item class="version-item" value={version} label={version}>
                  {#snippet children({ selected })}
                    <span>{version}</span>
                    {#if selected}
                      <Check size={16} strokeWidth={2} aria-hidden="true" />
                    {/if}
                  {/snippet}
                </Combobox.Item>
              {/each}
            {/if}

            {#if downloadableVersions.length > 0}
              <div class="version-content-heading">Descargar</div>
              {#each downloadableVersions as version (version)}
                <Combobox.Item class="version-item" value={version} label={version}>
                  {#snippet children({ selected })}
                    <span class="version-item-label">
                      <span class="download-status-dot" aria-hidden="true"></span>
                      <span>{version}</span>
                    </span>
                    <Download class="download-version-icon" size={16} strokeWidth={2} aria-hidden="true" />
                  {/snippet}
                </Combobox.Item>
              {/each}
            {:else if installedVersions.length === 0}
              <span class="version-empty">No versions found</span>
            {/if}
          </Combobox.Viewport>
        </Combobox.Content>
      </Combobox.Portal>
    </Combobox.Root>

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
    appearance: none;
    border-radius: 7px;
    justify-content: space-between;
    width: 300px;
    padding: 0 14px;
  }

  :global(.version-content) {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 7px;
    box-shadow: 0 8px 24px rgb(11 11 11 / 12%);
    box-sizing: border-box;
    max-height: 320px;
    min-width: var(--bits-select-anchor-width);
    overflow-y: auto;
    padding: 8px 4px;
    z-index: 10;
  }

  .version-content-heading {
    color: var(--color-boulder-950);
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.02em;
    padding: 7px 12px 6px;
  }

  :global(.version-item) {
    align-items: center;
    border-radius: 4px;
    color: var(--color-boulder-950);
    cursor: default;
    display: flex;
    font: inherit;
    justify-content: space-between;
    gap: 12px;
    height: 40px;
    min-height: 40px;
    padding: 0 12px;
    user-select: none;
  }

  :global(.version-item[data-highlighted]) {
    background: var(--color-east-bay-50);
    color: var(--color-east-bay-800);
  }

  .version-item-label {
    align-items: center;
    display: inline-flex;
    gap: 10px;
  }

  .download-status-dot {
    border: 1.5px solid var(--color-east-bay-400);
    border-radius: 50%;
    display: inline-block;
    flex: 0 0 8px;
    height: 8px;
    width: 8px;
  }

  :global(.download-version-icon) {
    color: var(--color-east-bay-700);
    flex-shrink: 0;
  }

  .version-search-row {
    align-items: center;
    display: flex;
    position: relative;
  }

  :global(.version-search-icon) {
    color: var(--color-east-bay-800);
    left: 12px;
    pointer-events: none;
    position: absolute;
    z-index: 1;
  }

  :global(.version-search) {
    background: var(--color-boulder-50);
    border: 1px solid var(--color-boulder-200);
    border-radius: 5px;
    box-sizing: border-box;
    color: var(--color-boulder-950);
    font: inherit;
    height: 36px;
    outline: none;
    padding: 0 12px 0 36px;
    width: 100%;
  }

  :global(.version-search:focus) {
    border-color: var(--color-east-bay-500);
  }

  .version-empty {
    color: var(--color-east-bay-800);
    display: block;
    padding: 9px;
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
