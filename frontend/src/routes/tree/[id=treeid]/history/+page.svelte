<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeTabs from '$lib/components/tree/TreeTabs.svelte';
	import ChangeList from './ChangeList.svelte';
	import { pageState } from './page.svelte';

	const { data } = $props();

	$effect(() => {
		pageState.reload(data.id);
	});
</script>

<Dialog title="Tree History" nopadding>
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>Error loading tree: {pageState.error.description}</p>
	{:else if pageState.tree}
		<Title
			id={pageState.tree.id}
			title={pageState.tree.species}
			address={pageState.tree.address}
			padded
		/>
		<TreeTabs tree={pageState.tree.id} active="history" />

		<div class="padded">
			<ChangeList changes={pageState.changes} />
		</div>
	{/if}
</Dialog>

<style>
	.padded,
	p {
		padding: 0 var(--gap);
	}
</style>
