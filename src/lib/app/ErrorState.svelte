<script lang="ts">
  import type { Snippet } from "svelte";
  import { House, RotateCw, SearchX, TriangleAlert } from "@lucide/svelte";
  import { t } from "#lib/i18n";

  type Props = {
    status?: number;
    title?: string;
    description?: string;
    details?: string;
    onretry?: () => void;
    homeHref?: string | null;
    actions?: Snippet;
  };

  let {
    status = 500,
    title,
    description,
    details,
    onretry = () => location.reload(),
    homeHref = "/",
    actions,
  }: Props = $props();

  let isNotFound = $derived(status === 404);
  let resolvedTitle = $derived(
    title ?? t(isNotFound ? "errorPage.notFoundTitle" : "errorPage.genericTitle"),
  );
  let resolvedDescription = $derived(
    description ?? t(isNotFound ? "errorPage.notFoundDescription" : "errorPage.genericDescription"),
  );
</script>

<main class="flex flex-1 items-center justify-center px-6 py-16" role="alert">
  <div class="flex w-full max-w-md flex-col items-center text-center">
    <div
      class="mb-6 flex size-14 items-center justify-center rounded-2xl border border-boulder-200 bg-white text-boulder-700 shadow-sm"
    >
      {#if isNotFound}
        <SearchX class="size-6" aria-hidden="true" />
      {:else}
        <TriangleAlert class="size-6" aria-hidden="true" />
      {/if}
    </div>

    <p class="font-mono text-xs font-medium tracking-widest text-boulder-500 uppercase">
      {t("errorPage.eyebrow")} · {status}
    </p>
    <h1 class="mt-2 text-2xl font-semibold tracking-tight text-boulder-950">{resolvedTitle}</h1>
    <p class="mt-2 text-sm leading-relaxed text-boulder-600">{resolvedDescription}</p>

    <div class="mt-8 flex flex-wrap items-center justify-center gap-3">
      {#if !isNotFound}
        <button
          type="button"
          onclick={onretry}
          class="inline-flex items-center gap-2 rounded-lg bg-east-bay-700 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-east-bay-800 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-east-bay-600"
        >
          <RotateCw class="size-4" aria-hidden="true" />
          {t("errorPage.retry")}
        </button>
      {/if}
      {#if homeHref}
        <a
          href={homeHref}
          class="inline-flex items-center gap-2 rounded-lg border border-boulder-300 bg-white px-4 py-2 text-sm font-medium text-boulder-800 transition-colors hover:bg-boulder-100 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-east-bay-600"
        >
          <House class="size-4" aria-hidden="true" />
          {t("errorPage.goHome")}
        </a>
      {/if}
      {@render actions?.()}
    </div>

    {#if details}
      <details class="mt-8 w-full text-left">
        <summary class="cursor-pointer text-xs font-medium text-boulder-500 hover:text-boulder-700">
          {t("errorPage.technicalDetails")}
        </summary>
        <pre
          class="mt-2 max-h-40 overflow-auto rounded-lg border border-boulder-200 bg-boulder-100 p-3 font-mono text-xs break-words whitespace-pre-wrap text-boulder-700">{details}</pre>
      </details>
    {/if}
  </div>
</main>
