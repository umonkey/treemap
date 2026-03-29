<script lang="ts">
	import { loadStreetStats } from '$lib/hooks';
	import { routes } from '$lib/routes';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { loading, error, data, reload, reorder } = loadStreetStats();

	$effect(() => {
		reload();
	});
</script>

<Dialog title="Trees by Address">
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
</Dialog>

<style>
	table {
		line-height: 1.5;
		width: 100%;
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
