<script lang="ts">
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadTreesByDiameter } from '$lib/hooks';
	import { Header, NarrowPage } from '$lib/ui';

	const { loading, error, data, reload } = loadTreesByDiameter();

	$effect(() => {
		reload();
	});

	const format = (value: number | null): string => {
		if (!value) {
			return 'no data';
		}

		return `${value.toFixed(1)} m`;
	};
</script>

<svelte:head>
	<title>Widest trees</title>
</svelte:head>

<Header title="Widest trees" />

<NarrowPage>
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		{#each $data as tree}
			<NewTreesListItem {tree} extra={format(tree.diameter)} />
		{/each}
	{/if}
</NarrowPage>
