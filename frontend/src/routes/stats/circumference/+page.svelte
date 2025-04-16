<script lang="ts">
import Header from "$lib/components/tree/Header.svelte";
import NewTreesListItem from "$lib/components/updates/NewTreesListItem.svelte";
import { loadTreesByCircumference } from "$lib/hooks";

const { loading, error, data, reload } = loadTreesByCircumference();

$effect(() => {
	reload();
});

const format = (value: number | null): string => {
	if (!value) {
		return "no data";
	}

	return `${(value * 100).toFixed(0)} cm`;
};
</script>

<svelte:head>
	<title>Thickest trees</title>
</svelte:head>

<Header title="Thickest trees" />

<div class="trees padded">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		{#each $data as tree}
			<NewTreesListItem {tree} extra={format(tree.circumference)} />
		{/each}
	{/if}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
