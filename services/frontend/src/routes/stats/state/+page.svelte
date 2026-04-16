<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { routes } from '$lib/routes';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<Dialog title="Trees by Status">
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>{pageState.error.description}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l">State</th>
					<th class="r">Count</th>
				</tr>
			</thead>
			<tbody>
				{#each pageState.data as { state, count }}
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
