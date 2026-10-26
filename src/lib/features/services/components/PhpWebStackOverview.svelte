<script lang="ts">
  import { ExternalLink, FolderOpen } from "@lucide/svelte";
  type Props = {
    localUrl: string;
    documentRoot: string;
    activeProfileName: string;
    onOpenUrl: (url: string) => void;
    onOpenPath: (path: string) => void;
  };
  let { localUrl, documentRoot, activeProfileName, onOpenUrl, onOpenPath }: Props = $props();
</script>

<div class="grid gap-4 md:grid-cols-3 md:divide-x md:divide-boulder-100">
  <div class="flex min-w-0 flex-col gap-1 md:pr-5">
    <span class="text-xs font-medium text-boulder-500">URL local</span>
    <button
      type="button"
      class="inline-flex max-w-full cursor-pointer items-center gap-1.5 text-left focus-visible:outline-2 focus-visible:outline-east-bay-400"
      onclick={() => onOpenUrl(localUrl)}
      title={`Abrir ${localUrl} en el navegador`}
      ><span
        class="truncate font-mono text-[13px] font-semibold text-boulder-900 hover:text-east-bay-700 hover:underline"
        >{localUrl}</span
      ><ExternalLink size={13} class="shrink-0 text-boulder-500" /></button
    >
  </div>
  <div class="flex min-w-0 flex-col gap-1 md:px-5">
    <span class="text-xs font-medium text-boulder-500">Carpeta web (Document Root)</span>
    <button
      type="button"
      class="inline-flex max-w-full cursor-pointer items-center gap-1.5 text-left focus-visible:outline-2 focus-visible:outline-east-bay-400"
      onclick={() => onOpenPath(documentRoot)}
      title={`Abrir ${documentRoot} en el explorador de archivos`}
      ><span
        class="truncate font-mono text-[13px] font-semibold text-boulder-900 hover:text-east-bay-700 hover:underline"
        >{documentRoot}</span
      ><FolderOpen size={13} class="shrink-0 text-boulder-500" /></button
    >
  </div>
  <div class="flex min-w-0 flex-col gap-1 md:pl-5">
    <span class="text-xs font-medium text-boulder-500">Perfil activo</span>
    <span class="truncate text-[13px] font-semibold text-boulder-900">{activeProfileName}</span>
  </div>
</div>
