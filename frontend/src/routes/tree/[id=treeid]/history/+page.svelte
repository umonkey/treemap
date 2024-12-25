<script>
	import { formatSpecies } from '$lib/utils/trees';

	import Header from '$lib/components/tree/Header.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeMenu from '$lib/components/tree/Menu.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<svelte:head>
	<title>{formatSpecies(tree.species)} - Details</title>
</svelte:head>

<Header title="Tree" />
<Title title={formatSpecies(tree.species)} address={tree.address} />
<Tabs tree={tree.id} active="history" />
<TreeMenu id={tree.id} />

<p>No history of changes for this tree yet.</p>
<p>(This area is under construction.)</p>

<style>
	p {
		padding: 0 var(--gap);
	}
</style>
