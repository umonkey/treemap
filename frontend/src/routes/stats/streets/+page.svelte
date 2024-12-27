<script lang="ts">
	import { routes } from '$lib/routes';

	import Header from '$lib/components/tree/Header.svelte';

	const { data } = $props();
</script>

<svelte:head>
	<title>Trees by species</title>
</svelte:head>

<Header title="Statistics" />

<div class="padded">
	<h1>Trees by address</h1>

	<ul>
		{#each data.stats as { address, count }}
			<li><a href={routes.searchAddress(address)}>{address}</a> ({count})</li>
		{/each}
	</ul>
</div>

<style>
	ul {
		line-height: 1.5em;
	}
</style>
