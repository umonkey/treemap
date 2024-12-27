<script lang="ts">
	import { routes } from '$lib/routes';

	import Header from '$lib/components/tree/Header.svelte';

	const { data } = $props();
	let sorted = $state(data.stats);

	const sortByName = () => {
		sorted = data.stats.sort((a, b) => a.address.localeCompare(b.address));
	};

	const sortByCount = () => {
		sorted = data.stats.sort((a, b) => b.count - a.count);
	};
</script>

<svelte:head>
	<title>Trees by species</title>
</svelte:head>

<Header title="Statistics" />

<div class="padded">
	<h1>Trees by address</h1>

	<table>
		<thead>
			<tr>
				<th class="l" onclick={sortByName}>Address</th>
				<th class="r" onclick={sortByCount}>Count</th>
			</tr>
		</thead>
		<tbody>
			{#each sorted as { address, count }}
				<tr>
					<td class="l"><a href={routes.searchAddress(address)}>{address}</a></td>
					<td class="r">{count}</td>
				</tr>
			{/each}
		</tbody>
	</table>
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
