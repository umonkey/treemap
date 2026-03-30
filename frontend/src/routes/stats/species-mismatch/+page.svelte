<script lang="ts">
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadSpeciesMismatch } from '$lib/hooks';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { loading, error, data, reload } = loadSpeciesMismatch();

	$effect(() => {
		reload();
	});
</script>

<Dialog title="Species Mismatches">
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
</Dialog>
