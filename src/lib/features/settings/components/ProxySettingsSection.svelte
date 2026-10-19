<script lang="ts">
  import { Network } from "@lucide/svelte";
  import { t } from "#lib/i18n";
  type Props = { enabled: boolean; host: string; port: string; username: string; password: string };
  let {
    enabled = $bindable(),
    host = $bindable(),
    port = $bindable(),
    username = $bindable(),
    password = $bindable(),
  }: Props = $props();
</script>

<section
  class="col-span-full min-w-0 rounded-lg border border-boulder-200 bg-white"
  aria-labelledby="proxy-title"
>
  <div class="flex items-center gap-3 border-b border-boulder-100 p-4.5">
    <span
      class="flex size-8.5 items-center justify-center rounded-md border border-east-bay-100 bg-east-bay-50 text-east-bay-700"
      ><Network size={18} aria-hidden="true" /></span
    >
    <div>
      <h2 id="proxy-title" class="text-sm font-semibold text-boulder-900">{t("settings.proxy")}</h2>
      <p class="mt-1 text-xs leading-relaxed text-boulder-500">{t("settings.proxyDescription")}</p>
    </div>
    <label
      class="ml-auto inline-flex cursor-pointer items-center gap-2 text-[11px] font-semibold text-boulder-500"
      ><input
        class="peer sr-only"
        type="checkbox"
        bind:checked={enabled}
        aria-label={t("settings.proxy")}
      /><span
        class="relative h-5.5 w-9.5 rounded-full bg-boulder-300 transition peer-checked:bg-east-bay-700 peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-east-bay-400 after:absolute after:top-0.5 after:left-0.5 after:size-4.5 after:rounded-full after:bg-white after:transition-transform peer-checked:after:translate-x-4"
      ></span><span>{enabled ? t("settings.enabled") : t("settings.disabled")}</span></label
    >
  </div>
  {#if enabled}
    <div class="grid grid-cols-[minmax(0,1fr)_150px] gap-x-4 gap-y-3.5 p-4.5 max-md:grid-cols-1">
      <label class="flex flex-col gap-1.5"
        ><span class="text-xs font-semibold text-boulder-700">{t("common.host")}</span><input
          id="proxy-host"
          class="h-9 w-full rounded-md border border-boulder-200 bg-white px-2.5 text-xs text-boulder-800 outline-none focus:border-east-bay-400 focus:ring-2 focus:ring-east-bay-100"
          bind:value={host}
          placeholder="proxy.example.com"
          autocomplete="off"
        /></label
      >
      <label class="flex flex-col gap-1.5"
        ><span class="text-xs font-semibold text-boulder-700">{t("common.port")}</span><input
          id="proxy-port"
          class="h-9 w-full rounded-md border border-boulder-200 bg-white px-2.5 text-xs text-boulder-800 outline-none focus:border-east-bay-400 focus:ring-2 focus:ring-east-bay-100"
          type="number"
          min="1"
          max="65535"
          bind:value={port}
          placeholder="8080"
          inputmode="numeric"
        /></label
      >
      <label class="flex flex-col gap-1.5"
        ><span class="text-xs font-semibold text-boulder-700"
          >{t("common.username")}
          <span class="font-normal text-boulder-500">({t("settings.optional")})</span></span
        ><input
          id="proxy-username"
          class="h-9 w-full rounded-md border border-boulder-200 bg-white px-2.5 text-xs text-boulder-800 outline-none focus:border-east-bay-400 focus:ring-2 focus:ring-east-bay-100"
          bind:value={username}
          autocomplete="username"
        /></label
      >
      <label class="flex flex-col gap-1.5"
        ><span class="text-xs font-semibold text-boulder-700"
          >{t("common.password")}
          <span class="font-normal text-boulder-500">({t("settings.optional")})</span></span
        ><input
          id="proxy-password"
          class="h-9 w-full rounded-md border border-boulder-200 bg-white px-2.5 text-xs text-boulder-800 outline-none focus:border-east-bay-400 focus:ring-2 focus:ring-east-bay-100"
          type="password"
          bind:value={password}
          autocomplete="current-password"
        /></label
      >
      <p class="col-span-full -mt-0.5 text-xs leading-relaxed text-boulder-500 max-md:col-auto">
        {t("settings.proxyHint")}
      </p>
    </div>
  {:else}<p class="p-4.5 text-xs text-boulder-500">{t("settings.directConnection")}</p>{/if}
</section>
