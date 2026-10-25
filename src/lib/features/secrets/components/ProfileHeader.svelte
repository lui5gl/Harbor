<script lang="ts">
  import { t } from "#lib/i18n";
  import { Code2, FlaskConical, FolderGit2 } from "@lucide/svelte";
  import type { Environment, Project } from "../types";

  type Props = {
    project: Project;
    environment: Environment;
    activeEnvironmentId: number | null;
  };

  let { project, environment, activeEnvironmentId }: Props = $props();
  let isActive = $derived(environment.id === activeEnvironmentId);
</script>

<header class="mb-3 flex flex-col gap-3 border-b border-boulder-200 pb-3.5">
  <div class="flex flex-wrap items-center justify-between gap-3">
    <div class="flex min-w-0 items-center gap-2">
      <div class="flex min-w-0 items-center gap-1.5">
        <FolderGit2 size={14} class="shrink-0 text-east-bay-700" aria-hidden="true" />
        <span class="max-w-40 truncate text-sm font-semibold text-boulder-500">{project.name}</span>
      </div>
      <span class="text-sm font-medium text-boulder-300">/</span>
      <div class="flex min-w-0 items-center gap-1.5">
        {#if environment.name.toLowerCase() === "development"}<Code2
            size={14}
            class="shrink-0 text-east-bay-700"
            aria-hidden="true"
          />{:else if environment.name.toLowerCase() === "staging"}<FlaskConical
            size={14}
            class="shrink-0 text-violet-700"
            aria-hidden="true"
          />{/if}
        <h2 id="environment-title" class="truncate text-sm font-semibold text-boulder-950">
          {environment.name}
        </h2>
        {#if isActive}<span
            class="inline-flex items-center gap-1.5 rounded-full border border-emerald-100 bg-emerald-50 px-2 py-0.5 text-[10.5px] font-semibold text-emerald-700"
            ><span class="size-1.5 rounded-full bg-emerald-500 ring-2 ring-emerald-500/20"
            ></span><span>{t("secrets.inWindows")}</span></span
          >{/if}
      </div>
    </div>
  </div>
</header>
