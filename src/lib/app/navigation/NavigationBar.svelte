<script lang="ts">
  import { page } from "$app/state";
  import { Anchor, Boxes, KeyRound, Settings } from "@lucide/svelte";
  import { t } from "#lib/i18n";

  const applicationName = "Harbor";
</script>

<header
  class="flex h-15 shrink-0 select-none items-center justify-between border-b border-boulder-200 bg-boulder-50 px-6 max-sm:px-3"
  data-tauri-drag-region
>
  <a
    class="flex items-center gap-2.5 text-boulder-950 no-underline"
    href="/services"
    aria-label={t("nav.home")}
  >
    <span
      class="flex size-8 items-center justify-center rounded-lg border border-east-bay-100 bg-east-bay-50 text-east-bay-700"
      aria-hidden="true"
    >
      <Anchor size={16} strokeWidth={2} />
    </span>
    <span class="flex items-baseline gap-2">
      <span class="text-[15px] font-semibold">{applicationName}</span>
      <span
        class="text-[11px] font-medium uppercase tracking-[0.04em] text-boulder-500 max-sm:hidden"
        >Desktop</span
      >
    </span>
  </a>

  <nav class="flex items-center gap-1" aria-label={t("nav.primaryNavigation")}>
    {#each [{ href: "/services", label: t("nav.services"), icon: Boxes }, { href: "/secrets", label: t("nav.secrets"), icon: KeyRound }, { href: "/settings", label: t("nav.settings"), icon: Settings }] as item (item.href)}
      {@const isActive =
        page.url.pathname === item.href || page.url.pathname.startsWith(item.href + "/")}
      <a
        class={`inline-flex min-h-8.5 items-center gap-1.5 rounded-md px-2.5 text-[13px] font-semibold no-underline transition-colors max-sm:px-1.5 ${isActive ? "bg-east-bay-50 text-east-bay-700" : "text-boulder-600 hover:bg-boulder-100 hover:text-boulder-950"}`}
        href={item.href}
        data-tauri-drag-region="false"
        aria-current={isActive ? "page" : undefined}
      >
        <item.icon size={16} strokeWidth={1.9} aria-hidden="true" />
        <span class="max-sm:hidden">{item.label}</span>
      </a>
    {/each}
  </nav>
</header>
