<script lang="ts">
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadTreesByDiameter } from '$lib/hooks';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { loading, error, data, reload } = loadTreesByDiameter();

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

<Dialog title="Widest Trees">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		{#each $data as tree}
			<NewTreesListItem {tree} extra={format(tree.diameter)} />
		{/each}
	{/if}
</Dialog>
