<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import StreetViewPage from './StreetViewPage.svelte';

	const id = $derived($page.params.id as string);
</script>

<Dialog title={locale.treeShortTitle()} nopadding>
	<StreetViewPage {id} />
</Dialog>
