<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadTreesByCircumference } from '$lib/hooks/loadTreesByCircumference';

	const { loading, error, data, reload } = loadTreesByCircumference();

	$effect(() => {
		reload();
	});

	const format = (value: number | null): string => {
		if (!value) {
			return 'no data';
		}

		return `${(value * 100).toFixed(0)} cm`;
	};
</script>

<Dialog title="Thickest Trees">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		{#each $data as tree}
			<NewTreesListItem {tree} extra={format(tree.circumference)} />
		{/each}
	{/if}
</Dialog>
