<script lang="ts">
  import { t } from "#lib/i18n";
  import { FolderGit2 } from "@lucide/svelte";
  import type { Environment, Project } from "../types";

  type Props = {
    project: Project;
    environment: Environment;
    activeEnvironmentId: number | null;
  };

  let { project, environment, activeEnvironmentId }: Props = $props();
  let isActive = $derived(environment.id === activeEnvironmentId);
</script>

<header
  class="mb-2.5 flex min-h-12.25 items-center justify-between gap-2 border-b border-boulder-200 px-1 pt-0.5 pb-2.5"
>
  <div class="flex min-w-0 items-center gap-0.5 text-[13px] font-medium text-boulder-800">
    <FolderGit2 size={14} class="mr-1 shrink-0 text-east-bay-700" aria-hidden="true" />
    <span class="max-w-40 truncate">{project.name}</span>
    <span class="shrink-0 text-boulder-300" aria-hidden="true">/</span>
    <h2 id="environment-title" class="truncate font-medium">{environment.name}</h2>
  </div>
  {#if isActive}<span
      class="inline-flex h-7 shrink-0 items-center gap-1.5 rounded-md border border-emerald-100 bg-emerald-50 px-2.5 text-[11px] font-semibold text-emerald-700"
      ><span class="size-1.5 rounded-full bg-emerald-500 ring-2 ring-emerald-500/20"></span><span
        >{t("secrets.inWindows")}</span
      ></span
    >{/if}
</header>
