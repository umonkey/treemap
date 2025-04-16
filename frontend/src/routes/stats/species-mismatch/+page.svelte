<script lang="ts">
import Header from "$lib/components/tree/Header.svelte";
import NewTreesListItem from "$lib/components/updates/NewTreesListItem.svelte";
import { loadSpeciesMismatch } from "$lib/hooks";

const { loading, error, data, reload } = loadSpeciesMismatch();

$effect(() => {
	reload();
});
</script>

<svelte:head>
	<title>Mismatched species</title>
</svelte:head>

<Header title="Species mismatches" />

<div class="trees padded">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else if $data}
		{#if $data.length === 0}
			<p>There are no wongly identified species, all good.</p>
		{:else}
			{#each $data as tree}
				<NewTreesListItem {tree} />
			{/each}
		{/if}
	{/if}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
