<script lang="ts">
	import { loadStateStats } from '$lib/hooks';
	import { routes } from '$lib/routes';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { loading, error, data, reload } = loadStateStats();

	$effect(() => {
		reload();
	});
</script>

<Dialog title="Trees by Status">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>{$error.description}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l">State</th>
					<th class="r">Count</th>
				</tr>
			</thead>
			<tbody>
				{#each $data as { state, count }}
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
