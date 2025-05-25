<script lang="ts">
	import { formatSpecies } from '$lib/utils/trees';
	import { loadTreeHistory } from '$lib/hooks';
	import Title from '$lib/components/tree/Title.svelte';
	import { ChangeList, NarrowPage, TreeTabs, TreeContextMenu } from '$lib/ui';

	const { data } = $props();
	const { loading, tree, changes, error, reload } = loadTreeHistory();

	$effect(() => {
		reload(data.id);
	});
</script>

<NarrowPage title="Tree History" nopadding>
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>Error loading tree: {$error.description}</p>
	{:else}
		<Title title={formatSpecies($tree.species)} address={$tree.address} />
		<TreeTabs tree={$tree.id} active="history" comment_count={$tree?.comment_count ?? 0} />
		<TreeContextMenu id={$tree.id} />

		<div class="padded">
			<ChangeList changes={$changes} />
		</div>
	{/if}
</NarrowPage>

<style>
	.padded,
	p {
		padding: 0 var(--gap);
	}
</style>
