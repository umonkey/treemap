<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import NewTreesListItem from '../NewTreesListItem.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});

	const format = (value: number | null): string => {
		if (!value) {
			return 'no data';
		}

		return `${(value * 100).toFixed(0)} cm`;
	};
</script>

<Dialog title="Thickest Trees">
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>{pageState.error.description}</p>
	{:else}
		{#each pageState.data as tree}
			<NewTreesListItem {tree} extra={format(tree.circumference)} />
		{/each}
	{/if}
</Dialog>
