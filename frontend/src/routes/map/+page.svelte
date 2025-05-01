<script lang="ts">
	import { Header, Map, MapPreview } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { mapCenter, mapStore, mapZoom } from '$lib/stores/mapStore';
	import { isMapperMode } from '$lib/stores/modeStore';
	import { hooks } from './hooks';
	import { onMount, onDestroy } from 'svelte';

	const { data } = $props();
	const searchQuery = data.searchQuery;

	const { handlePreviewChange } = hooks(onMount, onDestroy);

	const title = searchQuery ? locale.mapTitleQuery(searchQuery) : locale.mapTitle();

	// The user moves/pans the map.  Save the new center and zoom.
	const onMove = (center: number[], zoom: number) => {
		mapStore.update((state) => ({
			...state,
			center,
			zoom
		}));
	};

	$effect(() => handlePreviewChange(data.preview));
</script>

<svelte:head>
	<title>{locale.mapTitle()}</title>
</svelte:head>

<Header {title} />

<div class="mapContainer">
	<Map
		center={$mapCenter}
		zoom={$mapZoom}
		{onMove}
		{searchQuery}
		crosshair={$isMapperMode}
		canAdd={$isMapperMode}
	/>

	<MapPreview id={data.preview} />
</div>

<style>
	.mapContainer {
		height: calc(100dvh - 41px);
		z-index: var(--z-map-preview);
		position: relative;
	}

	@media (max-width: 480px) {
		.mapContainer {
			height: calc(100dvh - 91px);
		}
	}
</style>
