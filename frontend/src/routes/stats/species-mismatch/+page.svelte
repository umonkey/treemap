<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';
	import HomeTabs from '$lib/components/HomeTabs.svelte';

	const { data } = $props();
	const { trees } = data;
</script>

<svelte:head>
	<title>Mismatched species</title>
</svelte:head>

<Header title="Species mismatches" />
<HomeTabs active="stats" />

<div class="trees padded">
	{#each trees as tree}
		<NewTreesListItem {tree} />
	{:else}
		<p>There are no wongly identified species, all good.</p>
	{/each}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
