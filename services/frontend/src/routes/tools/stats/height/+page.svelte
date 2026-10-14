<script lang="ts">
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import NewTreesListItem from '$lib/components/tree/NewTreesListItem.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

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

<svelte:head>
	<title>Highest Trees</title>
</svelte:head>

<article>
	<PageHeader text="Highest Trees" />
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>{pageState.error.description}</p>
	{:else}
		{#each pageState.data as tree}
			<NewTreesListItem {tree} extra={format(tree.height)} />
		{/each}
	{/if}
</article>
