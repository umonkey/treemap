<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadTreesByDiameter } from '$lib/hooks';

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

<div class="trees padded">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		{#each $data as tree}
			<NewTreesListItem {tree} extra={format(tree.diameter)} />
		{/each}
	{/if}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
