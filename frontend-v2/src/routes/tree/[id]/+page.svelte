<script>
	import Actions from './components/Actions.svelte';
	import Description from './components/Description.svelte';
	import Gallery from './components/Gallery.svelte';
	import Header from './components/Header.svelte';
	import Links from './components/Links.svelte';
	import Properties from './components/Properties.svelte';
	import Tabs from './components/Tabs.svelte';
	import Title from './components/Title.svelte';
</script>

<svelte:head>
	<title>Quercus rubra</title>
</svelte:head>

<Header />
<Title />
<Tabs tree="134372280418570240" />
<Gallery />
<Actions />
<Properties />
<Links />
<Description />
