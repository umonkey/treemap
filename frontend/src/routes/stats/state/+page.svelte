<script lang="ts">
	import Header from '$lib/components/tree/Header.svelte';
	import { routes } from '$lib/routes';

	const { data } = $props();
	const { stats, error } = data;
</script>

<svelte:head>
	<title>Trees by state</title>
</svelte:head>

<Header title="Trees by state" />

{#if error}
	<p>{error}</p>
{:else}
	<div class="padded">
		<h1>Trees by state</h1>

		<table>
			<thead>
				<tr>
					<th class="l">State</th>
					<th class="r">Count</th>
				</tr>
			</thead>
			<tbody>
				{#each stats as { state, count }}
					<tr>
						{#if state}
							<td class="l"
								><a href={routes.searchState(state)}>{state ? state : '(not set)'}</a></td
							>
						{:else}
							<td class="l">not set</td>
						{/if}
						<td class="r">{count}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}

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
