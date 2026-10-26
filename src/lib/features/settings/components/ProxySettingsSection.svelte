<script lang="ts">
  import { Switch } from "@ark-ui/svelte/switch";
  import { Field } from "@ark-ui/svelte/field";
  import { t } from "#lib/i18n";
  type Props = { enabled: boolean; host: string; port: string; username: string; password: string };
  let {
    enabled = $bindable(),
    host = $bindable(),
    port = $bindable(),
    username = $bindable(),
    password = $bindable(),
  }: Props = $props();
  const inputClass =
    "h-9 w-full rounded-md border border-boulder-300 bg-white px-2.5 text-xs text-boulder-900 outline-none focus:border-east-bay-500 focus:ring-2 focus:ring-east-bay-500/15";
</script>

<section
  class="grid grid-cols-[minmax(220px,1fr)_minmax(0,1.7fr)] gap-x-12 gap-y-4 py-7 max-md:grid-cols-1"
  aria-labelledby="proxy-title"
>
  <div>
    <h2 id="proxy-title" class="text-sm font-semibold text-boulder-950">{t("settings.proxy")}</h2>
    <p class="mt-1 max-w-xs text-xs leading-relaxed text-boulder-600">
      {t("settings.proxyDescription")}
    </p>
  </div>
  <div class="flex min-w-0 flex-col gap-4">
    <Switch.Root
      class="inline-flex cursor-pointer items-center gap-3 text-xs font-semibold text-boulder-800"
      bind:checked={enabled}
      aria-label={t("settings.proxy")}
      ><Switch.Control
        class="relative h-5.5 w-9.5 shrink-0 rounded-full bg-boulder-300 transition-colors data-[state=checked]:bg-east-bay-700 data-focus:outline-2 data-focus:outline-offset-2 data-focus:outline-east-bay-400 after:absolute after:top-0.5 after:left-0.5 after:size-4.5 after:rounded-full after:bg-white after:transition-transform data-[state=checked]:after:translate-x-4"
        ><Switch.Thumb class="sr-only" /></Switch.Control
      ><Switch.HiddenInput /><span>{enabled ? t("settings.enabled") : t("settings.disabled")}</span
      ></Switch.Root
    >
  {#if enabled}
    <div class="grid grid-cols-[minmax(0,1fr)_150px] gap-x-4 gap-y-3.5 max-md:grid-cols-1">
      <Field.Root class="flex flex-col gap-1.5">
        <Field.Label class="text-xs font-semibold text-boulder-700">{t("common.host")}</Field.Label>
        <Field.Input
          id="proxy-host"
          class={inputClass}
          bind:value={host}
          placeholder="proxy.example.com"
          autocomplete="off"
        />
      </Field.Root>
      <Field.Root class="flex flex-col gap-1.5">
        <Field.Label class="text-xs font-semibold text-boulder-700">{t("common.port")}</Field.Label>
        <Field.Input
          id="proxy-port"
          class={inputClass}
          type="number"
          min="1"
          max="65535"
          bind:value={port}
          placeholder="8080"
          inputmode="numeric"
        />
      </Field.Root>
      <Field.Root class="flex flex-col gap-1.5">
        <Field.Label class="text-xs font-semibold text-boulder-700"
          >{t("common.username")}
          <span class="font-normal text-boulder-500">({t("settings.optional")})</span></Field.Label
        >
        <Field.Input
          id="proxy-username"
          class={inputClass}
          bind:value={username}
          autocomplete="username"
        />
      </Field.Root>
      <Field.Root class="flex flex-col gap-1.5">
        <Field.Label class="text-xs font-semibold text-boulder-700"
          >{t("common.password")}
          <span class="font-normal text-boulder-500">({t("settings.optional")})</span></Field.Label
        >
        <Field.Input
          id="proxy-password"
          class={inputClass}
          type="password"
          bind:value={password}
          autocomplete="current-password"
        />
      </Field.Root>
      <p class="col-span-full -mt-0.5 text-xs leading-relaxed text-boulder-600 max-md:col-auto">
        {t("settings.proxyHint")}
      </p>
    </div>
  {:else}<p class="text-xs text-boulder-600">{t("settings.directConnection")}</p>{/if}
  </div>
</section>
