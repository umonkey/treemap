<script lang="ts">
	import { locale } from '$lib/locale';
	import { onMount } from 'svelte';
	import { pageState } from './hooks.svelte.ts';
	import MapLibre from '$lib/components/map/MapLibre.svelte';

	const { data } = $props();

	onMount(pageState.onMount);

	$effect(() => {
		pageState.handleTreeChange(data.preview);
	});
</script>

<svelte:head>
	<title>{locale.mapTitle()}</title>
</svelte:head>

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
