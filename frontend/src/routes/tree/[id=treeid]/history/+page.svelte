<script lang="ts">
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import TreeTabs from '$lib/components/tree/TreeTabs.svelte';
	import { loadTreeHistory } from '$lib/hooks';
	import { routes } from '$lib/routes';
	import { ChangeList, NarrowPage } from '$lib/ui';

	const { data } = $props();
	const { loading, tree, changes, error, reload } = loadTreeHistory();

	$effect(() => {
		reload(data.id);
	});
</script>

<NarrowPage title="Tree History" back={routes.mapPreview(data.id)} nopadding>
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>Error loading tree: {$error.description}</p>
	{:else if $tree}
		<Title title={$tree.species} address={$tree.address} padded />
		<TreeTabs tree={$tree.id} active="history" />
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
