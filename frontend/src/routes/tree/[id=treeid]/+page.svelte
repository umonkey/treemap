<script lang="ts">
	import { formatSpecies } from '$lib/utils/trees';
	import { locale } from '$lib/locale';

	import Actions from '$lib/components/tree/Actions.svelte';
	import Description from '$lib/components/tree/Description.svelte';
	import Gallery from '$lib/components/tree/Gallery.svelte';
	import Header from '$lib/components/tree/Header.svelte';
	import Links from '$lib/components/tree/Links.svelte';
	import Properties from '$lib/components/tree/Properties.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeMenu from '$lib/components/tree/Menu.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<svelte:head>
	<title>{locale.detailsTitle(formatSpecies(tree.species))}</title>
</svelte:head>

<Header title={locale.treeShortTitle()} />
<Title title={formatSpecies(tree.species)} address={tree.address} />
<Tabs tree={tree.id} active="details" />
<Gallery files={tree.files} />
<Actions {tree} />
<Properties {tree} />
<Links {tree} />
<Description text={tree.notes} />
<TreeMenu id={tree.id} />
