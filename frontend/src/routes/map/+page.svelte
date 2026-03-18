<script lang="ts">
	import { locale } from '$lib/locale';
	import { Header } from '$lib/ui';
	import { onDestroy, onMount } from 'svelte';
	import { hooks } from './hooks';
	import MapLibre from '$lib/components/map/MapLibre.svelte';

	const { data } = $props();

	const { handlePreviewChange, handleSearchQuery } = hooks(onMount, onDestroy);

	const title = data.searchQuery ? locale.mapTitleQuery(data.searchQuery) : locale.mapTitle();

	$effect(() => {
		handlePreviewChange(data.preview);
	});
	$effect(() => {
		handleSearchQuery(data.searchQuery);
	});
</script>

<svelte:head>
	<title>{locale.mapTitle()}</title>
</svelte:head>

<Header {title} />

<div class="mapContainer">
	<MapLibre />
</div>

<style>
	.mapContainer {
		z-index: var(--z-map-preview);
		position: relative;

		/* Add space (50px) to the bottom nav bar on mobile */
		height: calc(100dvh - var(--bottom-nav-height));
	}

	@media (min-width: 1024px) {
		.mapContainer {
			/* No need for bottom nav bar on desktop */
			height: calc(100dvh - var(--header-height));
		}
	}
</style>
