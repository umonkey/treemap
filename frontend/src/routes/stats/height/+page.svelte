<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadTreesByHeight } from '$lib/hooks/loadTreesByHeight';

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

<Dialog title="Highest Trees">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		{#each $data as tree}
			<NewTreesListItem {tree} extra={format(tree.height)} />
		{/each}
	{/if}
</Dialog>
