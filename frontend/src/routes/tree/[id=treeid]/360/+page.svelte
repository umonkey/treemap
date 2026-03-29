<script lang="ts">
	import { locale } from '$lib/locale';
	import { StreetViewPage } from '$lib/ui';
	import { routes } from '$lib/routes';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { data } = $props();
</script>

<Dialog title={locale.treeShortTitle()}>
	<StreetViewPage id={data.id} />
</Dialog>
