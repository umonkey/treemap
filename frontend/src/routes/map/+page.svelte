<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import Map from '$lib/components/Map.svelte';
	import MapPreview from '$lib/components/map/MapPreview.svelte';
	import { mapState, mapCenter, mapZoom } from '$lib/stores/map';
	import type { ITree } from '$lib/types';
	import { locale } from '$lib/locale';

	const { data } = $props();
	const searchQuery = data.searchQuery;

	let selectedTree: ITree | undefined = $state();

	const title = searchQuery ? locale.mapTitleQuery(searchQuery) : locale.mapTitle();

	const onChange = (tree: ITree) => {
		selectedTree = tree;
	};

	const onClosePreview = () => {
		selectedTree = undefined;
	};

	const onMove = (center: number[], zoom: number) => {
		mapState.set({
			center,
			zoom
		});
	};
</script>

<svelte:head>
	<title>{locale.mapTitle()}</title>
</svelte:head>

<Header {title} />

<div class="mapContainer">
	<Map center={$mapCenter} zoom={$mapZoom} {onChange} {onMove} {searchQuery} canAdd={true} />
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
