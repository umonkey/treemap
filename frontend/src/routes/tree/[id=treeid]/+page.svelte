<script lang="ts">
	import { locale } from '$lib/locale';
	import { formatSpecies } from '$lib/utils/trees';
	import { setLastTree } from '$lib/stores/mapStore';

	import Actions from '$lib/components/tree/Actions.svelte';
	import Description from '$lib/components/tree/Description.svelte';
	import Links from '$lib/components/tree/Links.svelte';
	import Properties from '$lib/components/tree/Properties.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import { Gallery, TreeContextMenu, TreeTabs, NarrowPage } from '$lib/ui';
	import { onMount } from 'svelte';

	const { data } = $props();
	const tree = data.tree;

	// Save last active tree.
	onMount(() => {
		setLastTree(tree.id);
		console.debug('[map] Last active tree set to', tree.id);
	});
</script>

<NarrowPage title={locale.treeShortTitle()} nopadding>
	<Title title={formatSpecies(tree.species)} address={tree.address} />
	<TreeTabs tree={tree.id} active="details" comment_count={tree.comment_count} />
	<Gallery id={tree.id} />
	<Actions {tree} />
	<Properties {tree} />
	<Links {tree} />
	<Description text={tree.notes} />
	<TreeContextMenu id={tree.id} />
</NarrowPage>
