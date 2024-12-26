<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';

	const { data } = $props();
	const { trees } = data;

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
	{#each trees as tree}
		<NewTreesListItem {tree} extra={format(tree.height)} />
	{/each}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
