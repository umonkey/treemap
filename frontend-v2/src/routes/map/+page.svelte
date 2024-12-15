<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import Map from '$lib/components/Map.svelte';
	import MapPreview from '$lib/components/map/MapPreview.svelte';
	import { mapState, mapCenter, mapZoom } from '$lib/stores/map';
	import type { ITree } from '$lib/types';

	const { data } = $props();
	const searchQuery = data.searchQuery;

	let selectedTree: ITree | undefined = $state();

	const title = searchQuery ? `Map: ${searchQuery}` : 'Map';

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
	<title>Tree Map</title>
</svelte:head>

<Header {title} />

<Map center={$mapCenter} zoom={$mapZoom} {onChange} {onMove} {searchQuery} />

<MapPreview tree={selectedTree} onClose={onClosePreview} />
