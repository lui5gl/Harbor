<script lang="ts">
  import { Network } from "@lucide/svelte";
  import { Switch } from "bits-ui";
  import { t } from "$lib/i18n";

  type Props = {
    enabled: boolean;
    host: string;
    port: string;
    username: string;
    password: string;
  };

  let {
    enabled = $bindable(),
    host = $bindable(),
    port = $bindable(),
    username = $bindable(),
    password = $bindable(),
  }: Props = $props();
</script>

<section class="settings-section proxy-section" aria-labelledby="proxy-title">
  <div class="section-heading">
    <span class="section-icon"><Network size={18} aria-hidden="true" /></span>
    <div>
      <h2 id="proxy-title">{t("settings.proxy")}</h2>
      <p>{t("settings.proxyDescription")}</p>
    </div>
    <div class="switch-label">
      <Switch.Root class="settings-switch" bind:checked={enabled} aria-label={t("settings.proxy")}>
        <Switch.Thumb class="settings-switch-thumb" />
      </Switch.Root>
      <span>{enabled ? t("settings.enabled") : t("settings.disabled")}</span>
    </div>
  </div>
  {#if enabled}
    <div class="proxy-form">
      <div class="form-row">
        <label for="proxy-host">{t("common.host")}</label>
        <input
          id="proxy-host"
          bind:value={host}
          placeholder="proxy.example.com"
          autocomplete="off"
        />
      </div>
      <div class="form-row port-field">
        <label for="proxy-port">{t("common.port")}</label>
        <input
          id="proxy-port"
          type="number"
          min="1"
          max="65535"
          bind:value={port}
          placeholder="8080"
          inputmode="numeric"
        />
      </div>
      <div class="form-row">
        <label for="proxy-username"
          >{t("common.username")} <span>({t("settings.optional")})</span></label
        >
        <input id="proxy-username" bind:value={username} autocomplete="username" />
      </div>
      <div class="form-row">
        <label for="proxy-password"
          >{t("common.password")} <span>({t("settings.optional")})</span></label
        >
        <input
          id="proxy-password"
          type="password"
          bind:value={password}
          autocomplete="current-password"
        />
      </div>
      <p class="hint">{t("settings.proxyHint")}</p>
    </div>
  {:else}
    <p class="proxy-disabled">{t("settings.directConnection")}</p>
  {/if}
</section>

<style>
  .section-heading {
    align-items: center;
    border-bottom: 1px solid var(--color-boulder-100);
    display: flex;
    gap: 11px;
    padding: 18px;
  }
  .section-icon {
    align-items: center;
    background: var(--color-east-bay-50);
    border: 1px solid var(--color-east-bay-100);
    border-radius: 6px;
    color: var(--color-east-bay-700);
    display: flex;
    height: 34px;
    justify-content: center;
    width: 34px;
  }
  h2 {
    color: var(--color-boulder-900);
    font-size: 14px;
    margin: 0;
  }
  .section-heading p,
  .hint {
    color: var(--color-boulder-500);
    font-size: 12px;
    line-height: 1.45;
    margin: 4px 0 0;
  }
  .switch-label {
    align-items: center;
    color: var(--color-boulder-500);
    display: inline-flex;
    font-size: 11px;
    font-weight: 650;
    gap: 7px;
    margin-left: auto;
  }
  :global(.settings-switch) {
    background: var(--color-boulder-300);
    border: 0;
    border-radius: 999px;
    cursor: pointer;
    flex: 0 0 auto;
    height: 22px;
    padding: 0;
    position: relative;
    width: 38px;
  }
  :global(.settings-switch[data-state="checked"]) {
    background: var(--color-east-bay-700);
  }
  :global(.settings-switch:focus-visible) {
    outline: 2px solid var(--color-east-bay-400);
    outline-offset: 2px;
  }
  :global(.settings-switch-thumb) {
    background: #fff;
    border-radius: 999px;
    display: block;
    height: 18px;
    margin: 2px;
    transform: translateX(0);
    transition: transform 150ms ease;
    width: 18px;
  }
  :global(.settings-switch[data-state="checked"] .settings-switch-thumb) {
    transform: translateX(16px);
  }
  .proxy-form {
    display: grid;
    gap: 15px 16px;
    grid-template-columns: minmax(0, 1fr) 150px;
    padding: 18px;
  }
  .form-row {
    display: flex;
    flex-direction: column;
    gap: 7px;
  }
  .form-row label {
    color: var(--color-boulder-700);
    font-size: 12px;
    font-weight: 650;
  }
  .form-row label span {
    color: var(--color-boulder-500);
    font-weight: 500;
  }
  .form-row input {
    background: #fff;
    border: 1px solid var(--color-boulder-200);
    border-radius: 6px;
    box-sizing: border-box;
    color: var(--color-boulder-800);
    font: inherit;
    font-size: 12px;
    height: 36px;
    padding: 0 10px;
    width: 100%;
  }
  .form-row input:focus {
    border-color: var(--color-east-bay-400);
    outline: 2px solid var(--color-east-bay-100);
  }
  .proxy-form .hint {
    grid-column: 1 / -1;
    margin-top: -2px;
  }
  .proxy-disabled {
    color: var(--color-boulder-500);
    font-size: 12px;
    margin: 0;
    padding: 18px;
  }
  @media (max-width: 760px) {
    .proxy-form {
      grid-template-columns: 1fr;
    }
    .proxy-form .hint {
      grid-column: auto;
    }
  }
</style>
