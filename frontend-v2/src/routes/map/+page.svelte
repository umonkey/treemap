<script>
	import Header from '$lib/components/tree/Header.svelte';
	import Map from '$lib/components/Map.svelte';
	import MapPreview from '$lib/components/map/MapPreview.svelte';
	import { mapState, mapCenter, mapZoom } from '$lib/stores/map';

	let selectedTree = $state(null);

	const onChange = (tree) => {
		selectedTree = tree;
	};

	const onClosePreview = () => {
		selectedTree = null;
	};

	const onMove = (center, zoom) => {
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
