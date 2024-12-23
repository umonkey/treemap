<script lang="ts">
	import { routes } from '$lib/routes';

	import Header from '$lib/components/tree/Header.svelte';
	import HomeTabs from '$lib/components/HomeTabs.svelte';
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>

<Header title="Tree Map" />
<HomeTabs active="stats" />

<div class="padded">
	<h1>Data reports</h1>

	<ul>
		<li><a href={routes.statsSpecies()}>Top species</a></li>
	</ul>
</div>
