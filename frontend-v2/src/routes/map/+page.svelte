<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import Map from '$lib/components/Map.svelte';
	import MapPreview from '$lib/components/map/MapPreview.svelte';
	import { mapState, mapCenter, mapZoom } from '$lib/stores/map';
	import type { ITree } from '$lib/types';

	let selectedTree: ITree|undefined = $state();

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
	<title>Map</title>
</svelte:head>

<Header title="Tree" />

<Map center={$mapCenter} zoom={$mapZoom} {onChange} {onMove} />

<MapPreview tree={selectedTree} onClose={onClosePreview} />
