<script lang="ts">
import Map from "$lib/components/Map.svelte";
import MapPreview from "$lib/components/map/MapPreview.svelte";
import Header from "$lib/components/tree/Header.svelte";
import { locale } from "$lib/locale";
import { goto, routes } from "$lib/routes";
import { mapCenter, mapStore, mapZoom } from "$lib/stores/mapStore";
import { isMapperMode } from "$lib/stores/modeStore";
import type { ITree } from "$lib/types";

const { data } = $props();
const searchQuery = data.searchQuery;
const selectedTree = $derived(data.preview);

const title = searchQuery
	? locale.mapTitleQuery(searchQuery)
	: locale.mapTitle();

// This is called when a user clicks a tree on the map.
// We need to show a preview, for this we redirect to the page
// with the right arguments.
const onChange = (tree: ITree) => {
	goto(routes.mapPreview(tree.id));
};

const onClosePreview = () => {
	goto(routes.map());
};

const onMove = (center: number[], zoom: number) => {
	mapStore.set({
		center,
		zoom,
	});
};
</script>

<svelte:head>
	<title>{locale.mapTitle()}</title>
</svelte:head>

<Header {title} />

<div class="mapContainer">
	<Map
		center={$mapCenter}
		zoom={$mapZoom}
		{onChange}
		{onMove}
		{searchQuery}
		crosshair={$isMapperMode}
		canAdd={$isMapperMode}
	/>
	<MapPreview tree={selectedTree} onClose={onClosePreview} />
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
