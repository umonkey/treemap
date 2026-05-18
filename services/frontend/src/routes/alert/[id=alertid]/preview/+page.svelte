<script lang="ts">
	import MapPreview from './MapPreview.svelte';
	import { page } from '$app/stores';
	import { locale } from './lang';

	const id = $derived($page.params.id as string);
</script>

<svelte:head>
	<title>{locale.tabTitle(id)}</title>
</svelte:head>

<MapPreview {id} />
