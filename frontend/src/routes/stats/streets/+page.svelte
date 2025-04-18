<script lang="ts">
	import { loadStreetStats } from '$lib/hooks';
	import { routes } from '$lib/routes';
	import { Header } from '$lib/ui';

	const { loading, error, data, reload, reorder } = loadStreetStats();

	$effect(() => {
		reload();
	});
</script>

<svelte:head>
	<title>Trees by species</title>
</svelte:head>

<Header title="Statistics" />

<div class="padded">
	<h1>Trees by address</h1>

	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l" onclick={() => reorder('address')}>Address</th>
					<th class="r" onclick={() => reorder('count')}>Count</th>
				</tr>
			</thead>
			<tbody>
				{#each $data as { address, count }}
					<tr>
						<td class="l"><a href={routes.searchAddress(address)}>{address}</a></td>
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
