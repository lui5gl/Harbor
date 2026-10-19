<script lang="ts">
  import { Dialog } from "@ark-ui/svelte/dialog";
  import { Loader2, X } from "@lucide/svelte";
  import InstallVersionList from "./InstallVersionList.svelte";
  type InstallVersionDialogProps = {
    open: boolean;
    serviceLabel: string;
    availableVersions: string[];
    installedVersions: string[];
    isInstalling: boolean;
    installProgress: number;
    installingVersion: string;
    installError?: string;
    onOpenChange: (open: boolean) => void;
    onInstall: (version: string) => Promise<void> | void;
  };
  let {
    open = $bindable(),
    serviceLabel,
    availableVersions,
    installedVersions,
    isInstalling,
    installProgress,
    installingVersion,
    installError = "",
    onOpenChange,
    onInstall,
  }: InstallVersionDialogProps = $props();
  function handleInstallClick(versionRaw: string) {
    if (!isInstalling) void onInstall(versionRaw);
  }
</script>

<Dialog.Root
  bind:open
  onOpenChange={(details) => {
    if (isInstalling && !details.open) {
      open = true;
      return;
    }
    onOpenChange(details.open);
  }}
>
  <Dialog.Backdrop class="fixed inset-0 z-50 bg-black/45 backdrop-blur-xs" />
  <Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <Dialog.Content
      class="flex h-[min(640px,calc(100dvh-3rem))] w-full max-w-xl flex-col rounded-xl border border-boulder-200 bg-white p-6 shadow-2xl outline-none"
    >
      <div class="mb-4 flex items-start justify-between">
        <div class="min-w-0">
          <Dialog.Title class="text-[17px] font-semibold text-boulder-950"
            >Install {serviceLabel} Version</Dialog.Title
          ><Dialog.Description class="mt-1 text-[13.5px] leading-relaxed text-boulder-600"
            >Download and install binary versions into your local Harbor runtimes repository.</Dialog.Description
          >
        </div>
        <button
          class="inline-flex size-8 cursor-pointer items-center justify-center rounded-md text-boulder-500 hover:bg-boulder-100 hover:text-boulder-950 disabled:opacity-40"
          type="button"
          disabled={isInstalling}
          onclick={() => (open = false)}
          aria-label="Close dialog"><X size={18} /></button
        >
      </div>
      {#if installError}<div
          class="mb-3.5 rounded-md border border-red-200 bg-red-50 px-3 py-2.5 text-[13px] text-red-800"
          role="alert"
        >
          {installError}
        </div>{/if}
      {#if isInstalling}<div
          class="mb-4 flex flex-col gap-2 rounded-lg border border-east-bay-200 bg-east-bay-50 px-3.5 py-3"
          role="status"
        >
          <div class="flex items-center gap-2">
            <Loader2 size={16} class="animate-spin text-east-bay-700" /><span
              class="flex-1 text-[13px] font-semibold text-east-bay-900"
              >Downloading & installing {serviceLabel} {installingVersion}...</span
            ><span class="font-mono text-xs text-east-bay-700">{installProgress}%</span>
          </div>
          <div class="h-1.5 overflow-hidden rounded-full bg-east-bay-100">
            <div
              class="h-full rounded-full bg-east-bay-700 transition-[width]"
              style={`width: ${Math.max(4, installProgress)}%`}
            ></div>
          </div>
        </div>{/if}
      <InstallVersionList
        {serviceLabel}
        {availableVersions}
        {installedVersions}
        {isInstalling}
        onInstall={handleInstallClick}
      />
    </Dialog.Content>
  </Dialog.Positioner>
</Dialog.Root>
