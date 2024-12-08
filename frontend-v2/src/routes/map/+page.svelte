<script>
	import Header from '$lib/components/tree/Header.svelte';
	import Map from '$lib/components/Map.svelte';
	import MapPreview from '$lib/components/map/MapPreview.svelte';
	import { DEFAULT_MAP_CENTER } from '$lib/constants';

	let selectedTree = $state(null);

	const onChange = (tree) => {
		selectedTree = tree;
	};

	const onClosePreview = () => {
		selectedTree = null;
	};
</script>

<svelte:head>
	<title>Map</title>
</svelte:head>

<Header />

<Map center={DEFAULT_MAP_CENTER} {onChange} />

<MapPreview tree={selectedTree} onClose={onClosePreview} />
