<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import NewTreesListItem from '../NewTreesListItem.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<Dialog title="Species Mismatches">
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>{pageState.error.description}</p>
	{:else if pageState.data}
		{#if pageState.data.length === 0}
			<p>There are no wongly identified species, all good.</p>
		{:else}
			{#each pageState.data as tree}
				<NewTreesListItem {tree} />
			{/each}
		{/if}
	{/if}
</Dialog>
