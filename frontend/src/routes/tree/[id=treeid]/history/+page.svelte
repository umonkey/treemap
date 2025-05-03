<script lang="ts">
	import { formatSpecies } from '$lib/utils/trees';
	import { loadTreeHistory } from '$lib/hooks';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import { ChangeList, NarrowPage, TreeContextMenu } from '$lib/ui';

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
		<Tabs tree={$tree.id} active="history" />
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
