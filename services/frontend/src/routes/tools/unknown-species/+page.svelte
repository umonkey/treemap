<script lang="ts">
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import NewTreesListItem from '$lib/components/tree/NewTreesListItem.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	$effect(() => {
		pageState.reload();
	});
</script>

<article>
	<PageHeader text="Guess Unknown Species" />

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
</article>
