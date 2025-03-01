<script lang="ts">
	import { routes } from '$lib/routes';
	import { loadSpeciesStats } from '$lib/hooks';
	import Header from '$lib/components/tree/Header.svelte';

	const { loading, error, data, reload, reorder } = loadSpeciesStats();

	$effect(() => {
		reload();
	});
</script>

<svelte:head>
	<title>Trees by species</title>
</svelte:head>

<Header title="Statistics" />

<div class="padded">
	<h1>Trees by species</h1>

	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>Error: {$error.description}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l" onclick={() => reorder('name')}>Species</th>
					<th class="r" onclick={() => reorder('count')}>Count</th>
				</tr>
			</thead>
			<tbody>
				{#each $data as { species, count }}
					<tr>
						<td class="l"><a href={routes.searchSpecies(species)}>{species}</a></td>
						<td class="r">{count}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>

<style>
	table {
		line-height: 1.5;
	}

	th.l,
	td.l {
		text-align: left;
	}

	th.r,
	td.r {
		text-align: right;
	}
</style>
