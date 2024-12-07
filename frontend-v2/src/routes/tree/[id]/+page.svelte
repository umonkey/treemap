<script>
	import Actions from './components/Actions.svelte';
	import Description from './components/Description.svelte';
	import Gallery from './components/Gallery.svelte';
	import Header from './components/Header.svelte';
	import Links from './components/Links.svelte';
	import Properties from './components/Properties.svelte';
	import Tabs from './components/Tabs.svelte';
	import Title from './components/Title.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<svelte:head>
	<title>{tree.species}</title>
</svelte:head>

<Header />
<Title title={tree.species} />
<Tabs tree={tree.id} active="details" />
<Gallery />
<Actions />
<Properties {tree} />
<Links />
<Description text={tree.notes} />
