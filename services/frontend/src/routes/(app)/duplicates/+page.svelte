<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { routes } from '$lib/routes';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<Dialog title="Duplicate Trees">
	<div>
		{#if pageState.loading}
			<p>Checking...</p>
		{:else if pageState.error}
			<p>{pageState.error.description}</p>
		{:else if pageState.data && pageState.data.duplicates.length > 0}
			<p>
				This page lists trees that have been identified as duplicates, located within a meter of
				each other.
			</p>
			<p>
				Please enrich the target tree with data from the duplicate, then remove it (mark as gone).
			</p>

			<table>
				<thead>
					<tr>
						<th>From</th>
						<th>To</th>
					</tr>
				</thead>
				<tbody>
					{#each pageState.data.duplicates as dup}
						<tr>
							<td><a href={routes.mapPreview(dup.from_id)}>{dup.from_id}</a></td>
							<td><a href={routes.mapPreview(dup.to_id)}>{dup.to_id}</a></td>
						</tr>
					{/each}
				</tbody>
			</table>
		{:else}
			<p>Congratulations, there are no duplicate trees!</p>
		{/if}
	</div>
</Dialog>

<style>
	table {
		font-family: monospace;
	}
</style>
