<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import NewTreesListItem from '$lib/components/updates/NewTreesListItem.svelte';

	const { data } = $props();
	const { trees } = data;
</script>

<Header title="New Trees" />

<div class="trees padded">
	{#each trees as tree}
		<NewTreesListItem {tree} />
	{/each}
</div>

<style>
	.trees {
		margin: var(--gap) 0;
	}
</style>
