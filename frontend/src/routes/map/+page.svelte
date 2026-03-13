<script lang="ts">
	import { locale } from '$lib/locale';
	import { mapCenter, mapZoom } from '$lib/stores/mapStore';
	import { isMapperMode } from '$lib/stores/modeStore';
	import { Header, Map, MapAddRow, MapAddTree, MapCenter, MapPin, MapSearch } from '$lib/ui';
	import { onDestroy, onMount } from 'svelte';
	import { hooks } from './hooks';

	const { data } = $props();

	const { pin, handlePreviewChange, handleSearchQuery, handleAddTree, handleAddRow } = hooks(
		onMount,
		onDestroy
	);

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
	<Map center={$mapCenter} zoom={$mapZoom} searchQuery={data.searchQuery}>
		{#if $pin}
			<MapPin center={$pin} />
		{/if}

		<MapSearch />

		{#if $isMapperMode}
			<MapCenter />
			<MapAddTree onConfirm={handleAddTree} />
			<MapAddRow onConfirm={handleAddRow} />
		{/if}
	</Map>
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
