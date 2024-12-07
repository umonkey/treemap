<script>
	import Header from '$lib/components/tree/Header.svelte';
	import Map from '$lib/components/Map.svelte';
	import { DEFAULT_MAP_CENTER } from '$lib/constants';
</script>

<svelte:head>
	<title>Map</title>
</svelte:head>

<Header />

<Map center={DEFAULT_MAP_CENTER} />
