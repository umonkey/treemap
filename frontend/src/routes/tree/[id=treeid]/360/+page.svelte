<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import StreetViewPage from './StreetViewPage.svelte';

	const { data } = $props();
</script>

<Dialog title={locale.treeShortTitle()} nopadding>
	<StreetViewPage id={data.id} />
</Dialog>
