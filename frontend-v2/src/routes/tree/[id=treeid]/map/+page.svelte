<script>
	import { formatSpecies } from '$lib/utils/trees';

	import Header from '$lib/components/tree/Header.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import Map from '$lib/components/Map.svelte';
	import TreeMenu from '$lib/components/tree/Menu.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<svelte:head>
	<title>{formatSpecies(tree.species)} - Details</title>
</svelte:head>

<Header title="Tree" />
<Title title={formatSpecies(tree.species)} address={tree.address} />
<Tabs tree={tree.id} active="map" />
<TreeMenu id={tree.id} />

<div class="mapContainer">
	<Map
		center={[tree.lat, tree.lon]}
		marker={[tree.lat, tree.lon]}
		zoom={18}
		className="treeTab"
		canAdd={true}
	/>
</div>

<style>
	.mapContainer {
		position: relative;
	}

	@media (max-width: 480px) {
		.mapContainer {
			height: calc(100dvh - 185px);
		}
	}

	@media (min-width: 481px) {
		.mapContainer {
			height: calc(100dvh - 41px);
		}
	}
</style>
