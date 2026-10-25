<script lang="ts">
  import { page } from "$app/state";
  import ErrorState from "#lib/app/ErrorState.svelte";
</script>

<ErrorState status={page.status} details={page.error?.message} />
