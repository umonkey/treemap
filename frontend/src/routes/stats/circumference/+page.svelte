<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';

	const { data } = $props();
	const { trees } = data;

	const format = (value: number | null): string => {
		if (!value) {
			return 'no data';
		}

		return `${(value * 100).toFixed(0)} cm`;
	};
</script>

<svelte:head>
	<title>Thickest trees</title>
</svelte:head>

<Header title="Thickest trees" />

<div class="trees padded">
	{#each trees as tree}
		<NewTreesListItem {tree} extra={format(tree.circumference)} />
	{/each}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
