<script lang="ts">
	import { Header, Map, MapPreview } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { mapCenter, mapZoom } from '$lib/stores/mapStore';
	import { isMapperMode } from '$lib/stores/modeStore';
	import { hooks } from './hooks';
	import { onMount, onDestroy } from 'svelte';

	const { data } = $props();
	const { pins, handlePreviewChange, handleSearchQuery } = hooks(onMount, onDestroy);

	const title = data.searchQuery ? locale.mapTitleQuery(data.searchQuery) : locale.mapTitle();

	$effect(() => handlePreviewChange(data.preview));
	$effect(() => handleSearchQuery(data.searchQuery));
</script>

<svelte:head>
	<title>{locale.mapTitle()}</title>
</svelte:head>

<Header {title} />

<div class="mapContainer">
	<Map
		center={$mapCenter}
		pins={$pins}
		zoom={$mapZoom}
		searchQuery={data.searchQuery}
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
