<script lang="ts">
  import { Button, ScrollArea } from "bits-ui";
  import { Server, Terminal } from "@lucide/svelte";
  import type { ServiceId } from "./types";

  type ServicesPanelProps = {
    selectedServiceId: ServiceId;
    activePhpVersion: string | null;
    isPhpRunning: boolean;
    activeNodeVersion: string | null;
    installedPhpCount: number;
    installedNodeCount: number;
    onSelect: (id: ServiceId) => void;
  };

  let {
    selectedServiceId,
    activePhpVersion,
    isPhpRunning,
    activeNodeVersion,
    installedPhpCount,
    installedNodeCount,
    onSelect
  }: ServicesPanelProps = $props();
</script>

<aside class="services-panel" aria-label="Services & Runtimes navigation">
  <div class="panel-header">
    <div class="header-title-row">
      <h2>Services</h2>
      <span class="service-count" title="2 services available">2</span>
    </div>
  </div>

  <ScrollArea.Root class="service-scroll-area" type="auto">
    <ScrollArea.Viewport class="service-list-viewport">
      <div class="service-sections">
        <section class="section-group" aria-label="Web Server & Runtime Stacks">
          <div class="section-heading">
            <span class="section-title">Web Stack & Runtimes</span>
          </div>

          <div class="service-list">
            <!-- PHP & Web Stack -->
            <Button.Root
              class={`service-item${selectedServiceId === "php-web" ? " selected" : ""}`}
              type="button"
              onclick={() => onSelect("php-web")}
            >
              <div class="service-item-icon web-stack-icon" aria-hidden="true">
                <Server size={18} strokeWidth={2} />
              </div>
              <div class="service-item-body">
                <div class="service-primary-row">
                  <span class="service-name">PHP & Web Stack</span>
                  {#if isPhpRunning}
                    <span class="running-badge">Running</span>
                  {/if}
                </div>
                <div class="service-secondary-row">
                  <span class="service-meta">
                    {activePhpVersion ? `PHP ${activePhpVersion}` : `${installedPhpCount} installed`}
                  </span>
                </div>
              </div>
            </Button.Root>

            <!-- Node.js Runtime -->
            <Button.Root
              class={`service-item${selectedServiceId === "nodejs" ? " selected" : ""}`}
              type="button"
              onclick={() => onSelect("nodejs")}
            >
              <div class="service-item-icon node-icon" aria-hidden="true">
                <Terminal size={18} strokeWidth={2} />
              </div>
              <div class="service-item-body">
                <div class="service-primary-row">
                  <span class="service-name">Node.js</span>
                  {#if activeNodeVersion}
                    <span class="active-badge">{activeNodeVersion}</span>
                  {/if}
                </div>
                <div class="service-secondary-row">
                  <span class="service-meta">
                    {installedNodeCount} {installedNodeCount === 1 ? "version" : "versions"}
                  </span>
                </div>
              </div>
            </Button.Root>
          </div>
        </section>
      </div>
    </ScrollArea.Viewport>
    <ScrollArea.Scrollbar class="service-scrollbar" orientation="vertical">
      <ScrollArea.Thumb class="service-scrollbar-thumb" />
    </ScrollArea.Scrollbar>
  </ScrollArea.Root>
</aside>

<style>
  .services-panel {
    background: #ffffff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 8px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    height: 100%;
    max-height: 560px;
    padding: 14px;
  }

  .panel-header {
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
    box-sizing: border-box;
    display: flex;
    justify-content: space-between;
    min-height: 38px;
    padding-bottom: 12px;
  }

  .header-title-row {
    align-items: center;
    display: flex;
    gap: 8px;
  }

  h2 {
    color: var(--color-boulder-950);
    font-size: 15px;
    font-weight: 650;
    margin: 0;
  }

  .service-count {
    background: var(--color-boulder-100);
    border-radius: 999px;
    color: var(--color-boulder-700);
    font-size: 11px;
    font-weight: 600;
    min-width: 18px;
    padding: 1px 6px;
    text-align: center;
  }

  :global(.service-scroll-area) {
    display: flex;
    flex: 1;
    margin-top: 10px;
    min-height: 0;
  }

  :global(.service-list-viewport) {
    height: 100%;
    width: 100%;
  }

  .service-sections {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .section-heading {
    color: var(--color-boulder-500);
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.04em;
    padding: 4px 6px;
    text-transform: uppercase;
  }

  .service-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  :global(.service-item) {
    align-items: center;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    box-sizing: border-box;
    cursor: pointer;
    display: flex;
    gap: 12px;
    min-height: 48px;
    padding: 8px 10px;
    text-align: left;
    transition: background-color 150ms ease, border-color 150ms ease;
    width: 100%;
  }

  :global(.service-item:hover) {
    background: var(--color-boulder-50);
  }

  :global(.service-item.selected) {
    background: var(--color-east-bay-50);
    border-color: var(--color-east-bay-200);
  }

  .service-item-icon {
    align-items: center;
    border-radius: 8px;
    display: flex;
    flex-shrink: 0;
    height: 34px;
    justify-content: center;
    width: 34px;
  }

  .web-stack-icon {
    background: var(--color-east-bay-100);
    color: var(--color-east-bay-800);
  }

  .node-icon {
    background: #ecfdf5;
    color: #065f46;
  }

  .service-item-body {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .service-primary-row {
    align-items: center;
    display: flex;
    justify-content: space-between;
    gap: 6px;
  }

  .service-name {
    color: var(--color-boulder-950);
    font-size: 13px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .service-secondary-row {
    align-items: center;
    display: flex;
  }

  .service-meta {
    color: var(--color-boulder-500);
    font-size: 11.5px;
    font-weight: 500;
  }

  .running-badge {
    background: #dcfce7;
    border: 1px solid #bbf7d0;
    border-radius: 999px;
    color: #166534;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.04em;
    padding: 1px 6px;
    text-transform: uppercase;
  }

  .active-badge {
    background: var(--color-east-bay-100);
    border-radius: 999px;
    color: var(--color-east-bay-800);
    font-size: 10.5px;
    font-weight: 600;
    padding: 1px 6px;
  }

  :global(.service-scrollbar) {
    background: transparent;
    display: flex;
    padding: 2px;
    user-select: none;
    width: 6px;
  }

  :global(.service-scrollbar-thumb) {
    background: var(--color-boulder-300);
    border-radius: 4px;
    flex: 1;
  }
</style>
