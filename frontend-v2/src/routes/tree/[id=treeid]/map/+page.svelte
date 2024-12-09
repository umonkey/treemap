<script>
	import Header from '$lib/components/tree/Header.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import Map from '$lib/components/Map.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<svelte:head>
	<title>{tree.species} - Details</title>
</svelte:head>

<Header />
<Title title={tree.species} />
<Tabs tree={tree.id} active="map" />

<Map center={[tree.lat, tree.lon]} className="treeTab" />
