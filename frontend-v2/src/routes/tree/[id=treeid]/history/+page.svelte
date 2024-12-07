<script>
	import Header from '$lib/components/tree/Header.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<svelte:head>
	<title>{tree.species} - Details</title>
</svelte:head>

<Header />
<Title title={tree.species} />
<Tabs tree={tree.id} active="history" />

<p>No history of changes for this tree yet.</p>
