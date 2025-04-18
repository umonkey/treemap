<script lang="ts">
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadTreesByHeight } from '$lib/hooks';
	import { Header } from '$lib/ui';

	const { loading, error, data, reload } = loadTreesByHeight();

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
	<title>Highest trees</title>
</svelte:head>

<Header title="Highest trees" />

<div class="trees padded">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		{#each $data as tree}
			<NewTreesListItem {tree} extra={format(tree.height)} />
		{/each}
	{/if}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
