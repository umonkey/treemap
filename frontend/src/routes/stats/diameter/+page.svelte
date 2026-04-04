<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});

	const format = (value: number | null): string => {
		if (!value) {
			return 'no data';
		}

		return `${value.toFixed(1)} m`;
	};
</script>

<Dialog title="Widest Trees">
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>{pageState.error.description}</p>
	{:else}
		{#each pageState.data as tree}
			<NewTreesListItem {tree} extra={format(tree.diameter)} />
		{/each}
	{/if}
</Dialog>
