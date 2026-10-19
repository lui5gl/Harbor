<script lang="ts">
  import { t } from "#lib/i18n";
  import { Server, Terminal } from "@lucide/svelte";
  import type { ServiceId } from "../types";
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
    onSelect,
  }: ServicesPanelProps = $props();
</script>

<aside
  class="flex h-full max-h-140 flex-col rounded-lg border border-boulder-200 bg-white p-3.5"
  aria-label={t("services.navigation")}
>
  <div class="flex min-h-9.5 items-center justify-between border-b border-boulder-100 pb-3">
    <div class="flex items-center gap-2">
      <h2 class="text-[15px] font-semibold text-boulder-950">{t("services.titleShort")}</h2>
      <span
        class="min-w-4.5 rounded-full bg-boulder-100 px-1.5 text-center text-[11px] font-semibold text-boulder-700"
        title={`2 ${t("services.available")}`}>2</span
      >
    </div>
  </div>
  <div class="mt-2.5 min-h-0 flex-1 overflow-y-auto">
    <section class="flex flex-col gap-1.5" aria-label={t("services.webSection")}>
      <div class="px-1.5 py-1 text-[11px] font-bold uppercase tracking-[0.04em] text-boulder-500">
        {t("services.webSection")}
      </div>
      <div class="flex flex-col gap-1">
        <button
          class={`flex min-h-12 w-full cursor-pointer items-center gap-3 rounded-md border px-2.5 py-2 text-left transition-colors ${selectedServiceId === "php-web" ? "border-east-bay-200 bg-east-bay-50" : "border-transparent hover:bg-boulder-50"}`}
          type="button"
          onclick={() => onSelect("php-web")}
          ><div
            class="flex size-8.5 shrink-0 items-center justify-center rounded-lg bg-east-bay-100 text-east-bay-800"
          >
            <Server size={18} />
          </div>
          <div class="flex min-w-0 flex-1 flex-col">
            <div class="flex items-center justify-between gap-2">
              <span class="font-semibold text-boulder-900">{t("services.webStack")}</span
              >{#if isPhpRunning}<span
                  class="rounded-full bg-emerald-50 px-1.5 py-0.5 text-[9px] font-bold text-emerald-700"
                  >{t("services.running")}</span
                >{/if}
            </div>
            <span class="text-[11px] text-boulder-500"
              >{activePhpVersion
                ? `PHP ${activePhpVersion}`
                : `${installedPhpCount} ${t("services.installed")}`}</span
            >
          </div></button
        >
        <button
          class={`flex min-h-12 w-full cursor-pointer items-center gap-3 rounded-md border px-2.5 py-2 text-left transition-colors ${selectedServiceId === "nodejs" ? "border-east-bay-200 bg-east-bay-50" : "border-transparent hover:bg-boulder-50"}`}
          type="button"
          onclick={() => onSelect("nodejs")}
          ><div
            class="flex size-8.5 shrink-0 items-center justify-center rounded-lg bg-emerald-50 text-emerald-800"
          >
            <Terminal size={18} />
          </div>
          <div class="flex min-w-0 flex-1 flex-col">
            <div class="flex items-center justify-between gap-2">
              <span class="font-semibold text-boulder-900">{t("services.node")}</span
              >{#if activeNodeVersion}<span
                  class="rounded-full bg-boulder-100 px-1.5 py-0.5 text-[9px] font-bold text-boulder-700"
                  >{activeNodeVersion}</span
                >{/if}
            </div>
            <span class="text-[11px] text-boulder-500"
              >{installedNodeCount}
              {installedNodeCount === 1 ? t("services.version") : t("services.versions")}</span
            >
          </div></button
        >
      </div>
    </section>
  </div>
</aside>
