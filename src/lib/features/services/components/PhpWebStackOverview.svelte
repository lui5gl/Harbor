<script lang="ts">
  import { ExternalLink, Folder, FolderOpen, Globe, Layers } from "@lucide/svelte";
  type Props = {
    localUrl: string;
    documentRoot: string;
    activeProfileName: string;
    onOpenUrl: (url: string) => void;
    onOpenPath: (path: string) => void;
  };
  let { localUrl, documentRoot, activeProfileName, onOpenUrl, onOpenPath }: Props = $props();
</script>

<div
  class="flex flex-col gap-4 rounded-lg border border-boulder-200 bg-boulder-50 px-4.5 py-3.5 md:flex-row md:items-center"
>
  <div class="flex min-w-0 flex-1 flex-col gap-1">
    <div class="flex items-center gap-1.5 text-boulder-500">
      <Globe size={14} class="text-boulder-400" /><span
        class="text-[11px] font-semibold uppercase tracking-[0.03em]">URL Local</span
      >
    </div>
    <button
      type="button"
      class="inline-flex max-w-full cursor-pointer items-center gap-1.5 text-left focus-visible:outline-2 focus-visible:outline-east-bay-400"
      onclick={() => onOpenUrl(localUrl)}
      title={`Abrir ${localUrl} en el navegador`}
      ><span
        class="truncate font-mono text-[13px] font-semibold text-boulder-900 hover:text-east-bay-700 hover:underline"
        >{localUrl}</span
      ><ExternalLink size={13} class="shrink-0 text-boulder-400" /></button
    >
  </div>
  <div class="hidden h-8 w-px bg-boulder-200 md:block"></div>
  <div class="flex min-w-0 flex-1 flex-col gap-1">
    <div class="flex items-center gap-1.5 text-boulder-500">
      <Folder size={14} class="text-boulder-400" /><span
        class="text-[11px] font-semibold uppercase tracking-[0.03em]"
        >Carpeta Web (Document Root)</span
      >
    </div>
    <button
      type="button"
      class="inline-flex max-w-full cursor-pointer items-center gap-1.5 text-left focus-visible:outline-2 focus-visible:outline-east-bay-400"
      onclick={() => onOpenPath(documentRoot)}
      title={`Abrir ${documentRoot} en el explorador de archivos`}
      ><span
        class="truncate font-mono text-[13px] font-semibold text-boulder-900 hover:text-east-bay-700 hover:underline"
        >{documentRoot}</span
      ><FolderOpen size={13} class="shrink-0 text-boulder-400" /></button
    >
  </div>
  <div class="hidden h-8 w-px bg-boulder-200 md:block"></div>
  <div class="flex min-w-0 flex-1 flex-col gap-1">
    <div class="flex items-center gap-1.5 text-boulder-500">
      <Layers size={14} class="text-boulder-400" /><span
        class="text-[11px] font-semibold uppercase tracking-[0.03em]">Perfil Activo</span
      >
    </div>
    <span class="truncate text-[13px] font-bold text-east-bay-900">{activeProfileName}</span>
  </div>
</div>
