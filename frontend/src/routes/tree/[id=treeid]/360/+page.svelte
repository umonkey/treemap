<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import StreetViewPage from '$lib/ui/street-view-page/StreetViewPage.svelte';

	const { data } = $props();
</script>

<Dialog title={locale.treeShortTitle()} nopadding>
	<StreetViewPage id={data.id} />
</Dialog>
