<script lang="ts">
	import { locale } from '$lib/locale';
	import { formatSpecies } from '$lib/utils/trees';
	import { setLastTree } from '$lib/stores/mapStore';

	import Actions from '$lib/components/tree/Actions.svelte';
	import Description from '$lib/components/tree/Description.svelte';
	import Links from '$lib/components/tree/Links.svelte';
	import Properties from '$lib/components/tree/Properties.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import { Gallery, Header, TreeContextMenu } from '$lib/ui';
	import { onMount } from 'svelte';

	const { data } = $props();
	const tree = data.tree;

	// Save last active tree.
	onMount(() => {
		setLastTree(tree.id);
		console.debug('[map] Last active tree set to', tree.id);
	});
</script>

<svelte:head>
	<title>{locale.detailsTitle(formatSpecies(tree.species))}</title>
</svelte:head>

<Header title={locale.treeShortTitle()} />
<Title title={formatSpecies(tree.species)} address={tree.address} />
<Tabs tree={tree.id} active="details" />
<Gallery id={tree.id} />
<Actions {tree} />
<Properties {tree} />
<Links {tree} />
<Description text={tree.notes} />
<TreeContextMenu id={tree.id} />
