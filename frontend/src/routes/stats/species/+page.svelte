<script lang="ts">
	import { routes } from '$lib/routes';

	import Header from '$lib/components/tree/Header.svelte';
	import HomeTabs from '$lib/components/HomeTabs.svelte';

	const { data } = $props();
</script>

<svelte:head>
	<title>Trees by species</title>
</svelte:head>

<Header title="Statistics" />

<HomeTabs active="stats" />

<div class="padded">
	<h1>Trees by species</h1>

	<ul>
		{#each data.stats as { species, count }}
			<li><a href={routes.searchQuery(species)}>{species}</a> ({count})</li>
		{/each}
	</ul>
</div>

<style>
	ul {
		line-height: 1.5em;
	}
</style>
