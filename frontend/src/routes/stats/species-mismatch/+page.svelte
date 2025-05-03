<script lang="ts">
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import { loadSpeciesMismatch } from '$lib/hooks';
	import { Header, NarrowPage } from '$lib/ui';

	const { loading, error, data, reload } = loadSpeciesMismatch();

	$effect(() => {
		reload();
	});
</script>

<svelte:head>
	<title>Mismatched species</title>
</svelte:head>

<Header title="Species mismatches" />

<NarrowPage>
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
</NarrowPage>
