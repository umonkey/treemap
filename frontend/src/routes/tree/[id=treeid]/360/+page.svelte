<script lang="ts">
	import { locale } from '$lib/locale';
	import { StreetViewPage } from '$lib/ui';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { data } = $props();
</script>

<Dialog title={locale.treeShortTitle()} nopadding>
	<StreetViewPage id={data.id} />
</Dialog>
